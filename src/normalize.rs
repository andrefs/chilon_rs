use crate::{
    ns_trie::NamespaceTrie,
    parse::{parse, ParserWrapper},
};
use log::{debug, info, trace};
use rayon::ThreadPoolBuilder;
use rio_api::{
    formatter::TriplesFormatter,
    model::Triple,
    model::{Literal, NamedNode, Subject, Term},
    parser::TriplesParser,
};
use rio_turtle::{TurtleError, TurtleFormatter};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::mpsc::{sync_channel, Sender, SyncSender},
    time::Instant,
};

type TripleFreq = BTreeMap<String, TripleFreqSec>;
type TripleFreqSec = BTreeMap<String, TripleFreqThird>;
type TripleFreqThird = BTreeMap<String, i32>;

trait TripleFreqFns {
    fn add(&mut self, triple: (String, String, String));
    fn iter_all(&self) -> Vec<(String, String, String, i32)>;
}

impl TripleFreqFns for TripleFreq {
    fn add(&mut self, triple: (String, String, String)) {
        let count = self
            .entry(triple.0)
            .or_default()
            .entry(triple.1)
            .or_default()
            .entry(triple.2)
            .or_default();
        *count += 1;
    }

    fn iter_all(&self) -> Vec<(String, String, String, i32)> {
        self.into_iter()
            .flat_map(|(s, m)| {
                m.into_iter().flat_map(|(p, m)| {
                    m.into_iter()
                        .map(|(o, count)| (s.clone(), p.clone(), o.clone(), *count))
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    NormalizedTriple {
        subject: NormalizedResource,
        predicate: NormalizedResource,
        object: NormalizedResource,
    },
    NamespaceUnknown {
        iri: String,
    },
    Finished,
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
                None => "STRING".into(),
                Some(l) => format!("STRING@{l}"),
            },
            NormalizedResource::TypedLiteral(TypedLit {
                namespace,
                alias,
                iri,
            }) => {
                format!("{}:{}", alias, &iri[namespace.len()..])
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
    namespaces: BTreeSet<GroupNS>,
    blank: bool,
    unknown: bool,
}

pub fn normalize_triples(
    paths: Vec<PathBuf>,
    ns_trie: &NamespaceTrie,
    ignore_unknown: bool,
    outf: &str,
) -> (TripleFreq, Groups) {
    let mut triples = TripleFreq::new();
    let mut used_groups: Groups = Default::default();

    let n_workers = std::cmp::max(2, std::cmp::min(paths.len(), num_cpus::get() - 2));
    info!("Creating pool with {n_workers} threads");

    let mut running = paths.len();
    let pool = ThreadPoolBuilder::new()
        .num_threads(n_workers)
        .build()
        .unwrap();

    pool.scope_fifo(|s| {
        let (tx, rx) = sync_channel::<Message>(100);
        for path in paths {
            let tx = tx.clone();
            s.spawn_fifo(move |_| {
                debug!("Parsing {:?}", path);
                let mut graph = parse(&path);
                proc_triples(&mut graph, &path, &tx, ns_trie, ignore_unknown);
            });
        }

        let mut i = 0;
        let mut last_i = 0;
        let mut start = Instant::now();

        let file_path = Path::new(".").join(outf).join("errors.log");

        let mut fd = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path.clone())
            .unwrap();

        loop {
            i += 1;
            if i % 1_000_000 == 1 {
                let elapsed = start.elapsed().as_millis();
                if elapsed != 0 {
                    trace!(
                        "Normalized {i} triples so far ({} triples/s)",
                        ((i - last_i) / elapsed) * 1000
                    );
                }
                last_i = i;
                start = Instant::now();
            }
            if running == 0 {
                info!("All threads finished");
                break;
            }
            if let Ok(message) = rx.recv() {
                proc_message(
                    message,
                    &mut triples,
                    &mut used_groups,
                    &mut fd,
                    &mut running,
                );
            }
        }
    });

    return (triples, used_groups);
}

fn proc_message(
    message: Message,
    triples: &mut TripleFreq,
    used_groups: &mut Groups,
    fd: &mut File,
    running: &mut usize,
) {
    match message {
        Message::NormalizedTriple {
            subject,
            predicate,
            object,
        } => {
            triples.add((
                subject.clone().into(),
                predicate.clone().into(),
                object.clone().into(),
            ));

            for resource in vec![subject, predicate, object] {
                match resource {
                    NormalizedResource::Unknown => {
                        used_groups.unknown = true;
                    }
                    NormalizedResource::BlankNode => {
                        used_groups.blank = true;
                    }
                    NormalizedResource::Literal(Lit { lang }) => {
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
                        used_groups.namespaces.insert(GroupNS { alias, namespace });
                    }
                    NormalizedResource::NamedNode(NNode { alias, namespace }) => {
                        used_groups.namespaces.insert(GroupNS { alias, namespace });
                    }
                }
            }
        }
        Message::NamespaceUnknown { iri } => {
            let msg = format!("Unknown namespace for resource '{iri}'");
            writeln!(fd, "Unknown namespace for resource '{iri}'").unwrap();
        }
        Message::Finished => {
            *running -= 1;
        }
    }
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
    while !graph.is_end() {
        i += 1;

        if i % 1_000_000 == 1 && !start.elapsed().is_zero() {
            let elapsed = start.elapsed().as_millis();
            if elapsed != 0 {
                trace!(
                    "[Thread#{tid}] Parsed {i} triples so far ({} triples/s)",
                    ((i - last_i) as u128 / elapsed) * 1000
                );
            }
            last_i = i;
            start = Instant::now();
        }
        if let Err(err) =
            graph.parse_step(&mut |t| proc_triple::<TurtleError>(t, tx, ns_trie, ignore_unknown))
        {
            panic!("Error normalizing file {}: {}", path.to_string_lossy(), err);
        }
    }
    tx.send(Message::Finished).unwrap();
}

fn proc_triple<E>(
    t: Triple,
    tx: &SyncSender<Message>,
    ns_trie: &NamespaceTrie,
    ignore_unknown: bool,
) -> Result<(), E> {
    let subject = handle_subject(t.subject, ns_trie);
    let predicate = handle_predicate(t.predicate, ns_trie);
    let object = handle_object(t.object, ns_trie);

    if ignore_unknown {
        if let Err(UnknownNamespaceError) = subject {
            return Ok(());
        }
        if let Err(UnknownNamespaceError) = predicate {
            return Ok(());
        }
        if let Err(UnknownNamespaceError) = object {
            return Ok(());
        }
    }

    if let Err(UnknownNamespaceError { iri: _ }) = subject {
        if let Subject::NamedNode(NamedNode { iri }) = t.subject {
            tx.send(Message::NamespaceUnknown {
                iri: iri.to_string(),
            })
            .unwrap();
        }
    }
    if let Err(UnknownNamespaceError { iri: _ }) = predicate {
        tx.send(Message::NamespaceUnknown {
            iri: t.predicate.to_string(),
        })
        .unwrap();
    }
    if let Err(UnknownNamespaceError { iri: _ }) = object {
        if let Term::NamedNode(NamedNode { iri }) = t.object {
            tx.send(Message::NamespaceUnknown {
                iri: iri.to_string(),
            })
            .unwrap();
        }
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

    Ok(()) as Result<(), E>
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
    for tf in nts.iter_all() {
        if min_occurs.is_some() && tf.3 < min_occurs.unwrap() {
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
                object: NamedNode { iri: "#GroupsLink" }.into(),
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
                    iri: format!("#{}", tf.0).as_str(),
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
                    iri: format!("#{}", tf.1).as_str(),
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
                    iri: format!("#{}", tf.2).as_str(),
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
                    value: tf.3.to_string().as_str(),
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

    formatter
        .format(&Triple {
            subject: NamedNode {
                iri: format!("#{}", group.alias).as_str(),
            }
            .into(),
            predicate: NamedNode {
                iri: "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
            }
            .into(),
            object: NamedNode {
                iri: &group.namespace,
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
