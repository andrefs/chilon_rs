use crate::parse::parse;
use rio_api::model::{BlankNode, Literal, NamedNode, Subject, Term};
use rio_turtle::TurtleError;
use threadpool::ThreadPool;

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
        self.entry(triple.0)
            .or_default()
            .entry(triple.1)
            .or_default()
            .entry(triple.2)
            .or_default()
            .add(1);
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

pub fn normalize_triples(paths: Vec<PathBuf>, ns_trie: &mut NamespaceTrie) -> TripleFreq {
    let n_workers = std::cmp::min(paths.len(), num_cpus::get() - 2);
    let pool: ThreadPool = ThreadPool::new(n_workers);
    let mut running = paths.len();
    let (tx, rx) = channel::<Message>();

    for path in paths {
        spawn(&pool, &tx, path);
    }

    let mut triples = TripleFreq::new();

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

fn spawn(pool: &ThreadPool, tx: &Sender<Message>, path: PathBuf) {
    let tx = tx.clone();

    pool.execute(move || {
        let mut graph = parse(&path);
        graph
            .parse_all(&mut |t| {
                let mut subject = handle_subject(t.subject);
                let mut predicate = handle_predicate(t.predicate);
                let mut object = handle_object(t.object);

                if let Term::Literal(lit) = t.object {
                    object = handle_literal(lit);
                }

                tx.send(Message::NormalizedTriple {
                    subject: subject.to_string(),
                    predicate,
                    object: object.to_string(),
                });
                Ok(()) as Result<(), TurtleError>
            })
            .unwrap();
        tx.send(Message::Finished).unwrap();
    });
}

fn handle_subject(sub: Subject) -> String {
    match sub {
        Subject::BlankNode(c) => "[BLANK]".to_string(),
        Subject::Triple(triple) => unimplemented!(),
        Subject::NamedNode(n) => handle_named_node(n),
    }
}

fn handle_predicate(pred: NamedNode) -> String {
    handle_named_node(pred)
}

fn handle_object(obj: Term) -> String {
    match obj {
        Term::BlankNode(c) => "[BLANK]".to_string(),
        Term::Triple(triple) => unimplemented!(),
        Term::NamedNode(n) => handle_named_node(n),
        Term::Literal(lit) => handle_literal(lit),
    }
}

fn handle_named_node(n: NamedNode) -> String {
    todo!()
}

fn handle_literal(lit: Literal) -> String {
    match lit {
        Literal::Simple { value } => "[LITERAL:string]".to_string(),
        Literal::LanguageTaggedString { value, language } => "[LITERAL:lang-string]".to_string(),
        Literal::Typed { value, datatype } => "[LITERAL:datatype]".to_string(),
    }
}
