use std::collections::BTreeMap;

use crate::iri_trie::{IriTrie, Stats};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SegTree {
    pub value: Option<Stats>,
    pub children: BTreeMap<String, SegTree>,
}

impl SegTree {
    fn from_aux(&mut self, iri_trie: IriTrie, word_acc: String) {
        for (c, node) in iri_trie.children {
            if ['/', '#'].contains(&c) {
                let sub_tree = SegTree {
                    children: BTreeMap::new(),
                    value: match node.value {
                        Some(stats) => Some(stats.desc),
                        None => None,
                    },
                };
                self.children.insert(format!("{word_acc}{c}"), sub_tree);
                self.from_aux(node, "".to_string());
            } else {
                self.from_aux(node, format!("{word_acc}{c}"));
            }
        }
    }
}

impl From<IriTrie> for SegTree {
    fn from(iri_trie: IriTrie) -> Self {
        let mut res = SegTree {
            value: None,
            children: BTreeMap::new(),
        };

        res.from_aux(iri_trie, "".to_string());

        return res;
    }
}
