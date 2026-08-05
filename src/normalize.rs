use crate::{
    counter::Counter,
    meta_info::{Task, TaskType},
    ns_trie::NamespaceTrie,
    parse::{parse, ParserWrapper},
};
use log::{error, info, trace};
use oxrdf::{Literal, NamedNode, NamedOrBlankNode, Term, Triple};
use oxttl::{TurtleParseError, TurtleSerializer};
use rayon::ThreadPoolBuilder;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{metadata, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    time::Instant,
};

type TripleFreq = BTreeMap<String, TripleFreqSec>;
type TripleFreqSec = BTreeMap<String, TripleFreqThird>;
type TripleFreqThird = BTreeMap<String, ObjIsDatatype>;
type ObjIsDatatype = BTreeMap<bool, i32>;

trait TripleFreqFns {
    fn add(&mut self, triple: (String, String, String, bool));
    fn iter_all(&self) -> Vec<(String, String, String, bool, i32)>;
}

impl TripleFreqFns for TripleFreq {
    fn add(&mut self, triple: (String, String, String, bool)) {
        let count = self
            .entry(triple.0)
            .or_default()
            .entry(triple.1)
            .or_default()
            .entry(triple.2)
            .or_default()
            .entry(triple.3)
            .or_default();
        *count += 1;
    }

    fn iter_all(&self) -> Vec<(String, String, String, bool, i32)> {
        self.iter()
            .flat_map(|(s, m)| {
                m.iter().flat_map(|(p, m)| {
                    m.iter().flat_map(|(o, m)| {
                        m.iter()
                            .map(|(d, count)| (s.clone(), p.clone(), o.clone(), *d, *count))
                    })
                })
            })
            .collect()
    }
}

#[derive(Debug)]
pub enum Message {
    Started {
        path: String,
    },
    NormalizedTriple {
        subject: NormalizedResource,
        predicate: NormalizedResource,
        object: NormalizedResource,
    },
    NamespacesUnknown {
        iris: Vec<String>,
    },
    Finished {
        path: String,
        triples: usize,
        blanks: usize,
        iris: usize,
        literals: usize,
    },
    FatalError {
        err: TurtleParseError,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum NormalizedResource {
    Unknown,
    BlankNode,
    Literal(Lit),
    TypedLiteral(TypedLit),
    NamedNode(NNode),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lit {
    lang: Option<String>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct TypedLit {
    namespace: String,
    alias: String,
    iri: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NNode {
    alias: String,
    namespace: String,
}

impl From<NormalizedResource> for String {
    fn from(nr: NormalizedResource) -> Self {
        match nr {
            NormalizedResource::Unknown => "UNKNOWN".to_string(),
            NormalizedResource::BlankNode => "BLANK".to_string(),
            NormalizedResource::Literal(Lit { lang }) => match lang {
                //None => "STRING".into(),
                //Some(l) => format!("STRING@{l}"),
                None => "xsd".into(),
                Some(_) => "rdf".into(),
            },
            NormalizedResource::TypedLiteral(TypedLit {
                namespace: _,
                alias,
                iri: _,
            }) => {
                alias
                //format!("{}:{}", alias, &iri[namespace.len()..])
            }
            NormalizedResource::NamedNode(NNode {
                alias,
                namespace: _,
            }) => alias,
        }
    }
}

#[derive(Ord, Eq, PartialEq, PartialOrd)]
pub struct GroupNS {
    alias: String,
    namespace: String,
}

#[derive(Default)]
pub struct Groups {
    pub namespaces: BTreeSet<GroupNS>,
    blank: bool,
    unknown: bool,
}

pub fn normalize_triples(
    paths: Vec<PathBuf>,
    n_workers: usize,
    ns_trie: &NamespaceTrie,
    ignore_unknown: bool,
    outf: &str,
    total_triples: usize,
) -> (TripleFreq, Groups, BTreeMap<String, Task>) {
    let mut triples = TripleFreq::new();
    let mut used_groups: Groups = Default::default();

    if n_workers < 2 {
        panic!("Number of workers must be at least 2");
    }
    info!("Creating pool with {n_workers} threads");

    let mut running = paths.len();
    let pool = ThreadPoolBuilder::new()
        .num_threads(n_workers)
        .build()
        .unwrap();

    let mut tasks = BTreeMap::<String, Task>::new();

    pool.scope_fifo(|s| {
        let (tx, rx) = sync_channel::<Message>(100);

        for path in paths {
            let tx = tx.clone();

            s.spawn_fifo(move |_| {
                tx.send(Message::Started {
                    path: path.to_string_lossy().to_string(),
                })
                .unwrap();

                info!("Parsing {:?}", path);
                let mut graph = parse(&path);
                proc_triples(&mut graph, &path, &tx, ns_trie, ignore_unknown);
            });
        }

        let errors_path = Path::new(".").join(outf).join("errors.log");
        let mut fd = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(errors_path.clone())
            .unwrap();

        handle_loop(
            &mut running,
            rx,
            &mut triples,
            &mut used_groups,
            &mut tasks,
            &mut fd,
            ignore_unknown,
            total_triples,
        );
    });

    (triples, used_groups, tasks)
}

#[allow(clippy::too_many_arguments)]
fn handle_loop(
    running: &mut usize,
    rx: Receiver<Message>,
    triples: &mut TripleFreq,
    used_groups: &mut Groups,
    tasks: &mut BTreeMap<String, Task>,
    _fd: &mut File,
    ignore_unknown: bool,
    total_triples: usize,
) {
    let msg_c = &mut Counter::default();
    let trip_c = &mut Counter::default();
    let start = &mut Instant::now();

    loop {
        msg_c.inc();
        if msg_c.cur % 1_000_000 == 1 {
            restart_timers(start, msg_c, trip_c, ignore_unknown, total_triples);
        }
        if *running == 0 {
            info!("All threads finished");
            break;
        }
        if let Ok(message) = rx.recv() {
            match message {
                Message::Started { path } => {
                    let mut t = Task::new(path.clone(), TaskType::Normalize);
                    t.size = metadata(path.clone()).unwrap().len() as usize;
                    tasks.insert(path, t);
                }
                Message::NormalizedTriple {
                    subject,
                    predicate,
                    object,
                } => {
                    trip_c.inc();
                    proc_message(subject, predicate, object, triples, used_groups);
                }
                Message::NamespacesUnknown { iris } => {
                    for _iri in iris.iter() {
                        //let msg = format!("Unknown namespace for resource '{iri}'");
                        //writeln!(fd, "Unknown namespace for resource '{iri}'").unwrap();
                    }
                }
                Message::Finished {
                    path,
                    triples,
                    iris,
                    blanks,
                    literals,
                } => {
                    let t = tasks.get_mut(&path).unwrap();
                    t.triples = triples;
                    t.iris = iris;
                    t.blanks = blanks;
                    t.literals = literals;
                    t.finish(format!("Finished task {:?} on {}", t.task_type, t.name).as_str());

                    *running -= 1;
                }
                Message::FatalError { err } => {
                    error!("Fatal error: {err}");
                    *running -= 1;
                }
            }
        }
    }
}

fn restart_timers(
    start: &mut Instant,
    msg_c: &mut Counter,
    trip_c: &mut Counter,
    ignore_unknown: bool,
    total_triples: usize,
) {
    let elapsed = start.elapsed().as_millis();
    let msg_rate = (msg_c.delta() as u128)
        .checked_div(elapsed)
        .map(|r| r * 1000);
    let trip_rate = (trip_c.delta() as u128)
        .checked_div(elapsed)
        .map(|r| r * 1000);
    if msg_rate.is_some() || trip_rate.is_some() {
        trace!(
            "Received {} messages ({}/s), {} triples ({}/s) so far{})",
            msg_c.cur,
            msg_rate.unwrap_or(0),
            trip_c.cur,
            trip_rate.unwrap_or(0),
            if !ignore_unknown && total_triples > 0 {
                format!(" ({}%)", trip_c.cur * 100 / total_triples)
            } else {
                "".to_string()
            }
        );
    }

    msg_c.lap();
    trip_c.lap();
    *start = Instant::now();
}

fn proc_message(
    subject: NormalizedResource,
    predicate: NormalizedResource,
    object: NormalizedResource,
    triples: &mut TripleFreq,
    used_groups: &mut Groups,
) {
    let mut is_datatype = false;

    for resource in [subject.clone(), predicate.clone(), object.clone()] {
        match resource {
            NormalizedResource::Unknown => {
                used_groups.unknown = true;
            }
            NormalizedResource::BlankNode => {
                used_groups.blank = true;
            }
            NormalizedResource::Literal(Lit { lang }) => {
                is_datatype = true;
                used_groups.namespaces.insert(match lang {
                    None => GroupNS {
                        alias: "xsd".into(),
                        namespace: "http://www.w3.org/TR/xmlschema11-2/".into(),
                    },
                    Some(_) => GroupNS {
                        alias: "rdf".into(),
                        namespace: "http://www.w3.org/1999/02/22-rdf-syntax-ns#".into(),
                    },
                });
            }
            NormalizedResource::TypedLiteral(TypedLit {
                namespace,
                alias,
                iri: _,
            }) => {
                is_datatype = true;
                used_groups.namespaces.insert(GroupNS { alias, namespace });
            }
            NormalizedResource::NamedNode(NNode { alias, namespace }) => {
                used_groups.namespaces.insert(GroupNS { alias, namespace });
            }
        }
    }

    triples.add((subject.into(), predicate.into(), object.into(), is_datatype));
}

fn proc_triples(
    graph: &mut ParserWrapper,
    path: &Path,
    tx: &SyncSender<Message>,
    ns_trie: &NamespaceTrie,
    ignore_unknown: bool,
) {
    let tid = if let Some(id) = rayon::current_thread_index() {
        id.to_string()
    } else {
        "".to_string()
    };
    let mut i = 0;
    let mut last_i = 0;
    let mut start = Instant::now();

    let iri_c = 0;
    let blank_c = 0;
    let literal_c = 0;

    while let Some(result) = graph.next() {
        i += 1;

        if i % 1_000_000 == 1 && !start.elapsed().is_zero() {
            let elapsed = start.elapsed().as_millis();
            let rate = ((i - last_i) as u128)
                .checked_div(elapsed)
                .map(|r| r * 1000);
            if rate.is_some() {
                trace!(
                    "[Thread#{tid}] Parsed {i} triples so far ({} triples/s)",
                    rate.unwrap_or(0)
                );
            }
            last_i = i;
            start = Instant::now();
        }

        let t = match result {
            Ok(t) => t,
            Err(err) => {
                let msg = format!("Error normalizing file {}: {}", path.to_string_lossy(), err);
                error!("{}", msg);
                tx.send(Message::FatalError { err }).unwrap();
                return;
            }
        };

        let (_iris, _blanks, _literals) = proc_triple(t, tx, ns_trie, ignore_unknown);
    }
    tx.send(Message::Finished {
        path: path.to_string_lossy().to_string(),
        triples: i as usize,
        iris: iri_c,
        blanks: blank_c,
        literals: literal_c,
    })
    .unwrap();
}

fn count_resources(subject: &NamedOrBlankNode, object: &Term) -> (usize, usize, usize) {
    let mut iris = 0;
    let mut blanks = 0;
    let mut literals = 0;

    match subject {
        NamedOrBlankNode::NamedNode(_) => iris += 1,
        NamedOrBlankNode::BlankNode(_) => blanks += 1,
    }

    // predicate is always a NamedNode
    iris += 1;

    match object {
        Term::NamedNode(_) => iris += 1,
        Term::BlankNode(_) => blanks += 1,
        Term::Literal(_) => literals += 1,
    }

    (iris, blanks, literals)
}

fn proc_triple(
    t: Triple,
    tx: &SyncSender<Message>,
    ns_trie: &NamespaceTrie,
    ignore_unknown: bool,
) -> (usize, usize, usize) {
    let (iris, blanks, literals) = count_resources(&t.subject, &t.object);
    let subject = handle_subject(t.subject, ns_trie);
    let predicate = handle_predicate(t.predicate, ns_trie);
    let object = handle_object(t.object, ns_trie);

    if ignore_unknown {
        for res in [&subject, &predicate, &object] {
            if let Err(e) = res {
                error!("Skipping triple, unknown namespace: {e}");
            }
        }
        if subject.is_err() || predicate.is_err() || object.is_err() {
            return (iris, blanks, literals);
        }
    }

    let mut unknown_ns = Vec::new();

    if let Err(e) = &subject {
        unknown_ns.push(e.iri.clone());
    }
    if let Err(e) = &predicate {
        unknown_ns.push(e.iri.clone());
    }
    if let Err(e) = &object {
        unknown_ns.push(e.iri.clone());
    }

    if !unknown_ns.is_empty() {
        tx.send(Message::NamespacesUnknown { iris: unknown_ns })
            .unwrap();
    }

    tx.send(Message::NormalizedTriple {
        subject: match subject {
            Ok(ns) => ns.clone(),
            Err(UnknownNamespaceError { iri: _ }) => NormalizedResource::Unknown,
        },
        predicate: match predicate {
            Ok(ns) => ns.clone(),
            Err(UnknownNamespaceError { iri: _ }) => NormalizedResource::Unknown,
        },
        object: match object {
            Ok(ns) => ns.clone(),
            Err(UnknownNamespaceError { iri: _ }) => NormalizedResource::Unknown,
        },
    })
    .unwrap();

    (iris, blanks, literals)
}

fn handle_subject(
    sub: NamedOrBlankNode,
    ns_trie: &NamespaceTrie,
) -> Result<NormalizedResource, UnknownNamespaceError> {
    match sub {
        NamedOrBlankNode::BlankNode(_) => Ok(NormalizedResource::BlankNode),
        NamedOrBlankNode::NamedNode(n) => handle_named_node(n, ns_trie),
    }
}

fn handle_predicate(
    pred: NamedNode,
    ns_trie: &NamespaceTrie,
) -> Result<NormalizedResource, UnknownNamespaceError> {
    handle_named_node(pred, ns_trie)
}

fn handle_object(
    obj: Term,
    ns_trie: &NamespaceTrie,
) -> Result<NormalizedResource, UnknownNamespaceError> {
    match obj {
        Term::BlankNode(_) => Ok(NormalizedResource::BlankNode),
        Term::NamedNode(n) => handle_named_node(n, ns_trie),
        Term::Literal(lit) => handle_literal(lit, ns_trie),
    }
}

#[derive(Debug, Clone)]
pub struct UnknownNamespaceError {
    iri: String,
}

impl std::fmt::Display for UnknownNamespaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown namespace for IRI {}", self.iri)
    }
}

fn handle_named_node(
    n: NamedNode,
    ns_trie: &NamespaceTrie,
) -> Result<NormalizedResource, UnknownNamespaceError> {
    let res = ns_trie.longest_prefix(n.as_str(), true);
    if let Some((node, ns)) = res {
        if let Some((alias, _source)) = &node.value {
            return Ok(NormalizedResource::NamedNode(NNode {
                alias: alias.clone(),
                namespace: ns,
            }));
            //return Ok(node.value.as_ref().unwrap().clone());
        }
    }
    Err(UnknownNamespaceError {
        iri: n.as_str().to_string(),
    })
}

fn handle_literal(
    lit: Literal,
    ns_trie: &NamespaceTrie,
) -> Result<NormalizedResource, UnknownNamespaceError> {
    if let Some(language) = lit.language() {
        Ok(NormalizedResource::Literal(Lit {
            lang: Some(language.to_string()),
        }))
    } else {
        let datatype = lit.datatype();
        let res = ns_trie.longest_prefix(datatype.as_str(), true);
        if let Some((node, ns)) = res {
            if let Some((alias, _)) = &node.value {
                return Ok(NormalizedResource::TypedLiteral(TypedLit {
                    namespace: ns,
                    alias: alias.clone(),
                    iri: datatype.as_str().to_string(),
                }));
            }
        }
        Err(UnknownNamespaceError {
            iri: datatype.as_str().to_string(),
        })
    }
}

pub fn save_normalized_triples(
    nts: &TripleFreq,
    used_groups: Groups,
    min_occurs: Option<i32>,
    outf: &str,
) {
    let file_path = Path::new(".").join(outf).join("output.ttl");
    info!("Saving graph summary to {}", file_path.to_string_lossy());

    let mut id_count = 0;

    let mut fd = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path.clone())
        .unwrap();

    let base = "http://andrefs.com/graph-summ/v1";
    writeln!(fd, "@base <{}> .", { base }).unwrap();
    writeln!(fd, "@prefix ngont: <{}/ontology> .", base).unwrap(); // ontology (data-types?, unknown, blank, classes and predicates, etc)
    writeln!(fd, "@prefix ngns: <{}/instance> .", base).unwrap(); // namespaces (kgs, data types?)
    writeln!(fd).unwrap();

    let rdf = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

    let mut serializer = TurtleSerializer::new()
        .with_prefix("ngont", format!("{base}/ontology"))
        .unwrap()
        .with_prefix("ngns", format!("{base}/instance"))
        .unwrap()
        .for_writer(&mut fd);

    format_groups(used_groups, &mut serializer);

    serializer.finish().unwrap();
    writeln!(fd).unwrap();

    let mut serializer = TurtleSerializer::new()
        .with_prefix("ngont", format!("{base}/ontology"))
        .unwrap()
        .with_prefix("ngns", format!("{base}/instance"))
        .unwrap()
        .for_writer(&mut fd);

    for (s, p, o, is_datatype, occurs) in nts.iter_all() {
        if min_occurs.is_some() && occurs < min_occurs.unwrap() {
            continue;
        }

        id_count += 1;
        let t_id = format!("#t{:0width$}", id_count, width = 4);

        let type_link = if is_datatype {
            "#DatatypeLink"
        } else {
            "#GroupsLink"
        };

        for (pred, obj) in [
            (&format!("{rdf}type"), type_link),
            (&format!("{rdf}type"), &format!("{rdf}Statement")),
            (&format!("{rdf}subject"), &format!("#{s}")),
            (&format!("{rdf}predicate"), &format!("#{p}")),
            (&format!("{rdf}object"), &format!("#{o}")),
        ] {
            let t = Triple::new(
                NamedNode::new_unchecked(&t_id),
                NamedNode::new_unchecked(pred),
                NamedNode::new_unchecked(obj),
            );
            serializer.serialize_triple(&t).unwrap();
        }

        let t = Triple::new(
            NamedNode::new_unchecked(&t_id),
            NamedNode::new_unchecked("#occurrences"),
            Literal::new_typed_literal(
                occurs.to_string(),
                NamedNode::new_unchecked("http://www.w3.org/2001/XMLSchema#integer"),
            ),
        );
        serializer.serialize_triple(&t).unwrap();
    }
    serializer.finish().unwrap();
}

pub fn format_groups(
    groups: Groups,
    serializer: &mut oxttl::turtle::WriterTurtleSerializer<&mut File>,
) {
    for group in groups.namespaces {
        format_group(group, serializer);
    }
}

pub fn format_group(
    group: GroupNS,
    serializer: &mut oxttl::turtle::WriterTurtleSerializer<&mut File>,
) {
    let t = Triple::new(
        NamedNode::new_unchecked(&format!("#{}", group.alias)),
        NamedNode::new_unchecked("#namespacePrefix"),
        NamedNode::new_unchecked(&group.namespace),
    );
    serializer.serialize_triple(&t).unwrap();
}

#[cfg(test)]
mod tests {

    use crate::ns_trie::NamespaceSource;

    use super::*;

    #[test]
    fn handle_literal_simple() {
        let lit = Literal::Simple { value: "my-lit" };
        let ns_trie = NamespaceTrie::new();

        let res = handle_literal(lit, &ns_trie);

        assert!(res.is_ok());

        match res.unwrap() {
            NormalizedResource::Literal(lit) => {
                assert_eq!(lit.lang, None);
            }
            _ => panic!("Result should be a NormalizedResource::Literal"),
        }
    }

    #[test]
    fn handle_literal_lts() {
        let lit = Literal::LanguageTaggedString {
            value: "my-lit",
            language: "pt-PT",
        };
        let ns_trie = NamespaceTrie::new();

        let res = handle_literal(lit, &ns_trie);

        assert!(res.is_ok());

        match res.unwrap() {
            NormalizedResource::Literal(lit) => {
                assert_eq!(lit.lang, Some("pt-PT".into()));
            }
            _ => panic!("Result should be a NormalizedResource::Literal"),
        }
    }

    #[test]
    fn handle_literal_typed() {
        let iri = "http://example.org/#my-datatype";
        let ns = "http://example.org/";
        let alias = "example";

        let dt = NamedNode { iri };
        let lit = Literal::Typed {
            value: "my-lit",
            datatype: dt,
        };

        let mut ns_trie = NamespaceTrie::new();
        ns_trie.insert(ns, (alias.into(), NamespaceSource::User));

        let res = handle_literal(lit, &ns_trie);

        assert!(res.is_ok());

        match res.unwrap() {
            NormalizedResource::TypedLiteral(lit) => {
                assert_eq!(lit.alias, "example");
                assert_eq!(lit.namespace, ns);
                assert_eq!(lit.iri, "http://example.org/#my-datatype");
            }
            _ => panic!(
                "Result should be a NormalizedResource::Literal, but got:\n{:#?}",
                lit
            ),
        }
    }

    #[test]
    fn handle_literal_typed_unknown() {
        let dt_iri = "http://example.org/#my-datatype";
        let _alias = "mydt";

        let dt = NamedNode { iri: dt_iri };
        let lit = Literal::Typed {
            value: "my-lit",
            datatype: dt,
        };

        let ns_trie = NamespaceTrie::new();

        let res = handle_literal(lit, &ns_trie);

        assert!(res.is_err());

        let UnknownNamespaceError { iri } = res.unwrap_err();
        assert_eq!(iri, dt_iri)
    }

    #[test]
    fn handle_named_node_known() {
        let mut ns_trie = NamespaceTrie::new();
        ns_trie.insert("http://ex.org/", ("ex".into(), NamespaceSource::User));
        let nn = NamedNode {
            iri: "http://ex.org/foo",
        };
        let res = handle_named_node(nn, &ns_trie).unwrap();
        assert_eq!(
            res,
            NormalizedResource::NamedNode(NNode {
                alias: "ex".into(),
                namespace: "http://ex.org/".into()
            })
        );
    }

    #[test]
    fn handle_named_node_unknown() {
        let ns_trie = NamespaceTrie::new();
        let nn = NamedNode {
            iri: "http://unknown.org/foo",
        };
        let res = handle_named_node(nn, &ns_trie);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().iri, "http://unknown.org/foo");
    }

    #[test]
    fn handle_subject_blank() {
        let ns_trie = NamespaceTrie::new();
        let sub = Subject::BlankNode(rio_api::model::BlankNode { id: "b1" });
        let res = handle_subject(sub, &ns_trie).unwrap();
        assert_eq!(res, NormalizedResource::BlankNode);
    }

    #[test]
    fn handle_subject_named() {
        let mut ns_trie = NamespaceTrie::new();
        ns_trie.insert("http://ex.org/", ("ex".into(), NamespaceSource::User));
        let sub = Subject::NamedNode(NamedNode {
            iri: "http://ex.org/s",
        });
        let res = handle_subject(sub, &ns_trie).unwrap();
        assert_eq!(
            res,
            NormalizedResource::NamedNode(NNode {
                alias: "ex".into(),
                namespace: "http://ex.org/".into()
            })
        );
    }

    #[test]
    fn handle_object_blank() {
        let ns_trie = NamespaceTrie::new();
        let obj = Term::BlankNode(rio_api::model::BlankNode { id: "b1" });
        let res = handle_object(obj, &ns_trie).unwrap();
        assert_eq!(res, NormalizedResource::BlankNode);
    }

    #[test]
    fn handle_object_literal_simple() {
        let ns_trie = NamespaceTrie::new();
        let obj = Term::Literal(Literal::Simple { value: "hello" });
        let res = handle_object(obj, &ns_trie).unwrap();
        assert_eq!(res, NormalizedResource::Literal(Lit { lang: None }));
    }

    #[test]
    fn proc_triple_all_known() {
        let mut ns_trie = NamespaceTrie::new();
        ns_trie.insert("http://ex.org/", ("ex".into(), NamespaceSource::User));
        let triple = Triple {
            subject: Subject::NamedNode(NamedNode {
                iri: "http://ex.org/s",
            }),
            predicate: NamedNode {
                iri: "http://ex.org/p",
            },
            object: Term::NamedNode(NamedNode {
                iri: "http://ex.org/o",
            }),
        };
        let (tx, rx) = std::sync::mpsc::sync_channel(100);
        let (iris, blanks, literals) = proc_triple(triple, &tx, &ns_trie, false);

        assert_eq!(iris, 3);
        assert_eq!(blanks, 0);
        assert_eq!(literals, 0);

        let msg = rx.try_recv().unwrap();
        match msg {
            Message::NormalizedTriple {
                subject,
                predicate,
                object,
            } => {
                assert_eq!(
                    subject,
                    NormalizedResource::NamedNode(NNode {
                        alias: "ex".into(),
                        namespace: "http://ex.org/".into()
                    })
                );
                assert_eq!(
                    predicate,
                    NormalizedResource::NamedNode(NNode {
                        alias: "ex".into(),
                        namespace: "http://ex.org/".into()
                    })
                );
                assert_eq!(
                    object,
                    NormalizedResource::NamedNode(NNode {
                        alias: "ex".into(),
                        namespace: "http://ex.org/".into()
                    })
                );
            }
            _ => panic!("Expected NormalizedTriple"),
        }
    }

    #[test]
    fn count_resources_all_iris() {
        let sub = Subject::NamedNode(NamedNode {
            iri: "http://ex.org/s",
        });
        let obj = Term::NamedNode(NamedNode {
            iri: "http://ex.org/o",
        });
        let (iris, blanks, literals) = count_resources(&sub, &obj);
        assert_eq!(iris, 3); // subject + predicate + object
        assert_eq!(blanks, 0);
        assert_eq!(literals, 0);
    }

    #[test]
    fn count_resources_blank_literal() {
        let sub = Subject::BlankNode(rio_api::model::BlankNode { id: "b1" });
        let obj = Term::Literal(rio_api::model::Literal::Simple { value: "hello" });
        let (iris, blanks, literals) = count_resources(&sub, &obj);
        assert_eq!(iris, 1); // predicate only
        assert_eq!(blanks, 1);
        assert_eq!(literals, 1);
    }

    #[test]
    fn proc_message_all_named() {
        let mut triples = TripleFreq::new();
        let mut groups = Groups::default();
        proc_message(
            NormalizedResource::NamedNode(NNode {
                alias: "ex".into(),
                namespace: "http://ex.org/".into(),
            }),
            NormalizedResource::NamedNode(NNode {
                alias: "ex".into(),
                namespace: "http://ex.org/".into(),
            }),
            NormalizedResource::NamedNode(NNode {
                alias: "ex".into(),
                namespace: "http://ex.org/".into(),
            }),
            &mut triples,
            &mut groups,
        );
        assert!(groups.namespaces.contains(&GroupNS {
            alias: "ex".into(),
            namespace: "http://ex.org/".into()
        }));
        assert!(!groups.unknown);
        assert!(!groups.blank);
    }

    #[test]
    fn proc_message_unknown() {
        let mut triples = TripleFreq::new();
        let mut groups = Groups::default();
        proc_message(
            NormalizedResource::Unknown,
            NormalizedResource::Unknown,
            NormalizedResource::Unknown,
            &mut triples,
            &mut groups,
        );
        assert!(groups.unknown);
    }

    #[test]
    fn proc_message_blank() {
        let mut triples = TripleFreq::new();
        let mut groups = Groups::default();
        proc_message(
            NormalizedResource::BlankNode,
            NormalizedResource::BlankNode,
            NormalizedResource::BlankNode,
            &mut triples,
            &mut groups,
        );
        assert!(groups.blank);
    }

    #[test]
    fn proc_message_literal_simple() {
        let mut triples = TripleFreq::new();
        let mut groups = Groups::default();
        proc_message(
            NormalizedResource::Literal(Lit { lang: None }),
            NormalizedResource::Literal(Lit { lang: None }),
            NormalizedResource::Literal(Lit { lang: None }),
            &mut triples,
            &mut groups,
        );
        assert!(groups.namespaces.contains(&GroupNS {
            alias: "xsd".into(),
            namespace: "http://www.w3.org/TR/xmlschema11-2/".into()
        }));
    }
}
