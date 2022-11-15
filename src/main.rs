use chilon_rs::{
    chitrie::{NodeStats, TriplePos},
    trie::TraverseFns,
    Node,
};
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

pub enum Message {
    Resource { iri: String, position: TriplePos },
    PrefixDecl { namespace: String, alias: String },
    Finished,
}

fn main() {
    let cli = Cli::parse();

    let mut ns_map = prefixcc::load();
    // // TODO: add more mappings to ns_map  from user supplied rdf file with flag -p
    let mut iri_trie = build_iri_trie(cli.files, &mut ns_map);
    //println!("Total IRIs: {}", iri_trie.count_terminals());
    ////remove_known_prefixes(&ns_map, &mut iri_trie);
    //println!("Unmatched IRIs: {}", iri_trie.count_terminals());

    //println!("\n\n\ndown");
    //iri_trie.traverse(&|key, value| println!("{key}"));

    ////println!("Remove leaves 1: {}", remove_leaves(&mut iri_trie));
    ////println!("Remove leaves 2: {}", remove_leaves(&mut iri_trie));
    ////println!("Remove leaves 3: {}", remove_leaves(&mut iri_trie));

    //println!("\n\n\nup");
    //iri_trie.traverse_up(&|key, value| println!("{key}"));
    println!("{:#?}", iri_trie);
    println!("{}", iri_trie.pp(true));
}

fn remove_leaves(iri_trie: &mut IriTrie) -> bool {
    remove_leaves_aux(iri_trie, "".to_string())
}
fn remove_leaves_aux(iri_trie: &mut IriTrie, cur_str: String) -> bool {
    if iri_trie.children.is_empty() {
        return false;
    }
    let mut deleted = false;
    let mut to_remove = Vec::<char>::new();

    for (&ch, mut node) in iri_trie.children.iter_mut() {
        let child_deleted = remove_leaves_aux(&mut node, format!("{}{}", cur_str, ch));
        if !child_deleted && ['/', '#'].contains(&ch) {
            to_remove.push(ch);
            deleted = true;
        }
        deleted = deleted || child_deleted;
    }
    for ch in to_remove.iter() {
        iri_trie.children.remove(ch);
    }
    return deleted;
}

fn remove_known_prefixes(ns_map: &PrefixMap, iri_trie: &mut IriTrie) {
    for (_, namespace) in ns_map.iter() {
        iri_trie.remove(namespace, true);
    }
}

pub type IriTrie = Node<NodeStats>; // todo finish

fn init_stats(n: &mut IriTrie) {
    let new_stats = NodeStats::new();
    n.value = Some(new_stats);
}
fn inc_stats(position: TriplePos) -> impl Fn(&mut Node<NodeStats>) -> () {
    move |n: &mut IriTrie| {
        let mut new_stats = NodeStats::new();
        if n.value.is_none() {
            n.value = Some(new_stats);
        }
        n.value.as_mut().unwrap().desc.inc(position)
    }
}
fn dec_stats(n: &mut IriTrie) -> impl Fn(&mut Node<NodeStats>) -> () {
    move |n: &mut IriTrie| {
        let mut new_stats = NodeStats::new();
        if n.value.is_none() {
            n.value = Some(new_stats);
        }
        n.value.as_mut().unwrap().desc.inc(position)
    }
}

fn build_iri_trie(paths: Vec<PathBuf>, ns_map: &mut PrefixMap) -> IriTrie {
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
