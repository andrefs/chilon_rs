use chilon_rs::Node;
use clap::Parser;
use rio_api::parser::TriplesParser;
use rio_turtle::TurtleError;
use std::{
    collections::HashMap,
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
    Resource { iri: String },
    PrefixDecl { namespace: String, alias: String },
    Finished,
}

fn main() {
    let cli = Cli::parse();

    let mut ns_map = prefixcc::load();
    // // TODO: add more mappings to ns_map  from user supplied rdf file with flag -p
    let mut iri_trie = build_iri_trie(cli.files, &mut ns_map);
    println!("\nNamespace Map\n{:#?}", ns_map.len());
    println!("\nIri Trie nodes\n{}", iri_trie.count_nodes());
    println!("\nIri Trie terminals\n{}", iri_trie.count_terminals());
    println!("\nremoving\n");
    remove_known_prefixes(&ns_map, &mut iri_trie);
    //
    //println!("\nNamespace Map\n{:#?}", ns_map);
    println!("\nIri Trie nodes\n{}", iri_trie.count_nodes());
    println!("\nIri Trie terminals\n{}", iri_trie.count_terminals());
}

fn remove_known_prefixes(ns_map: &PrefixMap, iri_trie: &mut IriTrie) {
    for (_, namespace) in ns_map.iter() {
        iri_trie.remove(namespace, true);
    }
}

pub type IriTrie = Node<u32>; // todo finish

fn build_iri_trie(paths: Vec<PathBuf>, ns_map: &mut PrefixMap) -> IriTrie {
    let n_workers = std::cmp::min(paths.len(), num_cpus::get() - 2);
    let pool: ThreadPool = ThreadPool::new(n_workers);
    let mut running = paths.len();
    let (tx, rx) = channel::<Message>();

    for path in paths {
        spawn(&pool, &tx, path);
    }
    let mut iri_trie = Node::new();

    loop {
        if running == 0 {
            break;
        }
        if let Ok(message) = rx.recv() {
            match message {
                Message::Resource { iri } => {
                    iri_trie.insert(&iri, 1);
                }
                Message::PrefixDecl { namespace, alias } => {
                    // ns_map.push((alias.to_owned(), namespace));
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
                    })
                    .unwrap();
                }
                tx.send(Message::Resource {
                    iri: t.predicate.iri.to_owned(),
                })
                .unwrap();
                if let Term::NamedNode(NamedNode { iri }) = t.object {
                    tx.send(Message::Resource {
                        iri: iri.to_owned(),
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
