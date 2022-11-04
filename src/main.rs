use chilon_rs::{trie::InsertFns, Node};
use clap::Parser;
use rio_api::parser::TriplesParser;
use rio_turtle::TurtleError;
use std::{
    path::PathBuf,
    sync::mpsc::{channel, Sender},
};
use threadpool::ThreadPool;
mod args;
use args::Cli;
mod extract;
mod parse;
use parse::parse;
use rio_api::model::{NamedNode, Subject, Term};
mod prefixes;
use prefixes::prefixcc::{self, PrefixMap};

pub enum TriplePos {
    S,
    P,
    O,
}

pub enum Message {
    Resource { iri: String, position: TriplePos },
    PrefixDecl { namespace: String, alias: String },
    Finished,
}

// Represents occurrences as subject, predicate or object
#[derive(Debug, Default)]
pub struct Stats {
    s: u32,
    p: u32,
    o: u32,
    total: u32,
}

// Each node keeps its own stats (if terminal) and its descendants stats
#[derive(Debug)]
pub struct NodeStats {
    own: Option<Stats>,
    desc: Stats,
}

impl NodeStats {
    fn new() -> NodeStats {
        NodeStats {
            own: None,
            desc: Default::default(),
        }
    }

    fn new_terminal(pos: TriplePos) -> NodeStats {
        NodeStats {
            own: match pos {
                TriplePos::S => Some(Stats {
                    s: 1,
                    ..Default::default()
                }),
                TriplePos::P => Some(Stats {
                    p: 1,
                    ..Default::default()
                }),
                TriplePos::O => Some(Stats {
                    o: 1,
                    ..Default::default()
                }),
            },
            desc: Default::default(),
        }
    }
}
impl Default for NodeStats {
    fn default() -> Self {
        NodeStats {
            own: None,
            desc: Default::default(),
        }
    }
}

impl Stats {
    fn inc(&mut self, pos: TriplePos) {
        match pos {
            TriplePos::S => self.inc_s(),
            TriplePos::P => self.inc_p(),
            TriplePos::O => self.inc_o(),
        }
    }
    fn inc_s(&mut self) {
        self.s += 1;
    }
    fn inc_p(&mut self) {
        self.p += 1;
    }
    fn inc_o(&mut self) {
        self.o += 1;
    }
}

fn main() {
    let cli = Cli::parse();

    let mut ns_map = prefixcc::load();
    // // TODO: add more mappings to ns_map  from user supplied rdf file with flag -p
    let mut iri_trie = build_iri_trie(cli.files, &mut ns_map);
    println!("Total IRIs: {}", iri_trie.count_terminals());
    remove_known_prefixes(&ns_map, &mut iri_trie);
    println!("Unmatched IRIs: {}", iri_trie.count_terminals());

    iri_trie.traverse(&|key, value| println!("{key} [{:#?}]", value))
}

fn remove_known_prefixes(ns_map: &PrefixMap, iri_trie: &mut IriTrie) {
    for (_, namespace) in ns_map.iter() {
        iri_trie.remove(namespace, true);
    }
}

pub type IriTrie = Node<NodeStats>; // todo finish

fn add_stats(n: &mut Node<NodeStats>) {
    let mut new_stats = NodeStats::new();
    n.value = Some(new_stats);
}

fn build_iri_trie(paths: Vec<PathBuf>, ns_map: &mut PrefixMap) -> IriTrie {
    let n_workers = std::cmp::min(paths.len(), num_cpus::get() - 2);
    let pool: ThreadPool = ThreadPool::new(n_workers);
    let mut running = paths.len();
    let (tx, rx) = channel::<Message>();

    for path in paths {
        spawn(&pool, &tx, path);
    }
    let mut iri_trie = Node::<NodeStats>::new();

    loop {
        if running == 0 {
            break;
        }
        if let Ok(message) = rx.recv() {
            match message {
                Message::Resource { iri, position } => {
                    let stats = NodeStats::new_terminal(position);
                    iri_trie.insert_fn(
                        &iri,
                        stats,
                        InsertFns {
                            branch: Some(&add_stats),
                            terminal: None,
                        },
                    );
                }
                Message::PrefixDecl { namespace, alias } => {
                    ns_map.insert(alias.to_owned(), namespace);
                }
                Message::Finished => {
                    running -= 1;
                }
            }
        }
    }

    return iri_trie;
}

fn spawn(pool: &ThreadPool, tx: &Sender<Message>, path: PathBuf) {
    let tx = tx.clone();

    pool.execute(move || {
        let mut graph = parse(&path);
        graph
            .parse_all(&mut |t| {
                if let Subject::NamedNode(NamedNode { iri }) = t.subject {
                    tx.send(Message::Resource {
                        iri: iri.to_owned(),
                        position: TriplePos::S,
                    })
                    .unwrap();
                }
                tx.send(Message::Resource {
                    iri: t.predicate.iri.to_owned(),
                    position: TriplePos::P,
                })
                .unwrap();
                if let Term::NamedNode(NamedNode { iri }) = t.object {
                    tx.send(Message::Resource {
                        iri: iri.to_owned(),
                        position: TriplePos::O,
                    })
                    .unwrap();
                }

                Ok(()) as Result<(), TurtleError>
            })
            .unwrap();
        //println!("PREFIXES : {:?}", graph.prefixes());
        for (alias, namespace) in graph.prefixes().iter() {
            tx.send(Message::PrefixDecl {
                namespace: namespace.to_string(),
                alias: alias.to_string(),
            })
            .unwrap()
        }
        tx.send(Message::Finished).unwrap();
    });
}
