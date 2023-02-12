use std::{
    collections::VecDeque,
    fmt::Debug,
    sync::{Arc, RwLock},
};

use log::{info, warn};
use r3bl_rs_utils::{Arena, MTArena};

use crate::trie::{NodeData, Trie};

// Represents occurrences as subject, predicate or object
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Stats {
    pub s: u32,
    pub p: u32,
    pub o: u32,
    pub total: u32,
}

// Each node keeps its own stats (if terminal) and its descendants stats
#[derive(Debug, Clone, Copy)]
pub struct NodeStats {
    pub own: usize,
    pub desc: usize,
    pub uniq_desc: usize,
}
pub type IriTrie = Trie<NodeStats>; // todo finish

impl NodeStats {
    pub fn new() -> NodeStats {
        NodeStats {
            own: 0,
            desc: 0,
            uniq_desc: 0,
        }
    }

    pub fn new_terminal() -> NodeStats {
        NodeStats {
            own: 1,
            desc: 0,
            uniq_desc: 0,
        }
    }
}

pub trait IriTrieStatsExt {
    fn stats(&self) -> NodeStats;
    fn set_stats(&mut self, stats: Option<NodeStats>);
}

impl IriTrieStatsExt for NodeData<NodeStats> {
    fn stats(&self) -> NodeStats {
        self.value.unwrap_or_default()
    }
    fn set_stats(&mut self, stats: Option<NodeStats>) {
        self.value = stats;
    }
}

impl IriTrieStatsExt for IriTrie {
    fn stats(&self) -> NodeStats {
        let arena_arc = self.arena.get_arena_arc();
        let arena_read = arena_arc.read().unwrap();
        let root_arc = arena_read.get_node_arc(self.root_id).unwrap();
        let root_read = root_arc.read().unwrap();
        root_read.payload.stats()
    }
    fn set_stats(&mut self, stats: Option<NodeStats>) {
        let arena_arc = self.arena.get_arena_arc();
        let arena_read = arena_arc.read().unwrap();
        let root_arc = arena_read.get_node_arc(self.root_id).unwrap();
        let mut root_write = root_arc.write().unwrap();
        root_write.payload.set_stats(stats)
    }
}

impl Default for NodeStats {
    fn default() -> Self {
        NodeStats {
            desc: 0,
            own: 0,
            uniq_desc: 0,
        }
    }
}

pub fn inc_own(trie: &mut IriTrie, id: usize) {
    let arena_arc = trie.arena.get_arena_arc();
    let arena_read = arena_arc.read().unwrap();
    let node_arc = arena_read.get_node_arc(id).unwrap();
    let mut node_write = node_arc.write().unwrap();

    let mut stats = node_write.payload.stats();

    stats.own += 1;
    node_write.payload.set_stats(Some(stats));
}

pub fn upd_stats_visitor(trie: &mut IriTrie, parent_id: usize, ch: char) {
    update_stats(trie, parent_id);
}

pub fn update_stats(trie: &mut IriTrie, id: usize) {
    let arena_arc = trie.arena.get_arena_arc();
    let arena_read = arena_arc.read().unwrap();
    let node_arc = arena_read.get_node_arc(id).unwrap();

    let ((desc, uniq_desc), own) = {
        let node_read = node_arc.read().unwrap();
        (
            node_read
                .payload
                .children
                .iter()
                .map(|(_, child)| {
                    let child_arc = arena_read.get_node_arc(*child).unwrap();
                    let child_read = child_arc.read().unwrap();
                    let child_stats = child_read.payload.stats();
                    let desc = child_stats.own + child_stats.desc;
                    let uniq_desc =
                        if child_stats.own == 0 { 0 } else { 1 } + child_stats.uniq_desc;
                    (desc, uniq_desc)
                })
                .fold(
                    (0, 0),
                    |(desc, uniq_desc), (delta_desc, delta_uniq_desc)| {
                        (desc + delta_desc, uniq_desc + delta_uniq_desc)
                    },
                ),
            node_read.payload.stats().own,
        )
    };

    let stats = Some(NodeStats {
        desc,
        uniq_desc,
        own,
    });

    let mut node_write = node_arc.write().unwrap();
    node_write.payload.set_stats(stats);
}

pub struct NodeIter<T: Debug + Sync + Send + Clone> {
    arena_arc: Arc<RwLock<Arena<NodeData<T>>>>,
    queue: VecDeque<(String, usize)>,
}

impl<T: Debug + Sync + Send + Clone> Trie<T> {
    pub fn iter_leaves(&self) -> NodeIter<T> {
        let arena_arc = self.arena.get_arena_arc();

        NodeIter {
            arena_arc,
            queue: VecDeque::from([("".to_string(), self.root_id)]),
        }
    }
}

impl<T: Debug + Sync + Send + Clone> Iterator for NodeIter<T> {
    type Item = (String, NodeData<T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() {
            return None;
        }
        let (s, id) = self.queue.pop_front().unwrap();

        let arena_arc = self.arena_arc.clone();
        let arena_read = arena_arc.read().unwrap();
        let node_arc = arena_read.get_node_arc(id).unwrap();
        let node_read = node_arc.read().unwrap();

        let children = node_read.payload.children.clone();
        let mut sorted_children: Vec<_> = children.iter().collect();
        sorted_children.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

        for (k, v) in sorted_children.iter().rev() {
            self.queue.push_front((format!("{s}{k}"), **v));
        }
        if node_read.payload.children.is_empty() {
            return Some((s, node_read.payload.clone()));
        }
        return self.next();
    }
}

pub trait IriTrieExt {
    fn count(&self) -> usize;
    //fn remove_leaves(&mut self) -> bool;
    //fn remove_leaves_aux(&mut self, cur_str: String) -> bool;
    fn remove_prefixes(&mut self, ns_vec: &Vec<String>);
    fn remove_prefix(&mut self, namespace: &str) -> Option<NodeStats>;
    //fn value_along_path(&mut self, cur_str: String, str_acc: String, v: &mut Vec<(String, String)>);
}

impl IriTrieExt for IriTrie {
    fn count(&self) -> usize {
        let arena_arc = self.arena.get_arena_arc();
        let arena_read = arena_arc.read().unwrap();
        let root_arc = arena_read.get_node_arc(self.root_id).unwrap();
        let root_read = root_arc.read().unwrap();

        let stats = root_read.payload.stats();
        let mut total = 0;
        total += stats.desc + stats.own;
        return total;
    }

    fn remove_prefixes(&mut self, ns_vec: &Vec<String>) {
        for namespace in ns_vec.iter() {
            self.remove_prefix(namespace);
        }
        warn!(
            "IRIs with unknown namespaces: {} ({} occurrences).",
            self.count(),
            self.stats().desc,
        );
        let examples = self.iter_leaves().take(10).map(|x| x.0).collect::<Vec<_>>();
        // 1 example is the root node
        if examples.len() > 1 {
            info!("10 examples: {:#?}", examples);
        }
    }

    fn remove_prefix(&mut self, namespace: &str) -> Option<NodeStats> {
        let res = self.remove_fn(namespace, true, Some(&upd_stats_visitor));
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::trie::InsertFnVisitors;

    use super::*;

    #[test]
    fn inc_own_test() {
        let mut trie = IriTrie::new();
        trie.insert_fn(
            "http://example.org/",
            Default::default(),
            &InsertFnVisitors {
                node: None,
                terminal: Some(&inc_own),
            },
        );

        let res = trie.find("http://example.org/", true).unwrap();

        assert_eq!(res.0.stats().own, 1);
        assert_eq!(res.0.stats().desc, 0);
        assert_eq!(res.0.stats().uniq_desc, 0);
    }

    #[test]
    fn update_stats_test() {
        let mut trie = IriTrie::new();
        trie.insert_fn(
            "http://example.org/",
            Default::default(),
            &InsertFnVisitors {
                node: Some(&update_stats),
                terminal: Some(&inc_own),
            },
        );

        trie.insert_fn(
            "http://example.org/path1",
            Default::default(),
            &InsertFnVisitors {
                node: Some(&update_stats),
                terminal: Some(&inc_own),
            },
        );
        trie.insert_fn(
            "http://example.org/path2",
            Default::default(),
            &InsertFnVisitors {
                node: Some(&update_stats),
                terminal: Some(&inc_own),
            },
        );

        let res = trie.find("http://example.org/", true).unwrap();

        assert_eq!(res.0.stats().own, 1);
        assert_eq!(res.0.stats().desc, 2);
        assert_eq!(res.0.stats().uniq_desc, 2);
    }

    #[test]
    fn iter_test() {
        let mut trie = IriTrie::new();
        trie.insert("a", Default::default());
        trie.insert("abc", Default::default());
        trie.insert("abcdef", Default::default());
        trie.insert("ghi", Default::default());
        trie.insert("g", Default::default());

        let leaves = trie.iter_leaves().collect::<Vec<_>>();

        assert_eq!(leaves.len(), 2);
        assert_eq!(leaves[0].0, "abcdef");
        assert_eq!(leaves[1].0, "ghi");
    }

    #[test]
    fn count_test() {
        let mut trie = IriTrie::new();
        let visitors = InsertFnVisitors {
            node: Some(&update_stats),
            terminal: Some(&inc_own),
        };
        trie.insert_fn("a", Default::default(), &visitors);
        trie.insert_fn("abc", Default::default(), &visitors);
        trie.insert_fn("abcdef", Default::default(), &visitors);
        trie.insert_fn("ghi", Default::default(), &visitors);
        trie.insert_fn("g", Default::default(), &visitors);

        assert_eq!(trie.count(), 5);
    }

    #[test]

    fn remove_prefix_test() {
        let mut trie = IriTrie::new();
        let visitors = InsertFnVisitors {
            node: Some(&update_stats),
            terminal: Some(&inc_own),
        };
        trie.insert_fn("http://example.org/", Default::default(), &visitors);
        trie.insert_fn("http://example.org/path1", Default::default(), &visitors);
        trie.insert_fn("http://example.org/path2", Default::default(), &visitors);

        trie.remove_prefix("http://example.org/pat");

        assert_eq!(trie.count(), 1);
    }

    #[test]
    fn remove_prefixes_test() {
        let mut trie = IriTrie::new();
        let visitors = InsertFnVisitors {
            node: Some(&update_stats),
            terminal: Some(&inc_own),
        };
        trie.insert_fn("http://example.org/path1/a", Default::default(), &visitors);
        trie.insert_fn("http://example.org/path1/b", Default::default(), &visitors);
        trie.insert_fn("http://example.org/path2/a", Default::default(), &visitors);
        trie.insert_fn("http://example.org/path2/b", Default::default(), &visitors);
        trie.insert_fn("http://example.org/path3/a", Default::default(), &visitors);
        trie.insert_fn("http://example.org/path3/b", Default::default(), &visitors);

        trie.remove_prefixes(&vec![
            "http://example.org/path1".to_string(),
            "http://example.org/path2".to_string(),
        ]);
        //trie.remove_prefix("http://example.org/path1");
        //trie.remove_prefix("http://example.org/path2");

        assert_eq!(trie.count(), 2);
    }

    #[test]
    fn remove_fn_dec_stats() {
        let stats = NodeStats::new_terminal();
        let mut t = IriTrie::new();
        let visitors = InsertFnVisitors {
            node: Some(&update_stats),
            terminal: Some(&inc_own),
        };

        t.insert_fn("ab", stats, &visitors);
        t.insert_fn("abcde", stats, &visitors);
        t.remove_fn("abcd", true, Some(&upd_stats_visitor));

        assert_eq!(t.stats().desc, 2);
        assert_eq!(t.stats().uniq_desc, 1);
    }
}
