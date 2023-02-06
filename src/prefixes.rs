pub mod community;

use crate::counter::Counter;
use crate::iri_trie::{inc_own, update_stats, IriTrie, IriTrieExt, NodeStats};
use crate::ns_trie::{gen_alias, NamespaceSource, NamespaceTrie};
use crate::parse::{parse, ParserWrapper};
use crate::seg_tree::SegTree;
use crate::timed_task::Task;
use crate::trie::{InsertFnVisitors, Node};
use log::{debug, error, info, trace};
use rio_api::model::{NamedNode, Subject, Term, Triple};
use rio_turtle::TurtleError;
use std::collections::BTreeMap;
use std::sync::mpsc::{Receiver, SyncSender};
use std::time::Instant;
use std::{path::PathBuf, sync::mpsc::sync_channel};
use unicode_segmentation::UnicodeSegmentation;
use url::Url;

use crate::ns_trie::InferredNamespaces;
use rio_api::parser::TriplesParser;

pub enum Position {
    Subject,
    Predicate,
    Object,
}

pub enum Message {
    Resource { iri: String, pos: Position },
    PrefixDecl { namespace: String, alias: String },
    Finished { path: String, triples: usize },
    FatalError { err: TurtleError },
}

pub fn build_iri_trie(
    paths: Vec<PathBuf>,
    ns_trie: &mut NamespaceTrie,
    task: &mut Task,
    allow_subns: bool,
) -> (IriTrie, usize) {
    debug!("Building IRI trie");
    let n_workers = std::cmp::max(2, std::cmp::min(paths.len() + 1, num_cpus::get() - 2));
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

    let mut total_triples = 0;

    let mut tasks = BTreeMap::<String, Task>::new();

    pool.scope_fifo(|s| {
        let (tx, rx) = sync_channel::<Message>(100);
        for (index, path) in paths.iter().enumerate() {
            let tx = tx.clone();

            tasks.insert(
                path.to_string_lossy().to_string(),
                task.file_task(path.to_string_lossy().to_string()),
            );

            s.spawn_fifo(move |_| {
                info!("Parsing {:?} ({}/{running})", path, index + 1);
                let mut graph = parse(&path);
                proc_triples(&mut graph, &path, &tx);
            });
        }

        total_triples = handle_loop(
            &mut running,
            rx,
            &mut iri_trie,
            ns_trie,
            &mut local_ns,
            &mut tasks,
            allow_subns,
        );
    });

    handle_pref_decls(&mut iri_trie, local_ns, ns_trie);

    return (iri_trie, total_triples);
}

fn handle_loop(
    running: &mut usize,
    rx: Receiver<Message>,
    iri_trie: &mut Node<NodeStats>,
    ns_trie: &mut NamespaceTrie,
    local_ns: &mut BTreeMap<String, String>,
    tasks: &mut BTreeMap<String, Task>,
    allow_subns: bool,
) -> usize {
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

                        maintenance(iri_trie, ns_trie, allow_subns);
                    }

                    insert_resource(ns_trie, iri, iri_trie);
                }
                Message::PrefixDecl { namespace, alias } => {
                    debug!("Found local prefix {alias}: {namespace}");
                    local_ns.insert(namespace, alias);
                }
                Message::Finished { path, triples } => {
                    let mut t = tasks.remove(&path).unwrap();
                    t.triples = triples;
                    t.finish(format!("Finished task {:?} on {}", t.task_type, t.obj_name).as_str());

                    *running -= 1;
                    trace!("Running: {running}");
                }
                Message::FatalError { err } => {
                    *running -= 1;
                }
            }
        }
    }

    return trip_c.cur;
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

fn maintenance(iri_trie: &mut Node<NodeStats>, ns_trie: &mut NamespaceTrie, allow_subns: bool) {
    if let Some(size) = iri_trie.value {
        let IRI_TRIE_SIZE = 500_000;

        if size.desc > IRI_TRIE_SIZE {
            info!("IRI trie size over {IRI_TRIE_SIZE}, inferring namespaces");
            let seg_tree = SegTree::from(&*iri_trie);
            let (inferred, gbg_collected) = seg_tree.infer_namespaces();

            debug!("Adding inferred namespaces");
            let added = ns_trie.add_namespaces(&inferred, allow_subns);

            debug!("Removing {} IRIs with inferred namespaces", added.len());
            iri_trie.remove_prefixes(&added);

            debug!(
                "Forgetting {} IRIs with low occurring namespaces",
                gbg_collected.len()
            );
            iri_trie.remove_prefixes(&gbg_collected);
        }
    }
}

fn handle_pref_decls(
    iri_trie: &mut Node<NodeStats>,
    local_ns: BTreeMap<String, String>,
    ns_trie: &mut NamespaceTrie,
) {
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
    if elapsed != 0 {
        trace!(
            "Received {} resources ({}/s), {} triples ({}/s) so far",
            res_c.cur,
            (res_c.delta() as u128 / elapsed) * 1000,
            trip_c.cur,
            (trip_c.delta() as u128 / elapsed) * 1000,
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

fn proc_triples(graph: &mut ParserWrapper, path: &PathBuf, tx: &SyncSender<Message>) -> usize {
    let tx = tx.clone();

    let tid = if let Some(id) = rayon::current_thread_index() {
        id.to_string()
    } else {
        "".to_string()
    };
    let mut trip_c = 0;
    let mut start = Instant::now();
    let mut last_trip_c = 0;

    while !graph.is_end() {
        trip_c += 1;
        if trip_c % 1_000_000 == 1 {
            let elapsed = start.elapsed().as_millis();
            if elapsed != 0 {
                trace!(
                    "[Thread#{tid}] Parsed {trip_c} triples so far ({} triples/s)",
                    ((trip_c - last_trip_c) / elapsed) * 1000
                );
            }
            last_trip_c = trip_c;
            start = Instant::now();
        }

        graph
            .parse_step(&mut |t| proc_triple(t, &tx))
            .unwrap_or_else(|err| {
                let msg = format!("Error processing file {}: {}", path.to_string_lossy(), err);
                error!("{}", msg);
                tx.send(Message::FatalError { err }).unwrap();
                panic!("{}", msg)
            });
    }

    for (alias, namespace) in graph.prefixes().iter() {
        tx.send(Message::PrefixDecl {
            namespace: namespace.to_string(),
            alias: alias.to_string(),
        })
        .unwrap()
    }
    tx.send(Message::Finished {
        path: path.to_string_lossy().to_string(),
        triples: trip_c as usize,
    })
    .unwrap();

    return trip_c as usize;
}

fn proc_triple(t: Triple, tx: &SyncSender<Message>) -> Result<(), TurtleError> {
    // subject
    if let Subject::NamedNode(NamedNode { iri }) = t.subject {
        tx.send(Message::Resource {
            iri: normalize_iri(iri),
            pos: Position::Subject,
        })
        .unwrap();
    }
    // predicate
    tx.send(Message::Resource {
        iri: normalize_iri(t.predicate.iri),
        pos: Position::Predicate,
    })
    .unwrap();
    // object
    if let Term::NamedNode(NamedNode { iri }) = t.object {
        tx.send(Message::Resource {
            iri: normalize_iri(iri),
            pos: Position::Object,
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
