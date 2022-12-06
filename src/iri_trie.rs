use std::{
    borrow::Borrow,
    collections::{BTreeMap, VecDeque},
};

use crate::trie::Node;
use log::warn;

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
}
pub type IriTrie = Node<NodeStats>; // todo finish

impl NodeStats {
    pub fn new() -> NodeStats {
        NodeStats { own: 0, desc: 0 }
    }

    pub fn new_terminal() -> NodeStats {
        NodeStats { own: 1, desc: 0 }
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
        NodeStats { desc: 0, own: 0 }
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
    let desc = node
        .children
        .iter()
        .map(|(_, child)| {
            let child_stats = child.stats();
            child_stats.own + child_stats.desc
        })
        .fold(0, |desc, delta| desc + delta);

    let stats = Some(NodeStats {
        desc,
        own: node.stats().own,
    });
    node.set_stats(stats);
}

pub struct NodeIter<'a, T> {
    queue: VecDeque<(String, &'a Node<T>)>,
}

impl<T> Node<T> {
    pub fn iter_leaves(&self) -> NodeIter<'_, T> {
        NodeIter {
            queue: VecDeque::from([("".to_string(), self)]),
        }
    }
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = (String, &'a Node<T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() {
            return None;
        }
        let (s, n) = self.queue.pop_front().unwrap();
        for (k, v) in n.children.iter() {
            self.queue.push_front((format!("{s}{k}"), &v));
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
    fn remove_known_prefixes(&mut self, ns_vec: &Vec<String>);
    fn remove_prefix<S: ?Sized + Borrow<str>>(&mut self, namespace: &S) -> bool;
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

    fn remove_known_prefixes(&mut self, ns_vec: &Vec<String>) {
        for namespace in ns_vec.iter() {
            self.remove_prefix(namespace);
        }
        warn!(
            "IRIs with unknown namespaces: {} {:#?}",
            self.count(),
            self.value
        );
    }

    fn remove_prefix<S: ?Sized + Borrow<str>>(&mut self, namespace: &S) -> bool {
        self.remove_fn(namespace, true, Some(&upd_stats_visitor))
    }
}

#[cfg(test)]
mod tests {
    use crate::trie::InsertFnVisitors;

    use super::*;

    #[test]
    fn remove_fn_dec_stats() {
        let stats = NodeStats::new_terminal();
        let mut t = Node::new();
        t.insert_fn(
            "ab",
            stats,
            &InsertFnVisitors {
                node: Some(&update_stats),
                terminal: Some(&inc_own),
            },
        );
        t.insert_fn(
            "abcde",
            stats,
            &InsertFnVisitors {
                node: Some(&update_stats),
                terminal: Some(&inc_own),
            },
        );
        t.remove_fn("abcd", true, Some(&upd_stats_visitor));

        assert_eq!(t.value.unwrap().desc, 1);
    }
}
