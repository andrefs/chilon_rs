use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashSet, VecDeque},
};

use crate::{ns_trie::NamespaceTrie, seg_tree::SegTree, trie::Node};
use log::{debug, info, warn};

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
    pub own: Option<Stats>,
    pub own_uniq: usize,
    pub desc_uniq: usize,
    pub desc: Stats,
}
#[derive(Clone, Copy, Debug)]
pub enum TriplePos {
    S,
    P,
    O,
}

pub type IriTrie = Node<NodeStats>; // todo finish

impl NodeStats {
    pub fn new() -> NodeStats {
        NodeStats {
            own: None,
            own_uniq: 0,
            desc: Default::default(),
            desc_uniq: 0,
        }
    }

    pub fn new_terminal(pos: TriplePos) -> NodeStats {
        NodeStats {
            own: match pos {
                TriplePos::S => Some(Stats {
                    s: 1,
                    ..Default::default()
                }),
                TriplePos::P => Some(Stats {
                    p: 1,
                    ..Default::default()
                }),
                TriplePos::O => Some(Stats {
                    o: 1,
                    ..Default::default()
                }),
            },
            desc_uniq: 0,
            own_uniq: 1,
            desc: Default::default(),
        }
    }
}

pub trait IriTrieStatsExt {
    fn stats(&self) -> NodeStats;
    fn set_stats(&mut self, stats: Option<NodeStats>);
}

impl IriTrieStatsExt for IriTrie {
    fn stats(&self) -> NodeStats {
        match self.value {
            None => NodeStats::new(),
            Some(NodeStats {
                own,
                desc,
                desc_uniq,
                own_uniq,
            }) => NodeStats {
                own: match own {
                    None => Default::default(),
                    Some(s) => Some(s),
                },
                desc,
                desc_uniq,
                own_uniq,
            },
        }
    }
    fn set_stats(&mut self, stats: Option<NodeStats>) {
        self.value = stats;
    }
}

impl Default for NodeStats {
    fn default() -> Self {
        NodeStats {
            own: None,
            desc: Default::default(),
            desc_uniq: 0,
            own_uniq: 0,
        }
    }
}

impl Stats {
    pub fn inc(&mut self, pos: TriplePos) {
        match pos {
            TriplePos::S => self.inc_s(),
            TriplePos::P => self.inc_p(),
            TriplePos::O => self.inc_o(),
        }
    }
    pub fn inc_s(&mut self) {
        self.s += 1;
        self.total += 1;
    }
    pub fn inc_p(&mut self) {
        self.p += 1;
        self.total += 1;
    }
    pub fn inc_o(&mut self) {
        self.o += 1;
        self.total += 1;
    }
}

pub fn init_stats(n: &mut IriTrie) {
    let new_stats = NodeStats::new();
    n.value = Some(new_stats);
}
pub fn inc_stats(position: TriplePos) -> impl Fn(&mut IriTrie) -> () {
    move |n: &mut IriTrie| {
        let new_stats = NodeStats::new();
        if n.value.is_none() {
            n.value = Some(new_stats);
        }
        n.value.as_mut().unwrap().desc.inc(position)
    }
}
pub fn dec_stats(parent: &mut IriTrie, _: char, child: &IriTrie) {
    let mut par_desc = parent.value.as_mut().unwrap_or(&mut NodeStats::new()).desc;
    let child_own = child
        .value
        .as_ref()
        .unwrap()
        .own
        .unwrap_or(Default::default());
    let child_desc = child.value.as_ref().unwrap().desc;

    par_desc.s -= child_own.s + child_desc.s;
    par_desc.p -= child_own.p + child_desc.p;
    par_desc.o -= child_own.o + child_desc.o;
}

pub fn upd_stats_visitor(node: &mut IriTrie, _: char, _: Option<&IriTrie>) {
    update_stats(node);
}

pub fn update_stats(node: &mut IriTrie) {
    let (desc_s, desc_p, desc_o, desc_uniq) = node
        .children
        .iter()
        .map(|(_, child)| {
            let child_stats = child.stats();
            let own = child_stats.own.unwrap_or_default();
            (
                own.s + child_stats.desc.s,
                own.p + child_stats.desc.p,
                own.o + child_stats.desc.o,
                child_stats.desc_uniq + child_stats.own_uniq,
            )
        })
        .fold(
            (0, 0, 0, 0),
            |(desc_s, desc_p, desc_o, desc_uniq), (delta_s, delta_p, delta_o, delta_uniq)| {
                (
                    desc_s + delta_s,
                    desc_p + delta_p,
                    desc_o + delta_o,
                    desc_uniq + delta_uniq,
                )
            },
        );

    let desc_total = desc_s + desc_p + desc_o;

    let stats = Some(NodeStats {
        desc: Stats {
            s: desc_s,
            p: desc_p,
            o: desc_o,
            total: desc_total,
        },
        desc_uniq,
        own: node.stats().own,
        own_uniq: node.stats().own_uniq,
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
    fn count(&self) -> u32;
    fn remove_leaves(&mut self) -> bool;
    fn remove_leaves_aux(&mut self, cur_str: String) -> bool;
    fn remove_known_prefixes(&mut self, ns_vec: &Vec<String>);
    fn remove_prefix<S: ?Sized + Borrow<str>>(&mut self, namespace: &S) -> bool;
    fn value_along_path(&mut self, cur_str: String, str_acc: String, v: &mut Vec<(String, String)>);
    fn infer_namespaces_1(&self) -> Vec<String>;
    fn infer_namespaces_2(&self) -> Vec<String>;
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
                self.value.unwrap().desc.total.to_string()
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

    fn count(&self) -> u32 {
        let stats = self.stats();
        let mut total = 0;
        total += stats.desc.total;
        if let Some(o) = stats.own {
            total += o.total;
        }
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
        warn!("IRIs with unknwon namespaces: {}", self.count(),);
    }

    fn remove_prefix<S: ?Sized + Borrow<str>>(&mut self, namespace: &S) -> bool {
        self.remove_fn(namespace, true, Some(&upd_stats_visitor))
    }

    fn infer_namespaces_1(&self) -> Vec<String> {
        let mut v: HashSet<String> = HashSet::new();
        for (s, node) in self.iter_leaves() {
            if let Some(stats) = node.value {
                if stats.desc.total > 10 {
                    v.insert(s.clone());
                }
            }
        }
        return v.into_iter().collect();
    }
    fn infer_namespaces_2(&self) -> Vec<String> {
        let mut v: HashSet<String> = HashSet::new();
        for (s, node) in self.iter_leaves() {
            if let Some(stats) = node.value {
                if stats.desc.total > 10 {
                    v.insert(s.clone());
                }
            }
        }
        return v.into_iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_fn_dec_stats() {
        let pos = TriplePos::S;
        let stats = NodeStats::new_terminal(pos);
        let mut t = Node::new();
        t.insert_fn("ab", stats, Some(&update_stats));
        t.insert_fn("abcde", stats, Some(&update_stats));
        t.remove_fn("abcd", true, Some(&upd_stats_visitor));

        assert_eq!(t.value.unwrap().desc.s, 1);
        assert_eq!(t.value.unwrap().desc.p, 0);
        assert_eq!(t.value.unwrap().desc.o, 0);
        assert_eq!(t.value.unwrap().desc.total, 1);
    }
}
