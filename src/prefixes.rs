pub mod prefixcc;

use crate::iri_trie::{update_desc_stats, IriTrie, IriTrieExt, NodeStats, TriplePos};
use crate::ns_trie::NamespaceTrie;
use crate::parse::parse;
use log::{debug, warn};
use rio_api::model::{NamedNode, Subject, Term};
use rio_turtle::TurtleError;
use std::collections::BTreeMap;
use std::{
    path::PathBuf,
    sync::mpsc::{channel, Sender},
};

use rio_api::parser::TriplesParser;

pub enum Message {
    Resource { iri: String, position: TriplePos },
    PrefixDecl { namespace: String, alias: String },
    Finished,
}

pub fn build_iri_trie(paths: Vec<PathBuf>, ns_trie: &mut NamespaceTrie) -> IriTrie {
    debug!("Building IRI trie");
    let n_workers = std::cmp::min(paths.len(), num_cpus::get() - 2);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(n_workers)
        .build()
        .unwrap();
    let mut running = paths.len();
    let (tx, rx) = channel::<Message>();

    for path in paths {
        spawn(&pool, &tx, path);
    }
    let mut iri_trie = IriTrie::new();
    let mut local_ns = BTreeMap::<String, String>::new();

    let mut i = 0;
    loop {
        i += 1;
        if i % 1_000_000 == 0 {
            debug!("Read {i} resources so far")
        }
        if running == 0 {
            break;
        }
        if let Ok(message) = rx.recv() {
            match message {
                Message::Resource { iri, position } => {
                    let res = ns_trie.longest_prefix(iri.as_str(), true);
                    if res.is_none() || res.unwrap().1.is_empty() {
                        let stats = NodeStats::new_terminal(position);
                        iri_trie.insert_fn(&iri, stats, Some(&update_desc_stats));
                    }
                }
                Message::PrefixDecl { namespace, alias } => {
                    debug!("Found local prefix {alias}: {namespace}");
                    local_ns.insert(namespace, alias);
                }
                Message::Finished => {
                    running -= 1;
                }
            }
        }
    }

    // local file prefix decls are only sent in the end
    // remove the prefix and add to other prefix trie

    iri_trie.remove_known_prefixes(&local_ns.iter().map(|(ns, _)| ns.clone()).collect());
    for (namespace, alias) in local_ns.iter() {
        ns_trie.insert(&namespace.clone(), alias.clone());
    }

    return iri_trie;
}

fn spawn(pool: &rayon::ThreadPool, tx: &Sender<Message>, path: PathBuf) {
    let tx = tx.clone();

    pool.spawn(move || {
        let mut graph = parse(&path);
        print!("\r");
        debug!("parsing {:?}", path);
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

pub fn infer_namespaces(iri_trie: &IriTrie) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for ns in iri_trie.iter_leaves() {
        res.push(ns.0);
    }

    return res;
}
