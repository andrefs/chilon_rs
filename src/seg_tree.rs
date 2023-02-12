use r3bl_rs_utils::MTArena;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};
use url::Url;

use crate::{iri_trie::IriTrie, ns_trie::NamespaceSource};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SegNodeData {
    pub value: usize,
    pub children: BTreeMap<String, usize>,
}

#[derive(Debug)]
pub struct SegTree {
    pub arena: MTArena<SegNodeData>,
    pub root_id: usize,
}

impl From<&IriTrie> for SegTree {
    fn from(iri_trie: &IriTrie) -> Self {
        let mut res = SegTree {
            arena: MTArena::new(),
            root_id: 0,
        };

        {
            let arena_arc = res.arena.get_arena_arc();
            let mut arena_write = arena_arc.write().unwrap();

            arena_write.add_new_node(
                SegNodeData {
                    value: 0,
                    children: BTreeMap::new(),
                },
                None,
            );
        }

        res.from_aux(iri_trie);

        return res;
    }
}

impl SegTree {
    pub fn pp(&self, print_value: bool) -> String {
        let mut s = "".to_string();

        let arena_arc = self.arena.get_arena_arc();
        let arena_read = arena_arc.read().unwrap();

        let mut root_children = arena_read
            .get_node_arc(self.root_id)
            .unwrap()
            .read()
            .unwrap()
            .payload
            .children
            .iter()
            .map(|(seg, id)| (seg.clone(), *id))
            .collect::<Vec<_>>();
        root_children.sort_by(|(ch1, _), (ch2, _)| ch1.cmp(ch2));

        let mut stack = Vec::from(
            root_children
                .iter()
                .enumerate()
                .map(|(i, (seg, id))| (seg.clone(), *id, 0, i != 0))
                .rev()
                .collect::<Vec<_>>(),
        );

        while stack.len() > 0 {
            let (seg, cur_node_id, indent, new_line) = stack.pop().unwrap();
            let cur_node_arc = arena_read.get_node_arc(cur_node_id).unwrap();
            let cur_node_read = cur_node_arc.read().unwrap();
            let value = cur_node_read.payload.value.clone();
            let nr_children = cur_node_read.children_ids.len();

            if new_line {
                s.push('\n');
                s.push_str(&" ".repeat(indent));
            }
            s.push_str(&seg);

            if print_value {
                s.push_str(&format!("  {:?}", value));
            }

            let child_new_line = print_value || nr_children > 1;

            let mut children = cur_node_read.payload.children.iter().collect::<Vec<_>>();
            children.sort_by(|(ch1, _), (ch2, _)| ch2.cmp(ch1));

            for (seg, child_id) in children {
                stack.push((seg.clone(), *child_id, indent + 1, child_new_line));
                continue;
            }
        }
        s.push('\n');
        s
    }

    //fn from_aux(&mut self, node_id: &IriTrie, word_acc: String, prev_str: &str) {
    fn from_aux(&mut self, iri_trie: &IriTrie) {
        // seg_tree node id, iri_trie node id, word_acc, prev_str
        let mut stack = Vec::from([(
            self.root_id,
            iri_trie.root_id,
            "".to_string(),
            "".to_string(),
        )]);

        while let Some((seg_id, iri_id, word_acc, prev_str)) = stack.pop() {
            let (iri_children, iri_value) = {
                let iri_arena_arc = iri_trie.arena.get_arena_arc();
                let iri_arena_read = iri_arena_arc.read().unwrap();
                let iri_node_arc = iri_arena_read.get_node_arc(iri_id).unwrap();
                let iri_node_read = iri_node_arc.read().unwrap();
                let payload = iri_node_read.payload.clone();
                (payload.children, payload.value)
            };

            if iri_children.is_empty() {
                if !word_acc.is_empty() {
                    {
                        let seg_arena_arc = self.arena.get_arena_arc();
                        let mut seg_arena_write = seg_arena_arc.write().unwrap();

                        let new_seg_id = seg_arena_write.add_new_node(
                            SegNodeData {
                                value: match iri_value {
                                    Some(stats) => stats.desc,
                                    None => 0,
                                },
                                children: BTreeMap::new(),
                            },
                            Some(seg_id),
                        );

                        let seg_node_arc = seg_arena_write.get_node_arc(seg_id).unwrap();
                        let mut seg_node_write = seg_node_arc.write().unwrap();
                        seg_node_write.payload.children.insert(word_acc, new_seg_id);
                    }
                }
                continue;
            }

            for (c, child_iri_id) in iri_children {
                if ['/', '#'].contains(&c) {
                    let ns_cand = format!("{}{word_acc}{c}", prev_str.clone());
                    let url_obj = Url::parse(ns_cand.as_str());

                    // this is a URL and the kind we want
                    if url_obj.is_ok() && url_obj.unwrap().has_host() {
                        let seg_arena_arc = self.arena.get_arena_arc();
                        let mut seg_arena_write = seg_arena_arc.write().unwrap();

                        let new_seg_id = seg_arena_write.add_new_node(
                            SegNodeData {
                                value: match iri_value {
                                    Some(stats) => stats.desc,
                                    None => 0,
                                },
                                children: BTreeMap::new(),
                            },
                            Some(seg_id),
                        );

                        let seg_node_arc = seg_arena_write.get_node_arc(seg_id).unwrap();
                        let mut seg_node_write = seg_node_arc.write().unwrap();
                        seg_node_write
                            .payload
                            .children
                            .insert(format!("{word_acc}{c}"), new_seg_id);

                        stack.push((
                            new_seg_id,
                            child_iri_id,
                            "".to_string(),
                            format!("{}{word_acc}{c}", prev_str.clone()),
                        ));
                        continue;
                    }
                }
                stack.push((
                    seg_id,
                    child_iri_id,
                    format!("{word_acc}{c}"),
                    prev_str.clone(),
                ));
            }
        }
    }

    pub fn infer_namespaces(&self) -> (Vec<(String, usize, NamespaceSource)>, Vec<String>) {
        let mut h: BTreeSet<NamespaceCandidate> = BTreeSet::new();
        let mut gbg_collected: Vec<String> = Vec::new();
        let MIN_NS_SIZE = 1000;
        let MIN_DOMAIN_OCCURS = 100;

        let arena_arc = self.arena.get_arena_arc();
        let arena_read = arena_arc.read().unwrap();
        let node_arc = arena_read.get_node_arc(self.root_id).unwrap();
        let node_read = node_arc.read().unwrap();

        // self is empty string root node
        for (child_ns, child_id) in node_read.payload.children.clone() {
            let child_node_arc = arena_read.get_node_arc(child_id).unwrap();
            let child_node_read = child_node_arc.read().unwrap();
            let value = child_node_read.payload.value;

            if value < MIN_DOMAIN_OCCURS {
                gbg_collected.push(child_ns.to_string());
            }

            // include only children worthy of being namespaces
            if value > MIN_NS_SIZE {
                h.insert(NamespaceCandidate {
                    size: value,
                    children: node_read.children_ids.len(),
                    namespace: child_ns,
                    id: child_id,
                });
            }
        }

        self.infer_namespaces_aux(&mut h, MIN_NS_SIZE);

        let inferred = h
            .iter()
            .map(|ns| (ns.namespace.clone(), ns.size, NamespaceSource::Inference))
            .collect();

        return (inferred, gbg_collected);
    }

    fn infer_namespaces_aux(&self, h: &mut BTreeSet<NamespaceCandidate>, MIN_NS_SIZE: usize) {
        let MAX_NS = 5;
        let mut expanded = 0;
        let mut added = true;

        let arena_arc = self.arena.get_arena_arc();
        let arena_read = arena_arc.read().unwrap();

        while added && (expanded < MAX_NS) {
            added = false;
            let h_len = h.len();
            let mut found = false;

            match h
                .drain_filter(|item| {
                    if !found {
                        let node_arc = arena_read.get_node_arc(item.id).unwrap();
                        let node_read = node_arc.read().unwrap();

                        let suitable = node_read
                            .payload
                            .children
                            .iter()
                            .filter(|(_, n)| item.size > MIN_NS_SIZE)
                            .collect::<Vec<_>>();
                        if !suitable.is_empty() && ((suitable.len() + h_len) <= MAX_NS) {
                            found = true;
                            return true;
                        }
                    }
                    return false;
                })
                .collect::<Vec<_>>()
                .first()
                .cloned()
            {
                Some(parent) => {
                    h.remove(&parent);
                    expanded -= 1;

                    let parent_arc = arena_read.get_node_arc(parent.id).unwrap();
                    let parent_read = parent_arc.read().unwrap();

                    for (seg, id) in parent_read.payload.children.clone() {
                        let node_arc = arena_read.get_node_arc(id).unwrap();
                        let node_read = node_arc.read().unwrap();

                        if node_read.payload.value > MIN_NS_SIZE {
                            expanded += 1;
                            added = true;
                            h.insert(NamespaceCandidate {
                                id,
                                size: node_read.payload.value,
                                children: node_read.children_ids.len(),
                                namespace: format!("{}{seg}", parent.namespace),
                            });
                        }
                    }
                }
                None => return,
            }
        }
    }
}

#[derive(Clone, Debug, Eq)]
pub struct NamespaceCandidate {
    size: usize,
    children: usize,
    namespace: String,
    id: usize,
}

impl NamespaceCandidate {
    pub fn could_be_ns(&self, MIN_NS_SIZE: usize) -> bool {
        self.size >= MIN_NS_SIZE
    }
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn from_test() {
        let mut iri_trie = IriTrie::new();
        iri_trie.insert("http://www.example.com/path/1/more", Default::default());
        iri_trie.insert("http://www.example.pt/2", Default::default());
        iri_trie.insert("http://www.example.com/path/2", Default::default());

        let seg_tree = SegTree::from(&iri_trie);

        let arena_arc = seg_tree.arena.get_arena_arc();
        let arena_read = arena_arc.read().unwrap();
        let walk = arena_read.tree_walk_bfs(seg_tree.root_id).unwrap();

        let mut v = BTreeSet::new();

        for id in walk {
            let node_arc = arena_read.get_node_arc(id).unwrap();
            let node_read = node_arc.read().unwrap();
            for (seg, _) in node_read.payload.children.clone() {
                v.insert(seg);
            }
        }

        assert_eq!(v.len(), 6);
        assert!(v.contains("http://www.example.com/"));
        assert!(v.contains("http://www.example.pt/"));
        assert!(v.contains("path/"));
        assert!(v.contains("1/"));
        assert!(v.contains("2"));
        assert!(v.contains("more"));
    }
}
