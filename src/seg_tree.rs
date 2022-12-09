#![feature(btree_drain_filter)]
use log::info;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    usize,
};
use url::Url;

use crate::iri_trie::IriTrie;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SegTree {
    pub value: usize,
    pub children: BTreeMap<String, SegTree>,
}

impl SegTree {
    fn from_aux(&mut self, iri_trie: &IriTrie, word_acc: String, prev_str: &str) {
        if iri_trie.children.is_empty() {
            self.children.insert(
                word_acc,
                SegTree {
                    children: BTreeMap::new(),
                    value: match iri_trie.value {
                        Some(stats) => stats.desc,
                        None => 0,
                    },
                },
            );
            return;
        }
        for (c, node) in &iri_trie.children {
            if ['/', '#'].contains(&c) {
                let ns_cand = format!("{prev_str}{word_acc}{c}");
                let url_obj = Url::parse(ns_cand.as_str());

                // this is not a URL or the kind we want
                if url_obj.is_err() || !url_obj.unwrap().has_host() {
                    println!("    error parsing, will continue current segment");
                    self.from_aux(&node, format!("{word_acc}{c}"), prev_str);
                    return;
                }

                let sub_tree = SegTree {
                    children: BTreeMap::new(),
                    value: match node.value {
                        Some(stats) => stats.desc,
                        None => 0,
                    },
                };
                self.children
                    .entry(format!("{word_acc}{c}"))
                    .or_insert(sub_tree)
                    .from_aux(
                        &node,
                        "".to_string(),
                        format!("{prev_str}{word_acc}{c}").as_str(),
                    );
            } else {
                self.from_aux(&node, format!("{word_acc}{c}"), prev_str);
            }
        }
    }

    pub fn infer_namespaces(&self) -> Vec<(String, usize)> {
        let mut h: BTreeSet<NamespaceCandidate> = BTreeSet::new();
        let MIN_NS_SIZE = 20;

        // self is empty string root node
        for (ns, st) in self.children.iter() {
            // include only children worthy of being namespaces
            if st.could_be_ns(MIN_NS_SIZE) {
                h.insert(NamespaceCandidate {
                    size: st.value,
                    children: st.children.len(),
                    namespace: ns.to_string(),
                    node: st.clone(),
                });
            }
        }

        infer_namespaces_aux(&mut h, MIN_NS_SIZE);

        return h.iter().map(|ns| (ns.namespace.clone(), ns.size)).collect();
    }

    pub fn could_be_ns(&self, MIN_NS_SIZE: usize) -> bool {
        self.value >= MIN_NS_SIZE
    }
}

fn infer_namespaces_aux(h: &mut BTreeSet<NamespaceCandidate>, MIN_NS_SIZE: usize) {
    for x in h.iter() {
        info!(
            "infer_namespaces_aux {} {} {}",
            x.namespace, x.size, x.children
        );
    }
    let MAX_NS = 5;
    let mut expanded = 0;
    let mut added = true;

    while added && (expanded < MAX_NS) {
        //while h.len() < MAX_NS {
        added = false;
        let h_len = h.len();
        let mut found = false;
        match h
            .drain_filter(|item| {
                println!("CHECKING {} {expanded}", item.namespace);
                if !found {
                    let suitable = item
                        .node
                        .children
                        .iter()
                        .filter(|(_, n)| n.could_be_ns(MIN_NS_SIZE))
                        .collect::<Vec<_>>();
                    if !suitable.is_empty() && ((suitable.len() + h_len) <= MAX_NS) {
                        found = true;
                        println!("    YES");
                        return true;
                    }
                }
                println!("    NOPE");
                return false;
            })
            .collect::<Vec<_>>()
            .first()
            .cloned()
        {
            Some(parent) => {
                h.remove(&parent);
                expanded -= 1;
                println!("REMOVING {} {expanded}", parent.namespace);

                for (seg, node) in parent.node.children {
                    if node.could_be_ns(MIN_NS_SIZE) {
                        expanded += 1;
                        added = true;
                        println!("ADDING {}{seg} {expanded}", parent.namespace);
                        h.insert(NamespaceCandidate {
                            size: node.value,
                            children: node.children.len(),
                            namespace: format!("{}{seg}", parent.namespace),
                            node: node.clone(),
                        });
                    }
                }
            }
            None => return,
        }
    }
}

impl From<&IriTrie> for SegTree {
    fn from(iri_trie: &IriTrie) -> Self {
        let mut res = SegTree {
            value: 0,
            children: BTreeMap::new(),
        };

        res.from_aux(iri_trie, "".to_string(), "");

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
