use std::{borrow::Borrow, collections::BTreeMap};

use crate::{ns_trie::NamespaceTrie, trie::Node};

// Represents occurrences as subject, predicate or object
#[derive(Debug, Default, Clone, Copy)]
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
            desc: Default::default(),
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
            Some(NodeStats { own, desc }) => NodeStats {
                own: match own {
                    None => Default::default(),
                    Some(s) => Some(s),
                },
                desc,
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
    update_desc_stats(node);
}

pub fn update_desc_stats(node: &mut IriTrie) {
    let desc_s = 0 + node
        .children
        .iter()
        .map(|(_, child)| {
            let stats = child.stats();
            return if let Some(c) = stats.own { c.s } else { 0 } + stats.desc.s;
        })
        .sum::<u32>();
    let desc_p = 0 + node
        .children
        .iter()
        .map(|(_, child)| {
            let stats = child.stats();
            return if let Some(c) = stats.own { c.p } else { 0 } + stats.desc.p;
        })
        .sum::<u32>();
    let desc_o = 0 + node
        .children
        .iter()
        .map(|(_, child)| {
            let stats = child.stats();
            return if let Some(c) = stats.own { c.o } else { 0 } + stats.desc.o;
        })
        .sum::<u32>();
    let desc_total = desc_s + desc_p + desc_o;

    let desc_stats = Some(NodeStats {
        desc: Stats {
            s: desc_s,
            p: desc_p,
            o: desc_o,
            total: desc_total,
        },
        own: node.stats().own,
    });
    node.set_stats(desc_stats);
}

pub trait IriTrieExt {
    fn count(&self) -> u32;
    fn remove_leaves(&mut self) -> bool;
    fn remove_leaves_aux(&mut self, cur_str: String) -> bool;
    fn remove_known_prefixes(&mut self, ns_map: &NamespaceTrie);
    fn remove_prefix<S: ?Sized + Borrow<str>>(&mut self, namespace: &S) -> bool;
}

impl IriTrieExt for IriTrie {
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
            let child_deleted = node.remove_leaves_aux(format!("{}{}", cur_str, ch));
            if !child_deleted && ['/', '#'].contains(&ch) {
                to_remove.push(ch);
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

    fn remove_known_prefixes(&mut self, ns_trie: &NamespaceTrie) {
        for (_, node) in ns_trie.iter() {
            if let Some(namespace) = &node.value {
                self.remove_prefix(&namespace.as_str());
            }
        }
    }

    fn remove_prefix<S: ?Sized + Borrow<str>>(&mut self, namespace: &S) -> bool {
        self.remove_fn(namespace, true, Some(&upd_stats_visitor))
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
        t.insert_fn("ab", stats, Some(&update_desc_stats));
        t.insert_fn("abcde", stats, Some(&update_desc_stats));
        t.remove_fn("abcd", true, Some(&upd_stats_visitor));

        assert_eq!(t.value.unwrap().desc.s, 1);
        assert_eq!(t.value.unwrap().desc.p, 0);
        assert_eq!(t.value.unwrap().desc.o, 0);
        assert_eq!(t.value.unwrap().desc.total, 1);
    }
}
