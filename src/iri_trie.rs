use std::{
    borrow::Borrow,
    collections::{BTreeMap, VecDeque},
    fmt::Debug,
};

use crate::trie::Node;
use itertools::Itertools;
use log::{info, warn};

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
pub type IriTrie = Node<NodeStats>; // todo finish

impl NodeStats {
    pub fn new() -> NodeStats {
        NodeStats {
            own: 0,       // occurrences of this IRI (as terminal)
            desc: 0,      // occurrences of IRIs with this prefix
            uniq_desc: 0, // occurrences of IRIs with this prefix (unique)
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

impl IriTrieStatsExt for IriTrie {
    fn stats(&self) -> NodeStats {
        self.value.unwrap_or_default()
    }
    fn set_stats(&mut self, stats: Option<NodeStats>) {
        self.value = stats;
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

pub fn init_stats(n: &mut IriTrie) {
    let new_stats = NodeStats::new();
    n.value = Some(new_stats);
}

pub fn upd_stats_visitor(node: &mut IriTrie, _: char, _: Option<&IriTrie>) {
    update_stats(node);
}

pub fn inc_own(node: &mut IriTrie) {
    let mut stats = node.stats();
    stats.own += 1;
    node.set_stats(Some(stats));
}

pub fn update_stats(node: &mut IriTrie) {
    let (desc, uniq_desc) = node
        .children
        .iter()
        .map(|(_, child)| {
            let child_stats = child.stats();
            let desc = child_stats.own + child_stats.desc;
            let uniq_desc = if child_stats.own == 0 { 0 } else { 1 } + child_stats.uniq_desc;
            (desc, uniq_desc)
        })
        .fold(
            (0, 0),
            |(desc, uniq_desc), (delta_desc, delta_uniq_desc)| {
                (desc + delta_desc, uniq_desc + delta_uniq_desc)
            },
        );

    let stats = Some(NodeStats {
        desc,
        uniq_desc,
        own: node.stats().own,
    });
    node.set_stats(stats);
}

pub struct NodeIter<'a, T: Debug + Clone> {
    queue: Vec<(String, &'a Node<T>)>,
}

impl<T: Debug + Clone> Node<T> {
    pub fn iter_leaves(&self) -> NodeIter<'_, T> {
        NodeIter {
            queue: Vec::from([("".to_string(), self)]),
        }
    }
}

impl<'a, T: Debug + Clone> Iterator for NodeIter<'a, T> {
    type Item = (String, &'a Node<T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() {
            return None;
        }
        let (s, n) = self.queue.pop().unwrap();

        let mut sorted_children = n.children.iter().collect::<Vec<_>>();
        sorted_children.sort_by(|(k1, _), (k2, _)| (**k1).cmp(*k2));
        for (k, v) in sorted_children.iter().rev() {
            self.queue.push((format!("{s}{k}"), &v));
        }

        if n.children.is_empty() {
            return Some((s, n));
        }
        return self.next();
    }
}

pub trait IriTrieExt {
    fn count(&self) -> usize;
    fn remove_leaves(&mut self) -> bool;
    fn remove_leaves_aux(&mut self, cur_str: String) -> bool;
    fn remove_prefixes(&mut self, ns_vec: &Vec<String>);
    fn remove_prefix<S: ?Sized + Borrow<str>>(&mut self, namespace: &S) -> Option<NodeStats>;
    fn value_along_path(&mut self, cur_str: String, str_acc: String, v: &mut Vec<(String, String)>);
}

impl IriTrieExt for IriTrie {
    fn value_along_path(
        &mut self,
        str_left: String,
        str_acc: String,
        v: &mut Vec<(String, String)>,
    ) {
        v.push((
            str_acc.clone(),
            if self.value.is_some() {
                self.value.unwrap().desc.to_string()
            } else {
                "".to_string()
            },
        ));
        if str_left.is_empty() {
            return;
        }

        let first_char = str_left.chars().next().unwrap();
        let rest = &str_left[first_char.len_utf8()..];

        if !self.children.contains_key(&first_char) {
            panic!("Something is wrong: {str_left} has no char {first_char} ");
        }

        let node = self
            .children
            .get_mut(&first_char)
            .unwrap()
            .value_along_path(rest.to_string(), format!("{str_acc}{first_char}"), v);
    }

    fn count(&self) -> usize {
        let stats = self.stats();
        let mut total = 0;
        total += stats.desc + stats.own;
        return total;
    }

    fn remove_leaves(&mut self) -> bool {
        self.remove_leaves_aux("".to_string())
    }

    fn remove_leaves_aux(&mut self, cur_str: String) -> bool {
        if self.children.is_empty() {
            return false;
        }
        let mut deleted = false;
        let mut to_remove = Vec::<char>::new();

        for (&ch, node) in self.children.iter_mut() {
            let node_had_children = !node.children.is_empty();
            let child_deleted = node.remove_leaves_aux(format!("{}{}", cur_str, ch));
            if !child_deleted && ['/', '#'].contains(&ch) {
                to_remove.push(ch);
                // if ch was the last one it doesn't count
                //deleted = node_had_children;
                deleted = true;
            }
            deleted = deleted || child_deleted;
        }
        for ch in to_remove.iter() {
            let sub_node = self.get_mut(*ch).unwrap();
            sub_node.children = BTreeMap::new();
        }
        return deleted;
    }

    fn remove_prefixes(&mut self, ns_vec: &Vec<String>) {
        for namespace in ns_vec.iter() {
            self.remove_prefix(namespace);
        }
        warn!(
            "IRIs with unknown namespaces: {} ({} occurrences).",
            self.count(),
            self.value.unwrap_or(Default::default()).desc,
        );
        let examples = self.iter_leaves().take(10).map(|x| x.0).collect::<Vec<_>>();
        // 1 example is the root node
        if examples.len() > 1 {
            info!("10 examples: {:#?}", examples);
        }
    }

    fn remove_prefix<S: ?Sized + Borrow<str>>(&mut self, namespace: &S) -> Option<NodeStats> {
        //trace!("Removing namespace {} from IRI trie", namespace.borrow());
        self.remove_fn(namespace, true, Some(&upd_stats_visitor))
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

        assert_eq!(trie.count(), 2);
    }

    #[test]
    fn remove_fn_dec_stats() {
        let stats = NodeStats::new_terminal();
        let mut t = Node::new();
        let visitors = InsertFnVisitors {
            node: Some(&update_stats),
            terminal: Some(&inc_own),
        };
        t.insert_fn("ab", stats, &visitors);
        t.insert_fn("ab", stats, &visitors);
        t.insert_fn("abcde", stats, &visitors);
        t.remove_fn("abcd", true, Some(&upd_stats_visitor));

        assert_eq!(t.stats().desc, 2);
        assert_eq!(t.stats().uniq_desc, 1);
    }
}
