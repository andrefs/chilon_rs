pub mod community;

use crate::iri_trie::{inc_own, update_stats, IriTrie, IriTrieExt, NodeStats};
use crate::ns_trie::{gen_alias, NamespaceTrie};
use crate::parse::{parse, ParserWrapper};
use crate::seg_tree::SegTree;
use crate::trie::InsertFnVisitors;
use log::{debug, info, trace, warn};
use rio_api::model::{NamedNode, Subject, Term, Triple};
use rio_turtle::TurtleError;
use std::collections::BTreeMap;
use std::sync::mpsc::SyncSender;
use std::time::Instant;
use std::{path::PathBuf, sync::mpsc::sync_channel};
use unicode_segmentation::UnicodeSegmentation;
use url::Url;

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

    let mut iri_trie = IriTrie::new();
    let mut local_ns = BTreeMap::<String, String>::new();

    pool.scope_fifo(|s| {
        let (tx, rx) = sync_channel::<Message>(100);
        for path in paths {
            let tx = tx.clone();
            s.spawn_fifo(move |_| {
                debug!("Parsing {:?}", path);
                let mut graph = parse(&path);
                proc_triples(&mut graph, &path, &tx);
            });
        }


    let mut i = 0;
    let mut start = Instant::now();
let mut last_i = 0;

    loop {
        if running == 0 {
            break;
        }
        if let Ok(message) = rx.recv() {
            match message {
                Message::Resource { iri } => {
                    i += 1;
                    if i % 1_000_000 == 1 {
                        let elapsed = start.elapsed().as_millis();
                        if elapsed != 0 {
                            trace!(
                                "Received {i} resources so far ({} resources/s, trie size: {}, ns_trie size: {}, total seconds elapsed: {}s)",
                                ((i - last_i) / elapsed) * 1000,
                            iri_trie.count(),
                            ns_trie.count_terminals(),
                            start.elapsed().as_secs()
                            );
                        }

                        if let Some(size) = iri_trie.value {
                            let IRI_TRIE_SIZE = 1_000_000;
                            if size.desc > IRI_TRIE_SIZE {
                                warn!("IRI trie size over {IRI_TRIE_SIZE}, inferring namespaces");
                                let seg_tree = SegTree::from(&iri_trie);
                                let (inferred, gbg_collected) = seg_tree.infer_namespaces();

                                debug!("Adding inferred namespaces");
                                let added = ns_trie.add_inferred_namespaces(&inferred);

                                debug!("Removing {} IRIs with inferred namespaces", added.len());
                                iri_trie.remove_prefixes(&added);

                                debug!("Removing {} IRIs with garbage collected namespaces", gbg_collected.len());
                                iri_trie.remove_prefixes(&gbg_collected);
                            }
                        }
                                    last_i = i;
                        start = Instant::now();
                    }

                    let res = ns_trie.longest_prefix(iri.as_str(), true);
                    if res.is_none() || res.unwrap().1.is_empty() {
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
    });

    // message with local file prefix decls is only sent in the end
    // remove the prefix from iri trie and add to namespace trie
    iri_trie.remove_prefixes(&local_ns.iter().map(|(ns, _)| ns.clone()).collect());

    let ns_map = ns_trie.to_map();
    for (namespace, alias) in local_ns.iter() {
        let mut new_alias = alias.to_string();
        if new_alias.is_empty() {
            let url_obj = Url::parse(namespace.as_str());
            if url_obj.is_ok() {
                let alias_cand = gen_alias(url_obj.unwrap(), &ns_map);
                if alias_cand.is_some() {
                    new_alias = alias_cand.clone().unwrap();
                }
            }
        }
        if !new_alias.is_empty() {
            ns_trie.insert(&namespace.clone(), new_alias.clone());
        }
    }

    return iri_trie;
}

fn proc_triples(graph: &mut ParserWrapper, path: &PathBuf, tx: &SyncSender<Message>) {
    let tx = tx.clone();

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
            .parse_step(&mut |t| proc_triple(t, &tx))
            .unwrap_or_else(|err| {
                panic!("Error processing file {}: {}", path.to_string_lossy(), err)
            });
    }

    for (alias, namespace) in graph.prefixes().iter() {
        tx.send(Message::PrefixDecl {
            namespace: namespace.to_string(),
            alias: alias.to_string(),
        })
        .unwrap()
    }
    tx.send(Message::Finished).unwrap();
}

fn proc_triple(t: Triple, tx: &SyncSender<Message>) -> Result<(), TurtleError> {
    // subject
    if let Subject::NamedNode(NamedNode { iri }) = t.subject {
        tx.send(Message::Resource {
            iri: normalize_iri(iri),
        })
        .unwrap();
    }
    // predicate
    tx.send(Message::Resource {
        iri: normalize_iri(t.predicate.iri),
    })
    .unwrap();
    // object
    if let Term::NamedNode(NamedNode { iri }) = t.object {
        tx.send(Message::Resource {
            iri: normalize_iri(iri),
        })
        .unwrap();
    }

    Ok(()) as Result<(), TurtleError>
}

// TODO: improve IRI normalization
fn normalize_iri(iri: &str) -> String {
    let IRI_MAX_LENGTH = 200;

    if iri.len() > IRI_MAX_LENGTH {
        UnicodeSegmentation::graphemes(iri, true)
            .map(|x| x.to_string())
            .take(IRI_MAX_LENGTH)
            .collect::<Vec<String>>()
            .join("")
    } else {
        iri.to_string()
    }
}
