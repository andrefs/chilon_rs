use crate::chitrie::{dec_stats, inc_stats, IriTrie, NodeStats, TriplePos};
use crate::parse::parse;
use crate::trie::TraverseFns;
use rio_api::model::{NamedNode, Subject, Term};
use rio_turtle::TurtleError;
use threadpool::ThreadPool;

use self::prefixcc::PrefixMap;
use std::{
    path::PathBuf,
    sync::mpsc::{channel, Sender},
};

use rio_api::parser::TriplesParser;

pub mod prefixcc;

pub enum Message {
    Resource { iri: String, position: TriplePos },
    PrefixDecl { namespace: String, alias: String },
    Finished,
}

pub trait IriTrieExt {
    fn remove_leaves(&mut self) -> bool;
    fn remove_leaves_aux(&mut self, cur_str: String) -> bool;
    fn remove_known_prefixes(&mut self, ns_map: &PrefixMap);
}

impl IriTrieExt for IriTrie {
    fn remove_leaves(&mut self) -> bool {
        self.remove_leaves_aux("".to_string())
    }
    fn remove_leaves_aux(&mut self, cur_str: String) -> bool {
        if self.children.is_empty() {
            return false;
        }
        let mut deleted = false;
        let mut to_remove = Vec::<char>::new();

        for (&ch, node) in self.children.iter_mut() {
            let child_deleted = node.remove_leaves_aux(format!("{}{}", cur_str, ch));
            if !child_deleted && ['/', '#'].contains(&ch) {
                to_remove.push(ch);
                deleted = true;
            }
            deleted = deleted || child_deleted;
        }
        for ch in to_remove.iter() {
            self.children.remove(ch);
        }
        return deleted;
    }

    fn remove_known_prefixes(&mut self, ns_map: &PrefixMap) {
        for (_, namespace) in ns_map.iter() {
            self.remove_fn(namespace, true, Some(&dec_stats));
        }
    }
}

pub fn build_iri_trie(paths: Vec<PathBuf>, ns_map: &mut PrefixMap) -> IriTrie {
    let n_workers = std::cmp::min(paths.len(), num_cpus::get() - 2);
    let pool: ThreadPool = ThreadPool::new(n_workers);
    let mut running = paths.len();
    let (tx, rx) = channel::<Message>();

    for path in paths {
        spawn(&pool, &tx, path);
    }
    let mut iri_trie = IriTrie::new();

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
                        TraverseFns {
                            branch: Some(&inc_stats(position)),
                            terminal: None,
                        },
                    );
                    //iri_trie.insert(&iri, stats);
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
