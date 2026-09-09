pub mod community;

use crate::error::ChilonError;
use crate::meta_info::{InferHK, InferHKTask, Task, TaskType};
use crate::ns_trie::{gen_alias, NamespaceSource, NamespaceTrie};
use crate::parse::{parse, ParserWrapper};
use crate::seg_tree::SegTree;
use crate::trie::{InsertFnVisitors, Node};
use crate::{
    counter::Counter,
    iri_trie::{inc_own, update_stats, IriTrie, IriTrieExt, NodeStats},
};
use log::{debug, error, info, trace};
use oxrdf::{NamedOrBlankNode, Term, Triple};
use std::collections::BTreeMap;
use std::fs::metadata;
use std::path::Path;
use std::sync::mpsc::{Receiver, SyncSender};
use std::time::Instant;
use std::{path::PathBuf, sync::mpsc::sync_channel};
use unicode_segmentation::UnicodeSegmentation;
use url::Url;

use crate::ns_trie::InferredNamespaces;

pub enum Position {
    Subject,
    Predicate,
    Object,
}

pub enum Message {
    Started {
        path: String,
    },
    Resource {
        iri: String,
        pos: Position,
    },
    PrefixDecl {
        namespace: String,
        alias: String,
    },
    Finished {
        path: String,
        triples: usize,
        iris: usize,
        blanks: usize,
        literals: usize,
    },
    FatalError {
        err: ChilonError,
    },
}

pub fn build_iri_trie(
    paths: Vec<PathBuf>,
    n_workers: usize,
    ns_trie: &mut NamespaceTrie,
    allow_subns: bool,
) -> (IriTrie, BTreeMap<String, Task>, InferHK) {
    debug!("Building IRI trie");

    if n_workers < 2 {
        panic!("Number of workers must be at least 2");
    }
    info!("Creating pool with {n_workers} threads");
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(n_workers)
        .build()
        .unwrap();
    let paths_len = paths.len();
    let mut running = paths_len;
    debug!("Processing {running} files: {paths:?}");

    let mut iri_trie = IriTrie::new();
    let mut local_ns = BTreeMap::<String, String>::new();

    let mut tasks = BTreeMap::<String, Task>::new();
    let mut hk = InferHK::new();

    pool.scope_fifo(|s| {
        let (tx, rx) = sync_channel::<Message>(100);
        for (index, path) in paths.iter().enumerate() {
            let tx = tx.clone();

            s.spawn_fifo(move |_| {
                tx.send(Message::Started {
                    path: path.to_string_lossy().to_string(),
                })
                .unwrap();

                info!("Parsing {:?} ({}/{running})", path, index + 1);
                let mut graph = match parse(path) {
                    Ok(g) => g,
                    Err(err) => {
                        tx.send(Message::FatalError { err }).unwrap();
                        return;
                    }
                };
                proc_triples(&mut graph, path, &tx);
            });
        }

        handle_loop(
            &mut running,
            rx,
            &mut iri_trie,
            ns_trie,
            &mut local_ns,
            &mut tasks,
            &mut hk,
            allow_subns,
        );
    });

    handle_pref_decls(&mut iri_trie, local_ns, ns_trie);

    (iri_trie, tasks, hk)
}

#[allow(clippy::too_many_arguments)]
fn handle_loop(
    running: &mut usize,
    rx: Receiver<Message>,
    iri_trie: &mut Node<NodeStats>,
    ns_trie: &mut NamespaceTrie,
    local_ns: &mut BTreeMap<String, String>,
    tasks: &mut BTreeMap<String, Task>,
    hk: &mut InferHK,
    allow_subns: bool,
) {
    let res_c = &mut Counter::default();
    let trip_c = &mut Counter::default();
    let start = &mut Instant::now();

    loop {
        if *running == 0 {
            info!("All threads finished");
            break;
        }
        if let Ok(message) = rx.recv() {
            match message {
                Message::Started { path } => {
                    let mut t = Task::new(path.clone(), TaskType::InferNamespaces);
                    t.size = metadata(path.clone()).unwrap().len() as usize;
                    tasks.insert(path, t);
                }
                Message::Resource { iri, pos } => {
                    if let Position::Predicate = pos {
                        trip_c.inc();
                    }
                    res_c.inc();

                    if res_c.cur % 1_000_000 == 1 {
                        let it_c = iri_trie.count();
                        let it_n = iri_trie.count_nodes();
                        let nst_ct = ns_trie.count_terminals();
                        restart_timers(start, res_c, trip_c, it_c, it_n, nst_ct);

                        let infer_hk = maintenance(iri_trie, ns_trie, allow_subns);
                        if let Some(infer_hk) = infer_hk {
                            hk.add(infer_hk);
                        }
                    }

                    insert_resource(ns_trie, iri, iri_trie);
                }
                Message::PrefixDecl { namespace, alias } => {
                    debug!("Found local prefix {alias}: {namespace}");
                    local_ns.insert(namespace, alias);
                }
                Message::Finished {
                    path,
                    triples,
                    iris,
                    blanks,
                    literals,
                } => {
                    let t = tasks.get_mut(&path).unwrap();
                    t.triples = triples;
                    t.blanks = blanks;
                    t.iris = iris;
                    t.literals = literals;

                    t.finish(format!("Finished task {:?} on {}", t.task_type, t.name).as_str());

                    *running -= 1;
                    trace!("Running: {running}");
                }
                Message::FatalError { err } => {
                    error!("Fatal error: {err}");
                    *running -= 1;
                }
            }
        }
    }
}

fn insert_resource(ns_trie: &NamespaceTrie, iri: String, iri_trie: &mut Node<NodeStats>) {
    // find namespace for resource
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

fn maintenance(
    iri_trie: &mut Node<NodeStats>,
    ns_trie: &mut NamespaceTrie,
    allow_subns: bool,
) -> Option<InferHKTask> {
    let mut res = None::<InferHKTask>;

    if let Some(size) = iri_trie.value {
        const IRI_TRIE_SIZE: usize = 1_000_000;

        if size.desc > IRI_TRIE_SIZE {
            let mut t = InferHKTask::new();

            info!("IRI trie size over {IRI_TRIE_SIZE}, inferring namespaces");
            let seg_tree = SegTree::from(&*iri_trie);
            let (inferred, gbg_collected) = seg_tree.infer_namespaces();
            t.inferred_ns = inferred.len();
            t.discarded_ns = gbg_collected.len();

            debug!("Adding inferred namespaces");
            let added = ns_trie.add_namespaces(&inferred, allow_subns);
            t.added_ns = added.len();

            debug!("Removing {} IRIs with inferred namespaces", added.len());
            iri_trie.remove_prefixes(&added);

            debug!(
                "Forgetting {} IRIs with low occurring namespaces",
                gbg_collected.len()
            );
            iri_trie.remove_prefixes(&gbg_collected);

            t.finish();
            res = Some(t);
        }
    }

    res
}

fn handle_pref_decls(
    iri_trie: &mut Node<NodeStats>,
    local_ns: BTreeMap<String, String>,
    ns_trie: &mut NamespaceTrie,
) {
    // message with local file prefix decls is only sent in the end
    // remove the prefix from iri trie and add to namespace trie
    iri_trie.remove_prefixes(&local_ns.keys().cloned().collect::<Vec<String>>());

    let ns_map = ns_trie.to_map();
    for (namespace, alias) in local_ns.iter() {
        let mut new_alias = alias.to_string();
        if new_alias.is_empty() {
            let url_obj = Url::parse(namespace.as_str());
            if let Ok(url_obj1) = url_obj {
                let alias_cand = gen_alias(url_obj1, &ns_map);
                if alias_cand.is_some() {
                    new_alias = alias_cand.clone().unwrap();
                }
            }
        }
        if !new_alias.is_empty() {
            ns_trie.insert(
                &namespace.clone(),
                (new_alias.clone(), NamespaceSource::GraphFile),
            );
        }
    }
}

fn restart_timers(
    start: &mut Instant,
    res_c: &mut Counter,
    trip_c: &mut Counter,
    iri_trie_count: usize,
    iri_trie_nodes: u32,
    ns_trie_count_t: u32,
) {
    let elapsed = start.elapsed().as_millis();
    let msg_rate = (res_c.delta() as u128)
        .checked_div(elapsed)
        .map(|r| r * 1000);
    let trip_rate = (trip_c.delta() as u128)
        .checked_div(elapsed)
        .map(|r| r * 1000);
    if msg_rate.is_some() || trip_rate.is_some() {
        trace!(
            "Received {} resources ({}/s), {} triples ({}/s) so far",
            res_c.cur,
            msg_rate.unwrap_or(0),
            trip_c.cur,
            trip_rate.unwrap_or(0),
        );
        trace!(
            "iri trie size: {} ({} nodes), ns_trie size: {}, total seconds elapsed: {}s)",
            iri_trie_count,
            iri_trie_nodes,
            ns_trie_count_t,
            start.elapsed().as_secs()
        );
    }

    res_c.lap();
    trip_c.lap();
    *start = Instant::now();
}

fn proc_triples(graph: &mut ParserWrapper, path: &Path, tx: &SyncSender<Message>) -> usize {
    let tx = tx.clone();

    let tid = if let Some(id) = rayon::current_thread_index() {
        id.to_string()
    } else {
        "".to_string()
    };
    let mut trip_c = 0;
    let mut start = Instant::now();
    let mut last_trip_c = 0;

    let mut blank_c = 0;
    let mut iri_c = 0;
    let mut literal_c = 0;

    for result in graph.by_ref() {
        trip_c += 1;
        if trip_c % 1_000_000 == 1 {
            let elapsed = start.elapsed().as_millis();
            let rate = ((trip_c - last_trip_c) as u128)
                .checked_div(elapsed)
                .map(|r| r * 1000);
            if rate.is_some() {
                trace!(
                    "[Thread#{tid}] Parsed {trip_c} triples so far ({} triples/s)",
                    rate.unwrap_or(0)
                );
            }
            last_trip_c = trip_c;
            start = Instant::now();
        }

        let t = match result {
            Ok(t) => t,
            Err(err) => {
                let msg = format!("Error processing file {}: {}", path.to_string_lossy(), err);
                error!("{}", msg);
                tx.send(Message::FatalError { err: err.into() }).unwrap();
                return 0;
            }
        };
        let (blanks, literals, iris) = proc_triple(t, &tx);
        iri_c += iris;
        blank_c += blanks;
        literal_c += literals;
    }

    for (alias, namespace) in graph.prefixes() {
        tx.send(Message::PrefixDecl {
            namespace: namespace.to_string(),
            alias: alias.to_string(),
        })
        .unwrap()
    }
    tx.send(Message::Finished {
        path: path.to_string_lossy().to_string(),
        triples: trip_c as usize,
        iris: iri_c,
        blanks: blank_c,
        literals: literal_c,
    })
    .unwrap();

    trip_c as usize
}

fn proc_triple(t: Triple, tx: &SyncSender<Message>) -> (usize, usize, usize) {
    let mut blanks = 0;
    let mut literals = 0;
    let mut iris = 0;

    match t.subject {
        NamedOrBlankNode::NamedNode(n) => {
            iris += 1;
            tx.send(Message::Resource {
                iri: normalize_iri(n.as_str()),
                pos: Position::Subject,
            })
            .unwrap();
        }
        NamedOrBlankNode::BlankNode(_) => blanks += 1,
    }

    iris += 1;
    tx.send(Message::Resource {
        iri: normalize_iri(t.predicate.as_str()),
        pos: Position::Predicate,
    })
    .unwrap();

    match t.object {
        Term::NamedNode(n) => {
            iris += 1;
            tx.send(Message::Resource {
                iri: normalize_iri(n.as_str()),
                pos: Position::Object,
            })
            .unwrap();
        }
        Term::BlankNode(_) => blanks += 1,
        Term::Literal(_) => literals += 1,
    }

    (blanks, literals, iris)
}

fn normalize_iri(iri: &str) -> String {
    const IRI_MAX_LENGTH: usize = 200;
    if iri.len() <= IRI_MAX_LENGTH {
        return iri.to_string();
    }
    match Url::parse(iri) {
        Err(_) => UnicodeSegmentation::graphemes(iri, true)
            .take(IRI_MAX_LENGTH)
            .collect(),
        Ok(url) => {
            let rest = url.as_str();
            if rest.starts_with(&format!("{}://", url.scheme())) {
                let auth_start = url.scheme().len() + 3; // "://"
                let remainder = &rest[auth_start..];
                let auth_len = remainder.find(['/', '?', '#']).unwrap_or(remainder.len());
                let prefix_len = auth_start + auth_len;
                if prefix_len >= IRI_MAX_LENGTH {
                    UnicodeSegmentation::graphemes(rest, true)
                        .take(IRI_MAX_LENGTH)
                        .collect()
                } else {
                    let budget = IRI_MAX_LENGTH - prefix_len;
                    let tail: String = UnicodeSegmentation::graphemes(&rest[prefix_len..], true)
                        .take(budget)
                        .collect();
                    format!("{}{}", &rest[..prefix_len], tail)
                }
            } else {
                // non-hierarchical (e.g. mailto:, data:) — flat fallback
                UnicodeSegmentation::graphemes(rest, true)
                    .take(IRI_MAX_LENGTH)
                    .collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use oxrdf::{BlankNode, Literal, NamedNode};

    use super::*;

    #[test]
    fn short_iri_unchanged() {
        let iri = "https://example.com/short";
        assert_eq!(normalize_iri(iri), iri);
    }

    #[test]
    fn long_hierarchical_truncates_path() {
        let iri = format!("https://example.com/{}", "a".repeat(250));
        let res = normalize_iri(&iri);
        assert_eq!(res.len(), 200);
        assert!(res.starts_with("https://example.com"));
    }

    #[test]
    fn userinfo_preserved() {
        let iri = format!("http://user:pass@example.com/{}", "b".repeat(250));
        let res = normalize_iri(&iri);
        assert_eq!(res.len(), 200);
        assert!(res.starts_with("http://user:pass@example.com"));
    }

    #[test]
    fn non_hierarchical_falls_back_flat() {
        let iri = format!("mailto:{}", "x".repeat(250));
        let res = normalize_iri(&iri);
        assert_eq!(res.len(), 200);
    }

    #[test]
    fn unparseable_falls_back_flat() {
        let iri = format!("http:// bad.com/{}", "c".repeat(250));
        let res = normalize_iri(&iri);
        assert_eq!(res.len(), 200);
    }

    #[test]
    fn unicode_grapheme_safe() {
        let iri = format!("https://example.com/{}", "é".repeat(250));
        let res = normalize_iri(&iri);
        assert_eq!(res.chars().count(), 200);
    }

    #[test]
    fn proc_triple_all_named() {
        let t = Triple::new(
            NamedNode::new_unchecked("http://ex.org/s"),
            NamedNode::new_unchecked("http://ex.org/p"),
            NamedNode::new_unchecked("http://ex.org/o"),
        );
        let (tx, rx) = std::sync::mpsc::sync_channel(100);
        let (blanks, literals, iris) = proc_triple(t, &tx);
        assert_eq!(iris, 3);
        assert_eq!(blanks, 0);
        assert_eq!(literals, 0);
        assert_eq!(rx.try_iter().count(), 3);
    }

    #[test]
    fn proc_triple_blank_literal() {
        let t = Triple::new(
            BlankNode::new_unchecked("b1"),
            NamedNode::new_unchecked("http://ex.org/p"),
            Literal::new_simple_literal("hello"),
        );
        let (tx, rx) = std::sync::mpsc::sync_channel(100);
        let (blanks, literals, iris) = proc_triple(t, &tx);
        assert_eq!(iris, 1);
        assert_eq!(blanks, 1);
        assert_eq!(literals, 1);
        assert_eq!(rx.try_iter().count(), 1);
    }

    #[test]
    fn proc_triple_named_blank_mixed() {
        let t = Triple::new(
            NamedNode::new_unchecked("http://ex.org/s"),
            NamedNode::new_unchecked("http://ex.org/p"),
            oxrdf::BlankNode::new_unchecked("b2"),
        );
        let (tx, rx) = std::sync::mpsc::sync_channel(100);
        let (blanks, literals, iris) = proc_triple(t, &tx);
        assert_eq!(iris, 2);
        assert_eq!(blanks, 1);
        assert_eq!(literals, 0);
        assert_eq!(rx.try_iter().count(), 2);
    }

    #[test]
    fn insert_resource_adds_when_no_namespace() {
        let ns_trie = NamespaceTrie::new();
        let mut iri_trie = IriTrie::new();
        let iri = "http://unknown.org/resource".into();
        insert_resource(&ns_trie, iri, &mut iri_trie);
        let res = iri_trie.longest_prefix("http://unknown.org/resource", true);
        assert!(res.is_some());
    }

    #[test]
    fn insert_resource_skips_when_namespace_exists() {
        let mut ns_trie = NamespaceTrie::new();
        ns_trie.insert("http://known.org/", ("known".into(), NamespaceSource::User));
        let mut iri_trie = IriTrie::new();
        let iri = "http://known.org/resource".into();
        insert_resource(&ns_trie, iri, &mut iri_trie);
        let res = iri_trie.longest_prefix("http://known.org/resource", true);
        assert!(res.is_none()); // should NOT be inserted
    }
}
