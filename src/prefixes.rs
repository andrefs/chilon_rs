pub mod prefixcc;

use crate::iri_trie::{inc_own, update_stats, IriTrie, IriTrieExt, NodeStats};
use crate::ns_trie::NamespaceTrie;
use crate::parse::parse;
use crate::seg_tree::SegTree;
use crate::trie::InsertFnVisitors;
use log::{debug, error, info, trace, warn};
use rio_api::formatter::TriplesFormatter;
use rio_api::model::{NamedNode, Subject, Term, Triple};
use rio_turtle::TurtleError;
use std::collections::BTreeMap;
use std::time::{Duration, Instant};
use std::{
    path::PathBuf,
    sync::mpsc::{channel, Sender},
};

use crate::ns_trie::InferredNamespaces;
use rio_api::parser::TriplesParser;

pub enum Message {
    Resource { iri: String },
    PrefixDecl { namespace: String, alias: String },
    Finished,
}

pub fn build_iri_trie(paths: Vec<PathBuf>, ns_trie: &mut NamespaceTrie) -> IriTrie {
    debug!("Building IRI trie");
    let n_workers = std::cmp::max(2, std::cmp::min(paths.len(), num_cpus::get() - 2));
    info!("Creating pool with {n_workers} threads");
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(n_workers)
        .build()
        .unwrap();
    let mut running = paths.len();
    let (tx, rx) = channel::<Message>();

    for path in paths {
        spawn(&pool, &tx, path, ns_trie.to_owned());
    }
    let mut iri_trie = IriTrie::new();
    let mut local_ns = BTreeMap::<String, String>::new();

    let mut i = 0;
    let mut start = Instant::now();
    loop {
        if running == 0 {
            break;
        }
        if let Ok(message) = rx.recv() {
            match message {
                Message::Resource { iri } => {
                    i += 1;
                    if i % 1_000_000 == 1 {
                        trace!(
                            "Received {i} resources so far (total seconds elapsed: {}s)",
                            start.elapsed().as_secs()
                        );

                        if let Some(size) = iri_trie.value {
                            let IRI_TRIE_SIZE = 5_000_000;
                            if size.desc > IRI_TRIE_SIZE {
                                warn!("IRI trie size over {IRI_TRIE_SIZE}, inferring namespaces");
                                let seg_tree = SegTree::from(&iri_trie);
                                let inferred = seg_tree.infer_namespaces();

                                debug!("Adding inferred namespaces");
                                let added = ns_trie.add_inferred_namespaces(&inferred);

                                debug!("Removing IRIs with inferred namespaces");
                                iri_trie.remove_known_prefixes(&added);
                            }
                        }
                    }

                    let stats = NodeStats::new_terminal();
                    iri_trie.insert_fn(
                        &iri,
                        stats,
                        &InsertFnVisitors {
                            node: Some(&update_stats),
                            terminal: Some(&inc_own),
                        },
                    );
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

fn spawn(pool: &rayon::ThreadPool, tx: &Sender<Message>, path: PathBuf, ns_trie: NamespaceTrie) {
    let tx = tx.clone();

    pool.spawn_fifo(move || {
        let mut graph = parse(&path);
        debug!("Parsing {:?}", path);
        let tid = if let Some(id) = rayon::current_thread_index() {
            id.to_string()
        } else {
            "".to_string()
        };
        let mut i = 0;
        let mut start = Instant::now();
        let mut last_i = 0;

        while !graph.is_end() {
            i += 1;
            if i % 1_000_000 == 1 {
                let elapsed = start.elapsed().as_millis();
                if elapsed != 0 {
                    trace!(
                        "[Thread#{tid}] Parsed {i} triples so far ({} triples/s)",
                        ((i - last_i) / elapsed) * 1000
                    );
                }
                last_i = i;
                start = Instant::now();
            }

            graph
                .parse_step(&mut |t| proc_triple(t, &tx, &ns_trie))
                .unwrap_or_else(|err| error!("Error processing triple {}", err));
        }

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

fn proc_triple(
    t: Triple,
    tx: &Sender<Message>,
    ns_trie: &NamespaceTrie,
) -> Result<(), TurtleError> {
    // subject
    if let Subject::NamedNode(NamedNode { iri }) = t.subject {
        let res = ns_trie.longest_prefix(iri, true);
        if res.is_none() || res.unwrap().1.is_empty() {
            tx.send(Message::Resource {
                iri: iri.to_owned(),
            })
            .unwrap();
        }
    }
    // predicate
    let res = ns_trie.longest_prefix(t.predicate.iri, true);
    if res.is_none() || res.unwrap().1.is_empty() {
        tx.send(Message::Resource {
            iri: t.predicate.iri.to_owned(),
        })
        .unwrap();
    }
    // object
    if let Term::NamedNode(NamedNode { iri }) = t.object {
        let res = ns_trie.longest_prefix(iri, true);
        if res.is_none() || res.unwrap().1.is_empty() {
            tx.send(Message::Resource {
                iri: iri.to_owned(),
            })
            .unwrap();
        }
    }

    Ok(()) as Result<(), TurtleError>
}
