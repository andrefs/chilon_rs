use crate::parse::parse;
use rayon::{ThreadPool, ThreadPoolBuilder};
use rio_api::model::{BlankNode, Literal, NamedNode, Subject, Term};
use rio_turtle::TurtleError;

use crate::ns_trie::NamespaceTrie;
use std::{
    collections::BTreeMap,
    ops::Add,
    path::PathBuf,
    sync::mpsc::{channel, Sender},
};

use rio_api::parser::TriplesParser;

type TripleFreq = BTreeMap<String, TripleFreqSec>;
type TripleFreqSec = BTreeMap<String, TripleFreqThird>;
type TripleFreqThird = BTreeMap<String, i32>;

trait TripleFreqFns {
    fn add(&mut self, triple: (String, String, String));
}

impl TripleFreqFns for TripleFreq {
    fn add(&mut self, triple: (String, String, String)) {
        todo!()
        //self.entry(triple.0)
        //    .or_default()
        //    .entry(triple.1)
        //    .or_default()
        //    .entry(triple.2)
        //    .or_default()
        //    .add(1);
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
    todo!()
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
