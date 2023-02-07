use crate::{
    counter::Counter,
    meta_info::{Task, TaskType},
    ns_trie::NamespaceTrie,
    parse::{parse, ParserWrapper},
};
use log::{error, info, trace};
use rayon::ThreadPoolBuilder;
use rio_api::{
    formatter::TriplesFormatter,
    model::{Literal, NamedNode, Subject, Term, Triple},
    parser::TriplesParser,
};
use rio_turtle::{TurtleError, TurtleFormatter};
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
        self.into_iter()
            .flat_map(|(s, m)| {
                m.into_iter().flat_map(|(p, m)| {
                    m.into_iter().flat_map(|(o, m)| {
                        m.into_iter()
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
        err: TurtleError,
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
                namespace,
                alias,
                iri,
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
    ns_trie: &NamespaceTrie,
    ignore_unknown: bool,
    outf: &str,
) -> (TripleFreq, Groups, BTreeMap<String, Task>) {
    let mut triples = TripleFreq::new();
    let mut used_groups: Groups = Default::default();

    let n_workers = std::cmp::max(2, std::cmp::min(paths.len() + 1, num_cpus::get() - 2));
    info!("Creating pool with {n_workers} threads");

    let mut running = paths.len();
    let pool = ThreadPoolBuilder::new()
        .num_threads(n_workers)
        .build()
        .unwrap();

    let mut trip_c = 0;

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
            .open(errors_path.clone())
            .unwrap();

        handle_loop(
            &mut running,
            rx,
            &mut triples,
            &mut used_groups,
            &mut tasks,
            &mut fd,
        );
    });

    return (triples, used_groups, tasks);
}

fn handle_loop(
    running: &mut usize,
    rx: Receiver<Message>,
    triples: &mut TripleFreq,
    used_groups: &mut Groups,
    tasks: &mut BTreeMap<String, Task>,
    fd: &mut File,
) {
    let msg_c = &mut Counter::default();
    let trip_c = &mut Counter::default();
    let start = &mut Instant::now();

    loop {
        msg_c.inc();
        if msg_c.cur % 1_000_000 == 1 {
            restart_timers(start, msg_c, trip_c);
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
                    for iri in iris.iter() {
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
                    let mut t = tasks.get_mut(&path).unwrap();
                    t.triples = triples;
                    t.iris = iris;
                    t.blanks = blanks;
                    t.literals = literals;
                    t.finish(format!("Finished task {:?} on {}", t.task_type, t.name).as_str());

                    *running -= 1;
                }
                Message::FatalError { err } => {
                    *running -= 1;
                }
            }
        }
    }
}

fn restart_timers(start: &mut Instant, msg_c: &mut Counter, trip_c: &mut Counter) {
    let elapsed = start.elapsed().as_millis();
    if elapsed != 0 {
        trace!(
            "Received {} messages ({}/s), {} triples ({}/s) so far",
            msg_c.cur,
            (msg_c.delta() as u128 / elapsed) * 1000,
            trip_c.cur,
            (trip_c.delta() as u128 / elapsed) * 1000,
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

    for resource in vec![subject.clone(), predicate.clone(), object.clone()] {
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
    path: &PathBuf,
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

    let mut iri_c = 0;
    let mut blank_c = 0;
    let mut literal_c = 0;

    while !graph.is_end() {
        i += 1;

        if i % 1_000_000 == 1 && !start.elapsed().is_zero() {
            let elapsed = start.elapsed().as_millis();
            if elapsed != 0 {
                trace!(
                    "[Thread#{tid}] Parsed {i} triples so far ({} triples/s)",
                    ((i - last_i) / elapsed) * 1000
                );
            }
            last_i = i;
            start = Instant::now();
        }
        let res = graph.parse_step(&mut |t| {
            let (iris, blanks, literals) =
                proc_triple::<TurtleError>(t, tx, ns_trie, ignore_unknown);
            iri_c += iris;
            blank_c += blanks;
            literal_c += literals;
            Ok(())
        });

        if let Err(err) = res {
            let msg = format!("Error normalizing file {}: {}", path.to_string_lossy(), err);
            error!("{}", msg);
            tx.send(Message::FatalError { err }).unwrap();
            panic!("{}", msg);
        }
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

fn count_resources(subject: &Subject, object: &Term) -> (usize, usize, usize) {
    let mut iris = 0;
    let mut blanks = 0;
    let mut literals = 0;

    match subject {
        Subject::NamedNode(_) => iris += 1,
        Subject::BlankNode(_) => blanks += 1,
        Subject::Triple(_) => {
            unimplemented!("Triple subjects are not supported yet")
        }
    }

    // predicate is always a NamedNode
    iris += 1;

    match object {
        Term::NamedNode(_) => iris += 1,
        Term::BlankNode(_) => blanks += 1,
        Term::Literal(_) => literals += 1,
        Term::Triple(_) => {
            unimplemented!("Triple objects are not supported yet")
        }
    }

    (iris, blanks, literals)
}

fn proc_triple<E>(
    t: Triple,
    tx: &SyncSender<Message>,
    ns_trie: &NamespaceTrie,
    ignore_unknown: bool,
) -> (usize, usize, usize) {
    let subject = handle_subject(t.subject, ns_trie);
    let predicate = handle_predicate(t.predicate, ns_trie);
    let object = handle_object(t.object, ns_trie);

    let (iris, blanks, literals) = count_resources(&t.subject, &t.object);

    if ignore_unknown {
        for res in vec![&subject, &predicate, &object] {
            if let Err(UnknownNamespaceError) = res {
                return (iris, blanks, literals);
            }
        }
    }

    let mut unknown_ns = Vec::new();

    if let Err(UnknownNamespaceError { iri: _ }) = subject {
        if let Subject::NamedNode(NamedNode { iri }) = t.subject {
            unknown_ns.push(iri.to_string());
        }
    }
    if let Err(UnknownNamespaceError { iri: _ }) = predicate {
        unknown_ns.push(t.predicate.to_string());
    }

    if let Err(UnknownNamespaceError { iri: _ }) = object {
        if let Term::NamedNode(NamedNode { iri }) = t.object {
            unknown_ns.push(iri.to_string());
        }
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
    sub: Subject,
    ns_trie: &NamespaceTrie,
) -> Result<NormalizedResource, UnknownNamespaceError> {
    match sub {
        Subject::BlankNode(_) => Ok(NormalizedResource::BlankNode),
        Subject::Triple(_) => unimplemented!(),
        Subject::NamedNode(n) => handle_named_node(n, ns_trie),
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
        Term::Triple(_) => unimplemented!(),
        Term::NamedNode(n) => handle_named_node(n, ns_trie),
        Term::Literal(lit) => handle_literal(lit, ns_trie),
    }
}

#[derive(Debug, Clone)]
pub struct UnknownNamespaceError {
    iri: String,
}

fn handle_named_node(
    n: NamedNode,
    ns_trie: &NamespaceTrie,
) -> Result<NormalizedResource, UnknownNamespaceError> {
    let res = ns_trie.longest_prefix(n.iri, true);
    if let Some((node, ns)) = res {
        if let Some((alias, source)) = &node.value {
            return Ok(NormalizedResource::NamedNode(NNode {
                alias: alias.clone(),
                namespace: ns,
            }));
            //return Ok(node.value.as_ref().unwrap().clone());
        }
    }
    return Err(UnknownNamespaceError {
        iri: n.iri.to_string(),
    });
}

fn handle_literal(
    lit: Literal,
    ns_trie: &NamespaceTrie,
) -> Result<NormalizedResource, UnknownNamespaceError> {
    match lit {
        Literal::Simple { value: _ } => Ok(NormalizedResource::Literal(Lit { lang: None })),
        Literal::LanguageTaggedString { value: _, language } => {
            Ok(NormalizedResource::Literal(Lit {
                lang: Some(language.to_string()),
            }))
        }
        //Literal::Typed {
        //    value: _,
        //    datatype: _,
        //} => Ok(NormalizedResource::Literal(Lit {
        //    data_type: Some("other-datatype".into()),
        //})),
        Literal::Typed { value: _, datatype } => {
            let res = ns_trie.longest_prefix(datatype.iri, true);
            if let Some((node, ns)) = res {
                if node.value.is_some() {
                    let (alias, _) = node.value.as_ref().unwrap().clone();
                    return Ok(NormalizedResource::TypedLiteral(TypedLit {
                        namespace: ns,
                        alias,
                        iri: datatype.iri.into(),
                    }));
                }
            }
            return Err(UnknownNamespaceError {
                iri: datatype.iri.to_string(),
            });
        }
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
        .open(file_path.clone())
        .unwrap();

    let base = "http://andrefs.com/graph-summ/v1";
    writeln!(fd, "@base <{}> .", { base }).unwrap();
    writeln!(fd, "@prefix ngont: <{}/ontology> .", base).unwrap(); // ontology (data-types?, unknown, blank, classes and predicates, etc)
    writeln!(fd, "@prefix ngns: <{}/instance> .", base).unwrap(); // namespaces (kgs, data types?)
    writeln!(fd, "").unwrap();

    let rdf = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

    let mut formatter = TurtleFormatter::new(fd);
    // print namespace alias
    format_groups(used_groups, &mut formatter);

    fd = formatter.finish().unwrap();
    writeln!(fd, "").unwrap();

    formatter = TurtleFormatter::new(fd);
    for (s, p, o, is_datatype, occurs) in nts.iter_all() {
        if min_occurs.is_some() && occurs < min_occurs.unwrap() {
            continue;
        }

        id_count += 1;
        let t_id = format!("#t{:0width$}", id_count, width = 4);

        // declare groups link
        formatter
            .format(&Triple {
                subject: NamedNode { iri: t_id.as_str() }.into(),
                predicate: NamedNode {
                    iri: format!("{rdf}type").as_str(),
                },
                object: NamedNode {
                    iri: if is_datatype {
                        "#DatatypeLink"
                    } else {
                        "#GroupsLink"
                    },
                }
                .into(),
            })
            .unwrap();

        // declare statement id
        formatter
            .format(&Triple {
                subject: NamedNode { iri: t_id.as_str() }.into(),
                predicate: NamedNode {
                    iri: format!("{rdf}type").as_str(),
                },
                object: NamedNode {
                    iri: format!("{rdf}Statement").as_str(),
                }
                .into(),
            })
            .unwrap();

        // declare statement subject
        formatter
            .format(&Triple {
                subject: NamedNode { iri: t_id.as_str() }.into(),
                predicate: NamedNode {
                    iri: format!("{rdf}subject").as_str(),
                },
                object: NamedNode {
                    iri: format!("#{}", s).as_str(),
                }
                .into(),
            })
            .unwrap();

        // declare statement predicate
        formatter
            .format(&Triple {
                subject: NamedNode { iri: t_id.as_str() }.into(),
                predicate: NamedNode {
                    iri: format!("{rdf}predicate").as_str(),
                },
                object: NamedNode {
                    iri: format!("#{}", p).as_str(),
                }
                .into(),
            })
            .unwrap();

        // declare statement object
        formatter
            .format(&Triple {
                subject: NamedNode { iri: t_id.as_str() }.into(),
                predicate: NamedNode {
                    iri: format!("{rdf}object").as_str(),
                },
                object: NamedNode {
                    iri: format!("#{}", o).as_str(),
                }
                .into(),
            })
            .unwrap();

        // declare number of occurrences
        formatter
            .format(&Triple {
                subject: NamedNode { iri: t_id.as_str() }.into(),
                predicate: NamedNode {
                    iri: "#occurrences",
                },
                object: Literal::Typed {
                    value: occurs.to_string().as_str(),
                    datatype: NamedNode {
                        iri: "http://www.w3.org/2001/XMLSchema#integer",
                    },
                }
                .into(),
            })
            .unwrap();
    }
    formatter.finish().unwrap();
}

pub fn format_groups(groups: Groups, formatter: &mut TurtleFormatter<File>) {
    if groups.blank {
        let blank = "http://andrefs.com/graph-summ/v1/ontology#BLANK";
    }

    for group in groups.namespaces {
        format_group(group, formatter);
    }
}

pub fn format_group(group: GroupNS, formatter: &mut TurtleFormatter<File>) {
    let unknown = "http://andrefs.com/graph-summ/v1/ontology#UNKNOWN";
    let ns = "http://andrefs.com/graph-summ/v1/ontology#Namespace";

    formatter
        .format(&Triple {
            subject: NamedNode {
                iri: format!("#{}", group.alias).as_str(),
            }
            .into(),
            predicate: NamedNode {
                iri: "namespacePrefix",
            }
            .into(),
            object: NamedNode {
                iri: group.namespace.as_str(),
            }
            .into(),
        })
        .unwrap();
}

#[cfg(test)]
mod tests {

    use crate::{ns_trie::NamespaceSource, trie::Node};

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
        let alias = "mydt";

        let dt = NamedNode { iri: dt_iri };
        let lit = Literal::Typed {
            value: "my-lit",
            datatype: dt,
        };

        let ns_trie = NamespaceTrie::new();

        let res = handle_literal(lit, &ns_trie);

        assert!(res.is_err());

        match res.unwrap_err() {
            UnknownNamespaceError { iri } => {
                assert_eq!(iri, dt_iri)
            }
            _ => panic!("Result should be an UnknownNamespaceError"),
        }
    }
}
