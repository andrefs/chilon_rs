#![feature(btree_drain_filter)]
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashSet},
    fmt::format,
};

use crate::iri_trie::{IriTrie, Stats};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SegTree {
    pub value: usize,
    pub children: BTreeMap<String, SegTree>,
}

impl SegTree {
    fn from_aux(&mut self, iri_trie: IriTrie, word_acc: String) {
        if iri_trie.children.is_empty() {
            self.children.insert(
                word_acc,
                SegTree {
                    children: BTreeMap::new(),
                    value: match iri_trie.value {
                        Some(stats) => stats.desc.total as usize,
                        None => 0,
                    },
                },
            );
            return;
        }
        for (c, node) in iri_trie.children {
            if ['/', '#'].contains(&c) {
                let sub_tree = SegTree {
                    children: BTreeMap::new(),
                    value: match node.value {
                        Some(stats) => stats.desc.total as usize,
                        None => 0,
                    },
                };
                self.children
                    .entry(format!("{word_acc}{c}"))
                    .or_insert(sub_tree)
                    .from_aux(node, "".to_string());
            } else {
                self.from_aux(node, format!("{word_acc}{c}"));
            }
        }
    }

    pub fn infer_namespaces(&self) -> Vec<String> {
        let mut h: BTreeSet<NamespaceCandidate> = BTreeSet::new();
        h.insert(NamespaceCandidate {
            size: self.value,
            children: self.children.len(),
            namespace: "".to_string(),
            node: self.clone(),
        });

        infer_namespaces_aux(&mut h);

        return h.iter().map(|ns| ns.namespace.clone()).collect();
    }
}

fn infer_namespaces_aux(h: &mut BTreeSet<NamespaceCandidate>) {
    let MAX_NS = 5;
    while h.len() < MAX_NS {
        let h_len = h.len();
        let mut found = false;
        match h
            .drain_filter(|item| {
                if !found && item.children + h_len < MAX_NS {
                    found = true;
                    return true;
                }
                return false;
            })
            .collect::<Vec<_>>()
            .first()
            .cloned()
        {
            Some(parent) => {
                h.remove(&parent);
                for (c, node) in parent.node.children {
                    h.insert(NamespaceCandidate {
                        size: node.value,
                        children: node.children.len(),
                        namespace: format!("{}{c}", parent.namespace),
                        node: node.clone(),
                    });
                }
            }
            None => return,
        }
    }
}

impl From<IriTrie> for SegTree {
    fn from(iri_trie: IriTrie) -> Self {
        let mut res = SegTree {
            value: 0,
            children: BTreeMap::new(),
        };

        res.from_aux(iri_trie, "".to_string());

        return res;
    }
}

#[derive(Clone, Debug)]
pub struct NamespaceCandidate {
    size: usize,
    children: usize,
    namespace: String,
    node: SegTree,
}

impl Ord for NamespaceCandidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.size < other.size {
            return Ordering::Less;
        }
        if self.size > other.size {
            return Ordering::Greater;
        }
        if self.children > other.size {
            return Ordering::Less;
        }
        if self.children < other.size {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for NamespaceCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for NamespaceCandidate {
    fn eq(&self, other: &Self) -> bool {
        (self.size, self.children) == (other.size, other.children)
    }
}

impl Eq for NamespaceCandidate {}
