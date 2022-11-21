use crate::parse::parse;
use log::info;
use rayon::{ThreadPool, ThreadPoolBuilder};
use rio_api::formatter::TriplesFormatter;
use rio_api::model::Triple;
use rio_api::model::{BlankNode, Literal, NamedNode, Subject, Term};
use rio_turtle::TurtleError;
use rio_turtle::TurtleFormatter;

use crate::ns_trie::NamespaceTrie;
use std::fmt::format;
use std::fs::File;
use std::{
    collections::BTreeMap,
    ops::Add,
    path::PathBuf,
    sync::mpsc::{channel, Sender},
};
use std::{fs::OpenOptions, path::Path};

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

pub enum Message {
    NormalizedTriple {
        subject: String,
        predicate: String,
        object: String,
    },
    Finished,
}

pub fn normalize_triples(paths: Vec<PathBuf>, ns_trie: &NamespaceTrie) -> TripleFreq {
    let mut triples = TripleFreq::new();
    let n_workers = std::cmp::min(paths.len(), num_cpus::get() - 2);
    let pool = ThreadPoolBuilder::new()
        .num_threads(n_workers)
        .build()
        .unwrap();
    let mut running = paths.len();
    let (tx, rx) = channel::<Message>();

    for path in paths {
        spawn(&pool, &tx, path, ns_trie);
    }

    loop {
        if running == 0 {
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
                Message::Finished => {
                    running -= 1;
                }
            }
        }
    }

    return triples;
}

fn spawn(pool: &ThreadPool, tx: &Sender<Message>, path: PathBuf, ns_trie: &NamespaceTrie) {
    let tx = tx.clone();

    pool.scope(move |s| {
        s.spawn(move |s| {
            log::info!("  parsing {:?}", path);
            let mut graph = parse(&path);
            graph
                .parse_all(&mut |t| {
                    let subject = handle_subject(t.subject, ns_trie);
                    let predicate = handle_predicate(t.predicate, ns_trie);
                    let object = handle_object(t.object, ns_trie);

                    tx.send(Message::NormalizedTriple {
                        subject: subject.to_string(),
                        predicate,
                        object: object.to_string(),
                    })
                    .unwrap();
                    Ok(()) as Result<(), TurtleError>
                })
                .unwrap();
            tx.send(Message::Finished).unwrap();
        });
    });
}

fn handle_subject(sub: Subject, ns_trie: &NamespaceTrie) -> String {
    match sub {
        Subject::BlankNode(_) => "[BLANK]".to_string(),
        Subject::Triple(_) => unimplemented!(),
        Subject::NamedNode(n) => handle_named_node(n, ns_trie),
    }
}

fn handle_predicate(pred: NamedNode, ns_trie: &NamespaceTrie) -> String {
    handle_named_node(pred, ns_trie)
}

fn handle_object(obj: Term, ns_trie: &NamespaceTrie) -> String {
    match obj {
        Term::BlankNode(_) => "[BLANK]".to_string(),
        Term::Triple(_) => unimplemented!(),
        Term::NamedNode(n) => handle_named_node(n, ns_trie),
        Term::Literal(lit) => handle_literal(lit),
    }
}

fn handle_named_node(n: NamedNode, ns_trie: &NamespaceTrie) -> String {
    let res = ns_trie.longest_prefix(n.iri, true);
    if let Some((node, _)) = res {
        if node.value.is_some() {
            return node.value.as_ref().unwrap().clone();
        }
    }
    panic!("Could not normalize named node {}", n);
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

pub fn print_normalized_triples(nts: &TripleFreq) {
    let base_path = "output";
    let ext = "ttl";
    let mut path = format!("{}.{}", base_path, ext);
    let mut file_path = Path::new(&path);

    let mut copy_count = 1;
    while file_path.exists() {
        copy_count += 1;
        path = format!("{}-{}.{}", base_path, copy_count, ext);
        file_path = Path::new(&path);
    }
    info!("Saving graph summary to {}", file_path.display());

    let mut id_count = 0;

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
                subject: Subject::NamedNode(NamedNode { iri: &t_id }),
                predicate: NamedNode { iri: "a".clone() },
                object: Term::NamedNode(NamedNode {
                    iri: format!("{rdf}Statement").as_str(),
                }),
            })
            .unwrap();

        id_count += 1;
        formatter
            .format(&Triple {
                subject: Subject::NamedNode(NamedNode { iri: &t_id }),
                predicate: NamedNode {
                    iri: format!("{rdf}subject").as_str(),
                },
                object: Term::NamedNode(NamedNode {
                    iri: format!("{base_url}{}", tf.0).as_str(),
                }),
            })
            .unwrap();
        formatter
            .format(&Triple {
                subject: Subject::NamedNode(NamedNode { iri: &t_id }),
                predicate: NamedNode {
                    iri: format!("{rdf}predicate").as_str(),
                },
                object: Term::NamedNode(NamedNode {
                    iri: format!("{base_url}{}", tf.1).as_str(),
                }),
            })
            .unwrap();
        formatter
            .format(&Triple {
                subject: Subject::NamedNode(NamedNode { iri: &t_id }),
                predicate: NamedNode {
                    iri: format!("{rdf}object").as_str(),
                },
                object: Term::NamedNode(NamedNode {
                    iri: format!("{base_url}{}", tf.2).as_str(),
                }),
            })
            .unwrap();
        formatter
            .format(&Triple {
                subject: Subject::NamedNode(NamedNode { iri: &t_id }),
                predicate: NamedNode {
                    iri: format!("{base_url}occurrences").as_str(),
                },
                object: Term::Literal(Literal::Simple {
                    value: tf.3.to_string().as_str(),
                }),
            })
            .unwrap();

        id_count += 1;
    }
    formatter.finish().unwrap();
}
