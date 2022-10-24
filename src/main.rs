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

pub enum Message {
    Resource { iri: String },
    PrefixDecl { namespace: String, alias: String },
    Finished,
}

fn main() {
    let cli = Cli::parse();

    let n_workers = std::cmp::min(cli.files.len(), num_cpus::get() - 2);
    let pool: ThreadPool = ThreadPool::new(n_workers);
    let (tx, rx) = channel::<Message>();

    for path in cli.files.to_vec() {
        spawn(&pool, &tx, path);
    }
    let mut running = cli.files.len();
    let mut res_trie = trie_generic::Trie::<i32>::new(None);
    let mut pref_trie = trie_generic::Trie::<String>::new(None);

    loop {
        if running == 0 {
            break;
        }
        if let Ok(message) = rx.recv() {
            match message {
                Message::Resource { iri } => res_trie.add(&iri, Some(1)),
                Message::PrefixDecl { namespace, alias } => {
                    pref_trie.add(&namespace, Some(alias.to_owned()));
                }
                Message::Finished => {
                    running -= 1;
                }
            }
        }
    }

    println!("\nResource Trie\n{}", res_trie.pp(false));
    println!("\nPrefix Trie\n{}", pref_trie.pp(true));
}

fn spawn(pool: &ThreadPool, tx: &Sender<Message>, path: PathBuf) {
    let tx = tx.clone();
    pool.execute(move || {
        let mut parser = parse(&path);
        parser
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
        eprintln!("\nThread for {:?} finished", path);
        println!("PREFIXES : {:?}", parser.prefixes());
        for (alias, namespace) in parser.prefixes().iter() {
            tx.send(Message::PrefixDecl {
                namespace: namespace.to_string(),
                alias: alias.to_string(),
            })
            .unwrap()
        }
        tx.send(Message::Finished).unwrap();
    });
}
