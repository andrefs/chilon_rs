use crate::ns_trie::NamespaceTrie;
use crate::parse::parse;
use crate::util::gen_file_name;
use log::{debug, info, trace, warn};
use rayon::{ThreadPool, ThreadPoolBuilder};
use rio_api::formatter::TriplesFormatter;
use rio_api::model::Triple;
use rio_api::model::{Literal, NamedNode, Subject, Term};
use rio_turtle::TurtleFormatter;
use rio_turtle::{TurtleError, TurtleParser};
use std::fmt::format;
use std::fs::{write, OpenOptions};
use std::io::{BufRead, Write};
use std::time::Instant;
use std::{
    collections::BTreeMap,
    error::Error,
    path::PathBuf,
    sync::mpsc::{channel, Sender},
};

use rio_api::parser::TriplesParser;

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
        subject: String,
        predicate: String,
        object: String,
    },
    NamespaceUnknown {
        iri: String,
    },
    Finished,
}

pub fn normalize_triples(paths: Vec<PathBuf>, ns_trie: &NamespaceTrie) -> TripleFreq {
    let mut triples = TripleFreq::new();
    let n_workers = std::cmp::max(2, std::cmp::min(paths.len(), num_cpus::get() - 2));
    info!("Creating pool with {n_workers} threads");
    let pool = ThreadPoolBuilder::new()
        .num_threads(n_workers)
        .build()
        .unwrap();
    let mut running = paths.len();

    pool.scope_fifo(|s| {
        let (tx, rx) = channel::<Message>();
        for path in paths {
            let tx = tx.clone();
            s.spawn_fifo(move |_| {
                debug!("Parsing {:?}", path);
                let mut graph = parse(&path);
                proc_triples(&mut graph, &tx, ns_trie);
            });
        }

        let mut i = 0;
        let mut last_i = 0;
        let mut start = Instant::now();
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
                        triples.add((subject, predicate, object));
                    }
                    Message::NamespaceUnknown { iri: err } => {
                        //if let UnknownNamespaceError { iri } = err {
                        //    let msg = format!("Unknown namespace for resource {iri}");
                        //    warn!("{msg}");
                        //    //log_error_to_file(msg);
                        //}
                    }
                    Message::Finished => {
                        running -= 1;
                    }
                }
            }
        }
    });

    return triples;
}
fn proc_triples(
    graph: &mut TurtleParser<impl BufRead>,
    tx: &Sender<Message>,
    ns_trie: &NamespaceTrie,
) {
    let tid = if let Some(id) = rayon::current_thread_index() {
        id.to_string()
    } else {
        "".to_string()
    };
    let mut i = 0;
    let mut last_i = 0;
    let mut start = Instant::now();
    graph
        .parse_all(&mut |t| {
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

            let subject = handle_subject(t.subject, ns_trie);
            let predicate = handle_predicate(t.predicate, ns_trie);
            let object = handle_object(t.object, ns_trie);

            if let Err(UnknownNamespaceError { iri: _ }) = subject {
                if i == 0 {}
                tx.send(Message::NamespaceUnknown {
                    iri: t.subject.to_string(),
                })
                .unwrap();
            }
            if let Err(UnknownNamespaceError { iri: _ }) = predicate {
                if i == 0 {}
                tx.send(Message::NamespaceUnknown {
                    iri: t.predicate.to_string(),
                })
                .unwrap();
            }
            if let Err(UnknownNamespaceError { iri: _ }) = object {
                if i == 0 {}
                tx.send(Message::NamespaceUnknown {
                    iri: t.object.to_string(),
                })
                .unwrap();
            }

            tx.send(Message::NormalizedTriple {
                subject: match subject {
                    Ok(ns) => ns.clone(),
                    Err(UnknownNamespaceError { iri: _ }) => "[UNKNOWN]".to_string(),
                },
                predicate: match predicate {
                    Ok(ns) => ns.clone(),
                    Err(UnknownNamespaceError { iri: _ }) => "[UNKNOWN]".to_string(),
                },
                object: match object {
                    Ok(ns) => ns.clone(),
                    Err(UnknownNamespaceError { iri: _ }) => "[UNKNOWN]".to_string(),
                },
            })
            .unwrap();

            Ok(()) as Result<(), TurtleError>
        })
        .unwrap();
    tx.send(Message::Finished).unwrap();
}

fn log_error_to_file(err: String) {
    let base_path = "results/errors".to_string();
    let ext = "log".to_string();
    let file_path = gen_file_name(base_path, ext);

    let mut fd = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path.clone())
        .unwrap();
    writeln!(fd, "{}", err).unwrap();
}

fn handle_subject(sub: Subject, ns_trie: &NamespaceTrie) -> Result<String, UnknownNamespaceError> {
    match sub {
        Subject::BlankNode(_) => Ok("[BLANK]".to_string()),
        Subject::Triple(_) => unimplemented!(),
        Subject::NamedNode(n) => handle_named_node(n, ns_trie),
    }
}

fn handle_predicate(
    pred: NamedNode,
    ns_trie: &NamespaceTrie,
) -> Result<String, UnknownNamespaceError> {
    handle_named_node(pred, ns_trie)
}

fn handle_object(obj: Term, ns_trie: &NamespaceTrie) -> Result<String, UnknownNamespaceError> {
    match obj {
        Term::BlankNode(_) => Ok("[BLANK]".to_string()),
        Term::Triple(_) => unimplemented!(),
        Term::NamedNode(n) => handle_named_node(n, ns_trie),
        Term::Literal(lit) => Ok(handle_literal(lit)),
    }
}

#[derive(Debug, Clone)]
pub struct UnknownNamespaceError {
    iri: String,
}

fn handle_named_node(
    n: NamedNode,
    ns_trie: &NamespaceTrie,
) -> Result<String, UnknownNamespaceError> {
    let res = ns_trie.longest_prefix(n.iri, true);
    if let Some((node, _)) = res {
        if node.value.is_some() {
            return Ok(node.value.as_ref().unwrap().clone());
        }
    }
    return Err(UnknownNamespaceError {
        iri: n.iri.to_string(),
    });
}

fn handle_literal(lit: Literal) -> String {
    match lit {
        Literal::Simple { value: _ } => "[LITERAL:string]".to_string(),
        Literal::LanguageTaggedString {
            value: _,
            language: _,
        } => "[LITERAL:lang-string]".to_string(),
        Literal::Typed { value: _, datatype } => format!("[LITERAL:{datatype}]").to_string(),
    }
}

pub fn save_normalized_triples(nts: &TripleFreq) {
    let base_path = "results/output".to_string();
    let ext = "ttl".to_string();
    let file_path = gen_file_name(base_path, ext);
    info!("Saving graph summary to {}", file_path);

    let mut id_count = 1;

    let base_url = "http://andrefs.com/graph-summ/v1/".clone();
    let fd = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();
    let rdf = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";

    let mut formatter = TurtleFormatter::new(fd);

    for tf in nts.iter_all() {
        let t_id = format!("{base_url}t{:0width$}", id_count, width = 4);

        formatter
            .format(&Triple {
                subject: NamedNode { iri: &t_id }.into(),
                predicate: NamedNode {
                    iri: format!("{rdf}type").as_str(),
                },
                object: NamedNode {
                    iri: format!("{rdf}Statement").as_str(),
                }
                .into(),
            })
            .unwrap();

        id_count += 1;
        formatter
            .format(&Triple {
                subject: NamedNode { iri: &t_id }.into(),
                predicate: NamedNode {
                    iri: format!("{rdf}subject").as_str(),
                },
                object: NamedNode {
                    iri: format!("{base_url}{}", tf.0).as_str(),
                }
                .into(),
            })
            .unwrap();
        formatter
            .format(&Triple {
                subject: NamedNode { iri: &t_id }.into(),
                predicate: NamedNode {
                    iri: format!("{rdf}predicate").as_str(),
                },
                object: NamedNode {
                    iri: format!("{base_url}{}", tf.1).as_str(),
                }
                .into(),
            })
            .unwrap();
        formatter
            .format(&Triple {
                subject: NamedNode { iri: &t_id }.into(),
                predicate: NamedNode {
                    iri: format!("{rdf}object").as_str(),
                },
                object: NamedNode {
                    iri: format!("{base_url}{}", tf.2).as_str(),
                }
                .into(),
            })
            .unwrap();
        formatter
            .format(&Triple {
                subject: NamedNode { iri: &t_id }.into(),
                predicate: NamedNode {
                    iri: format!("{base_url}occurrences").as_str(),
                },
                object: Literal::Simple {
                    value: tf.3.to_string().as_str(),
                }
                .into(),
            })
            .unwrap();

        id_count += 1;
    }
    formatter.finish().unwrap();
}
