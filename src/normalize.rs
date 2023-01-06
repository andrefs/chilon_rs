use crate::{ns_trie::NamespaceTrie, parse::parse};
use log::{debug, error, info, trace, warn};
use rayon::ThreadPoolBuilder;
use rio_api::{
    formatter::TriplesFormatter,
    model::Triple,
    model::{BlankNode, Literal, NamedNode, Subject, Term},
    parser::TriplesParser,
};
use rio_turtle::{TurtleError, TurtleFormatter, TurtleParser};
use std::{
    collections::BTreeMap,
    fs::OpenOptions,
    io::{BufRead, Write},
    path::{Path, PathBuf},
    sync::mpsc::{channel, Sender},
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

#[derive(Debug, Clone)]
pub enum NormalizedResource {
    Unknown,
    BlankNode,
    Literal(Lit),
    TypedLiteral(TypedLit),
    NamedNode(NNode),
}

#[derive(Debug, Clone)]
pub struct Lit {
    data_type: Option<String>,
}
#[derive(Debug, Clone)]
pub struct TypedLit {
    namespace: String,
    alias: String,
    iri: String,
}

#[derive(Debug, Clone)]
pub struct NNode {
    alias: String,
    namespace: String,
}

impl From<NormalizedResource> for String {
    fn from(nr: NormalizedResource) -> Self {
        match nr {
            NormalizedResource::Unknown => "UNKNOWN".to_string(),
            NormalizedResource::BlankNode => "BLANK".to_string(),
            NormalizedResource::Literal(Lit { data_type }) => match data_type {
                None => "STRING".into(),
                Some(dt) => {
                    if dt == "lang-string" {
                        "LANG-STRING".to_string()
                    } else {
                        dt.to_string()
                    }
                }
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

pub fn normalize_triples(
    paths: Vec<PathBuf>,
    ns_trie: &NamespaceTrie,
    ignore_unknown: bool,
    outf: &str,
) -> (TripleFreq, BTreeMap<String, String>) {
    let mut triples = TripleFreq::new();
    let mut used_ns = BTreeMap::<String, String>::new();

    let n_workers = std::cmp::max(2, std::cmp::min(paths.len(), num_cpus::get() - 2));
    info!("Creating pool with {n_workers} threads");

    let mut running = paths.len();
    let pool = ThreadPoolBuilder::new()
        .num_threads(n_workers)
        .build()
        .unwrap();

    pool.scope_fifo(|s| {
        let (tx, rx) = channel::<Message>();
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
                        if let NormalizedResource::NamedNode(NNode { alias, namespace }) = subject {
                            used_ns.insert(alias, namespace);
                        }
                        if let NormalizedResource::NamedNode(NNode { alias, namespace }) = predicate
                        {
                            used_ns.insert(alias, namespace);
                        }
                        match object {
                            NormalizedResource::NamedNode(NNode { alias, namespace }) => {
                                used_ns.insert(alias, namespace);
                            }
                            NormalizedResource::TypedLiteral(TypedLit {
                                namespace,
                                alias,
                                iri: _,
                            }) => {
                                used_ns.insert(alias, namespace);
                            }
                            _ => {}
                        }
                    }
                    Message::NamespaceUnknown { iri } => {
                        let msg = format!("Unknown namespace for resource '{iri}'");
                        warn!("{msg}");
                        writeln!(fd, "Unknown namespace for resource '{iri}'").unwrap();
                        //log_error_to_file(fd, msg);
                    }
                    Message::Finished => {
                        running -= 1;
                    }
                }
            }
        }
    });

    return (triples, used_ns);
}

fn proc_triples(
    graph: &mut TurtleParser<impl BufRead>,
    path: &PathBuf,
    tx: &Sender<Message>,
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
            error!("Error normalizing file {}: {}", path.to_string_lossy(), err);
        }
    }
    tx.send(Message::Finished).unwrap();
}

fn proc_triple<E>(
    t: Triple,
    tx: &Sender<Message>,
    ns_trie: &NamespaceTrie,
    ignore_unknown: bool,
) -> Result<(), E> {
    let subject = handle_subject(t.subject, ns_trie);
    let predicate = handle_predicate(t.predicate, ns_trie);
    let object = handle_object(t.object, ns_trie);

    if !ignore_unknown {}
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
        if node.value.is_some() {
            return Ok(NormalizedResource::NamedNode(NNode {
                alias: node.value.as_ref().unwrap().clone(),
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
        Literal::Simple { value: _ } => Ok(NormalizedResource::Literal(Lit { data_type: None })),
        Literal::LanguageTaggedString {
            value: _,
            language: _,
        } => Ok(NormalizedResource::Literal(Lit {
            data_type: Some("lang-string".into()),
        })),
        Literal::Typed { value: _, datatype } => {
            let res = ns_trie.longest_prefix(datatype.iri, true);
            if let Some((node, ns)) = res {
                if node.value.is_some() {
                    let alias = node.value.as_ref().unwrap().clone();
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
    used_ns: BTreeMap<String, String>,
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

    writeln!(fd, "@base <http://andrefs.com/graph-summ/v1> .").unwrap();
    writeln!(fd, "").unwrap();

    let rdf = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

    let mut formatter = TurtleFormatter::new(fd);

    // print namespace alias
    for (alias, namespace) in used_ns {
        formatter
            .format(&Triple {
                subject: NamedNode {
                    iri: alias.as_str(),
                }
                .into(),
                predicate: NamedNode {
                    iri: "namespacePrefix",
                }
                .into(),
                object: NamedNode {
                    iri: namespace.as_str(),
                }
                .into(),
            })
            .unwrap();
    }
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
