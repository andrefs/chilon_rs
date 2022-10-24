//extern crate bzip2;

use bzip2::bufread::BzDecoder;
use clap::Parser;
use rio_api::parser::TriplesParser;
use rio_turtle::{TurtleError, TurtleParser};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Write;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
mod args;
use args::Cli;
use rio_api::model::{NamedNode, Subject, Term};

pub enum Message {
    Resource { iri: String },
    Finished,
}

fn main() {
    let cli = Cli::parse();

    let n_workers = std::cmp::min(cli.files.len(), num_cpus::get() - 2);
    let pool = ThreadPool::new(n_workers);
    let (tx, rx) = channel::<Message>();

    for path in cli.files.to_vec() {
        let tx = tx.clone();
        pool.execute(move || {
            let file = File::open(&path).unwrap();
            let buf_reader = BufReader::new(file);
            let dec = BzDecoder::new(buf_reader);
            let stream = BufReader::new(dec);

            let mut count = 0;

            TurtleParser::new(stream, None)
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

                    count += 1;
                    if count % 10000 == 0 {
                        eprint!(".");
                        io::stdout().flush().unwrap();
                    }
                    Ok(()) as Result<(), TurtleError>
                })
                .unwrap();
            eprintln!("\nThread for {:?} finished", path);
            tx.send(Message::Finished).unwrap();
        })
    }
    let mut running = cli.files.len();
    let mut t = trie_generic::Trie::<i32>::new(None);

    loop {
        if running == 0 {
            break;
        }
        if let Ok(message) = rx.recv() {
            match message {
                Message::Resource { iri } => t.add(&iri, Some(1)),
                Message::Finished => {
                    running -= 1;
                }
            }
        }
    }

    eprintln!("\nTrie\n{}", t.pp());
}

//fn main() {
//    //let file = File::open("../wn.ttl.bz2").unwrap();
//    let file = File::open("../infobox-properties_lang_en.ttl.bz2").unwrap();
//    let buf_reader = BufReader::new(file);
//    let dec = BzDecoder::new(buf_reader);
//    let stream = BufReader::new(dec);
//    //stream.read_to_end(&mut buffer).unwrap();
//    //println!("{}", String::from_utf8(buffer).unwrap());
//
//    let mut count = 0;
//    TurtleParser::new(stream, None)
//        .parse_all(&mut |_| {
//            if count % 10000 == 0 {
//                print!(".");
//                io::stdout().flush().unwrap();
//            }
//            count += 1;
//            Ok(()) as Result<(), TurtleError>
//        })
//        .unwrap();
//    println!("\nbench_rio_turtle {count}");
//}
