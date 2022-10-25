use clap::Parser;
use rio_api::parser::TriplesParser;
use rio_turtle::TurtleError;
use std::{
    path::PathBuf,
    sync::mpsc::{channel, Sender},
};
use threadpool::ThreadPool;
use trie_generic::Trie;
mod args;
use args::Cli;
mod extract;
mod parse;
use parse::parse;
use rio_api::model::{NamedNode, Subject, Term};

pub enum Message {
    Resource { iri: String },
    PrefixDecl { namespace: String, alias: String },
    Finished,
}

fn main() {
    let cli = Cli::parse();

    let mut pref_trie = trie_generic::Trie::<String>::new(None);
    let mut iri_trie = build_iri_trie(cli.files, &mut pref_trie);

    println!("\nResource Trie\n{}", iri_trie.pp(false));
    println!("\nPrefix Trie\n{}", pref_trie.pp(true));
}

fn build_iri_trie(paths: Vec<PathBuf>, pref_trie: &mut Trie<String>) -> Trie<i32> {
    let n_workers = std::cmp::min(paths.len(), num_cpus::get() - 2);
    let pool: ThreadPool = ThreadPool::new(n_workers);
    let mut running = paths.len();
    let (tx, rx) = channel::<Message>();
    for path in paths {
        spawn(&pool, &tx, path);
    }
    let mut iri_trie = trie_generic::Trie::<i32>::new(None);

    loop {
        if running == 0 {
            break;
        }
        if let Ok(message) = rx.recv() {
            match message {
                Message::Resource { iri } => {
                    iri_trie.add(&iri, Some(1));
                }
                Message::PrefixDecl { namespace, alias } => {
                    pref_trie.add(&namespace, Some(alias.to_owned()));
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
        println!("PREFIXES : {:?}", graph.prefixes());
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
