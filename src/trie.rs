use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Debug,
    sync::{Arc, RwLock},
};

use r3bl_rs_utils::{tree_memory_arena::MTArena, Arena};

#[derive(Debug, Clone)]
pub struct NodeData<T: Clone + Send + Debug + Sync> {
    pub value: Option<T>,
    pub children: BTreeMap<char, usize>,
    pub is_terminal: bool,
}

#[derive(Debug)]
pub struct Trie<T: Clone + Send + Debug + Sync + 'static> {
    pub arena: MTArena<NodeData<T>>,
    pub root_id: usize,
}

impl<T: Clone + Send + Debug + Sync> Trie<T> {
    pub fn new() -> Self {
        let arena = MTArena::new();
        let arena_arc = arena.get_arena_arc();
        let mut arena_write = arena_arc.write().unwrap();

        let root_id = arena_write.add_new_node(
            NodeData {
                value: None,
                children: BTreeMap::new(),
                is_terminal: false,
            },
            None,
        );
        Self { arena, root_id }
    }

    pub fn count_nodes(&self) -> u32 {
        self.iter_nodes().count() as u32
    }

    pub fn count_terminals(&self) -> u32 {
        self.iter().count() as u32
    }

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
            .map(|(ch, id)| (*ch, *id))
            .collect::<Vec<_>>();
        root_children.sort_by(|(ch1, _), (ch2, _)| ch1.cmp(ch2));

        let mut stack = Vec::from(
            root_children
                .iter()
                .enumerate()
                .map(|(i, (ch, id))| (*ch, *id, 0, i != 0))
                .rev()
                .collect::<Vec<_>>(),
        );

        while stack.len() > 0 {
            let (ch, cur_node_id, indent, new_line) = stack.pop().unwrap();
            let cur_node_arc = arena_read.get_node_arc(cur_node_id).unwrap();
            let cur_node_read = cur_node_arc.read().unwrap();
            let value = cur_node_read.payload.value.clone();
            let nr_children = cur_node_read.children_ids.len();
            let is_terminal = cur_node_read.payload.is_terminal;

            if new_line {
                s.push('\n');
                s.push_str(&" ".repeat(indent));
            }
            s.push(ch);
            if is_terminal {
                s.push('·');
            }
            if print_value && value.is_some() {
                s.push_str(&format!(
                    "{}{:?}",
                    if is_terminal { " " } else { "  " },
                    value
                ));
            }

            let child_new_line = (print_value && value.is_some()) || nr_children > 1 || is_terminal;

            let mut children = cur_node_read.payload.children.iter().collect::<Vec<_>>();
            children.sort_by(|(ch1, _), (ch2, _)| ch2.cmp(ch1));

            for (ch, child_id) in children {
                stack.push((*ch, *child_id, indent + 1, child_new_line));
                continue;
            }
        }
        s.push('\n');
        s
    }

    pub fn insert(&self, key: &str, value: T) {
        self.insert_fn(
            key,
            value,
            &InsertFnVisitors {
                node: None,
                terminal: None,
            },
        )
    }

    pub fn insert_fn(&self, key: &str, value: T, visitors: &InsertFnVisitors<T>) {
        let arena_arc = self.arena.get_arena_arc();

        let mut cur_node_id = self.root_id;

        let mut rest = key.chars();
        let mut next_char = rest.next();

        // find last node matching key chars
        while let Some(ch) = next_char {
            let arena_read = arena_arc.read().unwrap();
            let node_arc = arena_read.get_node_arc(cur_node_id).unwrap();
            let node_read = node_arc.read().unwrap();
            if !node_read.payload.children.contains_key(&ch) {
                break;
            }
            cur_node_id = node_read.payload.children[&ch];
            next_char = rest.next();
        }

        // add new nodes for remaining chars
        while let Some(ch) = next_char {
            next_char = rest.next();

            let mut arena_write = arena_arc.write().unwrap();
            let new_node_id = arena_write.add_new_node(
                NodeData {
                    value: None,
                    children: BTreeMap::new(),
                    is_terminal: false,
                },
                Some(cur_node_id),
            );
            {
                let node_arc = arena_write.get_node_arc(cur_node_id).unwrap();
                let node_data = &mut node_arc.write().unwrap().payload;
                node_data.children.insert(ch, new_node_id);
            }
            cur_node_id = new_node_id;
        }

        // set value on last node
        {
            let arena_write = arena_arc.write().unwrap();
            let node_arc = arena_write.get_node_arc(cur_node_id).unwrap();
            let mut node_write = node_arc.write().unwrap();
            node_write.payload.is_terminal = true;
            node_write.payload.value = Some(value);
        }

        // call terminal visitor on last node
        if let Some(visitor) = &visitors.terminal {
            visitor(self, cur_node_id);
        }

        // call node visitor on all nodes in path
        if let Some(visitor) = &visitors.node {
            let mut cur_node_id = cur_node_id;
            while let Some(parent_id) = {
                let arena_read = arena_arc.read().unwrap();
                let node_arc = arena_read.get_node_arc(cur_node_id).unwrap();
                let node_read = node_arc.read().unwrap();
                node_read.parent_id
            } {
                visitor(self, cur_node_id);
                cur_node_id = parent_id;
            }
        }

        // call node visitor on root node
        if let Some(visitor) = &visitors.node {
            visitor(self, self.root_id);
        }
    }

    pub fn remove(&mut self, key: &str, remove_subtree: bool) -> Option<T> {
        self.remove_fn(
            key,
            remove_subtree,
            None::<&dyn Fn(&Trie<T>, usize, char) -> ()>,
        )
    }

    pub fn remove_fn<U>(
        &self,
        key: &str,
        remove_subtree: bool,
        cb: Option<&dyn Fn(&Trie<T>, usize, char) -> U>,
    ) -> Option<T> {
        let arena_arc = self.arena.get_arena_arc();

        let mut cur_node_id = self.root_id;
        let mut parent_id = self.root_id;
        let mut last_terminal: Option<(usize, char, String)> = None;

        let mut rest = key.chars();
        let mut next_char = rest.next();
        let mut str_acc = String::new();

        // find last node matching key chars
        while let Some(ch) = next_char {
            str_acc.push(ch);
            let arena_read = arena_arc.read().unwrap();
            let node_arc = arena_read.get_node_arc(cur_node_id).unwrap();
            let node_read = node_arc.read().unwrap();
            if !node_read.payload.children.contains_key(&ch) {
                break;
            }
            if node_read.payload.is_terminal || node_read.payload.children.len() > 1 {
                last_terminal = Some((cur_node_id, ch, str_acc.clone()));
            }
            parent_id = cur_node_id;
            cur_node_id = node_read.payload.children[&ch];
            next_char = rest.next();
        }

        // if no node matches key, return None
        if next_char.is_some() {
            return None;
        }
        let (value, children) = {
            let arena_read = arena_arc.read().unwrap();
            let node_arc = arena_read.get_node_arc(cur_node_id).unwrap();
            let node_read = node_arc.read().unwrap();
            (
                node_read.payload.value.clone(),
                node_read.payload.children.clone(),
            )
        };
        let mut path_str = key.to_string();

        if !remove_subtree && children.len() > 0 {
            // just remove node value and mark as non terminal
            {
                let arena_write = arena_arc.write().unwrap();
                let node_arc = arena_write.get_node_arc(cur_node_id).unwrap();
                let mut node_write = node_arc.write().unwrap();
                node_write.payload.value = None;
                node_write.payload.is_terminal = false;
            }
        } else {
            // remove_subtree or node has no children, remove last terminal subtree
            if let Some((lt_id, lt_ch, lt_str)) = last_terminal {
                {
                    let arena_write = arena_arc.write().unwrap();
                    let node_arc = arena_write.get_node_arc(lt_id).unwrap();
                    let mut node_write = node_arc.write().unwrap();
                    node_write.payload.children.remove(&lt_ch).unwrap();
                };

                arena_arc.write().unwrap().delete_node(cur_node_id);

                path_str = lt_str;
                parent_id = lt_id;
            }
        }

        // call node visitor on all nodes in path
        if let Some(visitor) = cb {
            let mut chars = path_str.chars().rev();
            while let Some(ch) = chars.next() {
                visitor(self, parent_id, ch);
                parent_id = {
                    let arena_read = arena_arc.read().unwrap();
                    let node_arc = arena_read.get_node_arc(parent_id).unwrap();
                    let node_read = node_arc.read().unwrap();
                    if node_read.parent_id.is_none() {
                        break;
                    }
                    node_read.parent_id.unwrap()
                };
            }
        }

        value
    }

    pub fn is_terminal(&self, node_id: usize) -> bool {
        let arena_arc = self.arena.get_arena_arc();
        let arena_read = arena_arc.read().unwrap();
        let node_arc = arena_read.get_node_arc(node_id).unwrap();
        let node_read = node_arc.read().unwrap();
        node_read.payload.is_terminal
    }

    pub fn contains_key(&self, s: &str) -> bool {
        self.find(s, true).is_some()
    }

    pub fn find(&self, s: &str, must_be_terminal: bool) -> Option<(NodeData<T>, String)> {
        let lpo = LongestPrefOpts {
            must_be_terminal,
            must_match_fully: true,
        };
        self.longest_prefix_aux(s, lpo)
    }

    /// Returns the node corresponding to the longest prefix and the longest prefix String
    pub fn longest_prefix(&self, s: &str, must_be_terminal: bool) -> Option<(NodeData<T>, String)> {
        let lpo = LongestPrefOpts {
            must_be_terminal,
            must_match_fully: false,
        };
        self.longest_prefix_aux(s, lpo)
    }

    fn longest_prefix_aux(&self, s: &str, lpo: LongestPrefOpts) -> Option<(NodeData<T>, String)> {
        let mut last_term = None;
        let mut str_acc = "".to_string();
        let mut str_left = s;
        let mut cur_id = self.root_id;

        let arena_arc = self.arena.get_arena_arc();
        let arena_read = arena_arc.read().unwrap();

        loop {
            let cur_node = {
                let node_arc = arena_read.get_node_arc(cur_id).unwrap();
                let node_read = node_arc.read().unwrap();
                node_read.payload.clone()
            };

            // search string is over
            if str_left.is_empty() {
                if !cur_node.is_terminal && lpo.must_be_terminal {
                    if lpo.must_match_fully {
                        return None;
                    } else {
                        return last_term;
                    }
                }
                return Some((cur_node.clone(), str_acc.clone()));
            }

            let first_char = str_left.chars().next().unwrap();
            str_left = &str_left[first_char.len_utf8()..];

            let next_id = cur_node.children.get(&first_char);

            // there are no more children
            if cur_node.children.is_empty() || next_id.is_none() {
                if lpo.must_match_fully {
                    return None;
                }
                if !cur_node.is_terminal && lpo.must_be_terminal {
                    return last_term;
                }
                return Some((cur_node, str_acc.clone()));
            }

            if cur_node.is_terminal {
                last_term = Some((cur_node.clone(), str_acc.clone()));
            }

            cur_id = *next_id.unwrap();
            str_acc = format!("{str_acc}{first_char}");
        }
    }
}

pub struct InsertFnVisitors<'a, T: Clone + Send + Sync + Debug + 'static> {
    pub node: Option<&'a dyn Fn(&Trie<T>, usize)>,
    pub terminal: Option<&'a dyn Fn(&Trie<T>, usize)>,
}

struct LongestPrefOpts {
    must_be_terminal: bool,
    must_match_fully: bool,
}

pub struct NodeIterNodes<T: Debug + Sync + Send + Clone> {
    arena_arc: Arc<RwLock<Arena<NodeData<T>>>>,
    queue: VecDeque<(String, usize)>,
}

/// Iterate terminal nodes in the trie
impl<T: Debug + Sync + Send + Clone> Trie<T> {
    pub fn iter_nodes(&self) -> NodeIterNodes<T> {
        let arena_arc = self.arena.get_arena_arc();

        NodeIterNodes {
            arena_arc,
            queue: VecDeque::from([("".to_string(), self.root_id)]),
        }
    }
}

impl<T: Debug + Send + Sync + Clone> Iterator for NodeIterNodes<T> {
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
        return Some((s, node_read.payload.clone()));
    }
}

pub struct NodeIter<T: Debug + Sync + Send + Clone> {
    arena_arc: Arc<RwLock<Arena<NodeData<T>>>>,
    queue: VecDeque<(String, usize)>,
}

/// Iterate terminal nodes in the trie
impl<T: Debug + Sync + Send + Clone> Trie<T> {
    pub fn iter(&self) -> NodeIter<T> {
        let arena_arc = self.arena.get_arena_arc();

        NodeIter {
            arena_arc,
            queue: VecDeque::from([("".to_string(), self.root_id)]),
        }
    }
}

impl<T: Debug + Send + Sync + Clone> Iterator for NodeIter<T> {
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
        if node_read.payload.is_terminal {
            return Some((s, node_read.payload.clone()));
        }
        return self.next();
    }
}

#[cfg(test)]
mod tests {
    use crate::iri_trie::{inc_own, update_stats};

    use super::*;

    #[test]
    fn pp() {
        let mut t = Trie::new();
        t.insert("abc", 1);
        t.insert("de", 2);
        t.insert("df", 3);
        t.insert("abcxy", 3);
        assert_eq!(t.pp(false), "abc·\n   xy·\nd\n e·\n f·\n")
    }
    #[test]
    fn pp_print_value() {
        let mut t = Trie::new();
        t.insert("abc", 1);
        t.insert("ade", 2);

        assert_eq!(t.pp(true), "a\n bc· Some(1)\n de· Some(2)\n");
    }

    #[test]
    fn insert_1() {
        let mut t = Trie::new();
        t.insert("a", 1);
        assert_eq!(t.pp(true), "a· Some(1)\n");
    }

    #[test]
    fn insert_2() {
        let mut t = Trie::new();
        t.insert("a", 1);
        t.insert("abc", 2);
        t.insert("bce", 3);
        t.insert("ac", 4);
        assert_eq!(t.pp(false), "a·\n bc·\n c·\nbce·\n");
    }

    fn ins_vis() -> InsertFnVisitors<'static, usize> {
        InsertFnVisitors {
            node: Some(&|trie: &Trie<usize>, node_id: usize| {
                let arena_arc = trie.arena.get_arena_arc();
                let arena_read = arena_arc.read().unwrap();
                let node_arc = arena_read.get_node_arc(node_id).unwrap();

                let is_terminal = node_arc.read().unwrap().payload.is_terminal;

                let children_ids = node_arc
                    .read()
                    .unwrap()
                    .payload
                    .children
                    .values()
                    .map(|id| *id)
                    .collect::<Vec<usize>>();

                let new_value: usize;
                {
                    new_value = if is_terminal { 1 } else { 0 }
                        + children_ids
                            .iter()
                            .map(|id| {
                                trie.arena
                                    .get_arena_arc()
                                    .read()
                                    .unwrap()
                                    .get_node_arc(*id)
                                    .unwrap()
                                    .read()
                                    .unwrap()
                                    .payload
                                    .value
                                    .unwrap()
                            })
                            .sum::<usize>();
                }
                node_arc.write().unwrap().payload.value = Some(new_value);
            }),
            terminal: Some(&|trie: &Trie<usize>, node_id: usize| {
                trie.arena
                    .get_arena_arc()
                    .read()
                    .unwrap()
                    .get_node_arc(node_id)
                    .unwrap()
                    .write()
                    .unwrap()
                    .payload
                    .value = Some(1);
            }),
        }
    }

    #[test]
    fn insert_fn() {
        let visitors = ins_vis();

        let mut trie = Trie::new();
        trie.insert_fn("abc", 0, &visitors);
        trie.insert_fn("ade", 0, &visitors);
        trie.insert_fn("ab", 0, &visitors);
    }

    #[test]
    fn delete_node_1() {
        let mut t = Trie::new();
        t.insert("a", 1);
        t.insert("ab", 2);
        t.remove("ab", false);
        assert_eq!(t.pp(false), "a·\n");
    }

    #[test]
    fn delete_node_2() {
        let mut t = Trie::new();
        t.insert("a", 1);
        t.insert("abcde", 2);
        t.remove("ab", true);
        assert_eq!(t.pp(false), "a·\n");
    }

    #[test]
    fn delete_node_3() {
        let mut t = Trie::new();
        t.insert("a", 1);
        t.insert("abcde", 2);
        let res = t.remove("axyz", true);
        assert!(res.is_none());
    }

    #[test]
    fn delete_node_4() {
        let mut t = Trie::new();
        t.insert("a", 1);
        t.insert("abc", 2);
        t.insert("abcde", 3);
        t.remove("abc", false);
        assert_eq!(t.pp(false), "a·\n bcde·\n");
    }

    fn upd_stats_visitor(trie: &mut Trie<usize>, parent_id: usize, ch: char) {
        let visitors = ins_vis();
        visitors.node.unwrap()(trie, parent_id);
    }

    #[test]
    fn remove_callback() {
        let mut t = Trie::new();
        let visitors = ins_vis();
        t.insert_fn("a", 1, &visitors);
        t.insert_fn("abcde", 2, &visitors);
        t.insert_fn("abc", 3, &visitors);
        t.remove_fn("abcd", true, Some(&upd_stats_visitor));

        assert_eq!(t.pp(true), "a· Some(2)\n b  Some(1)\n  c· Some(1)\n");
    }

    #[test]
    fn contains_key() {
        let mut t = Trie::new();
        t.insert("a", 1);
        assert!(t.contains_key("a"));

        t.insert("abc", 2);
        assert!(!t.contains_key("b"));
        assert!(t.contains_key("abc"));
    }

    #[test]
    fn longest_prefix() {
        let mut t = Trie::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is more words", 3);
        let must_be_terminal = false;
        let (_, res) = t
            .longest_prefix("this is more wo", must_be_terminal)
            .unwrap();
        let expected: Vec<char> = "this is more wo".chars().collect();
        assert_eq!(res.chars().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn longest_prefix_no_full_match() {
        let mut t = Trie::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is more words", 3);
        let must_be_terminal = false;
        let (_, res) = t.longest_prefix("this is weeks", must_be_terminal).unwrap();
        let expected: Vec<char> = "this is w".chars().collect();
        assert_eq!(res.chars().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn longest_prefix_terminal() {
        let mut t = Trie::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is more words", 3);
        let must_be_terminal = true;
        let res = t.longest_prefix("this is more wo", must_be_terminal);
        let expected = "this is more";
        let (_, s) = res.unwrap();
        assert_eq!(s, expected);
    }

    #[test]
    fn longest_prefix_fail() {
        let mut t = Trie::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is more words", 3);
        let must_be_terminal = true;
        let res = t.longest_prefix("this is", must_be_terminal);
        assert!(res.is_none());
    }

    #[test]
    fn find() {
        let mut t = Trie::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is even more", 3);
        let must_be_terminal = true;
        let (n, s) = t.find("this is more", must_be_terminal).unwrap();
        assert_eq!(s, "this is more");
        assert_eq!(n.value.unwrap(), 2);
    }

    #[test]
    fn find_non_terminal() {
        let mut t = Trie::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is even more", 3);
        let must_be_terminal = false;
        let (n, s) = t.find("this is m", must_be_terminal).unwrap();
        assert_eq!(s, "this is m");
        assert!(n.value.is_none());
        assert!(!n.is_terminal);
    }

    #[test]
    fn find_longer() {
        let mut t = Trie::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is even more", 3);
        let must_be_terminal = false;
        let res = t.find("this is more rabelz", must_be_terminal);
        assert!(res.is_none());
    }

    #[test]
    fn find_terminal() {
        let mut t = Trie::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is even more", 3);
        let must_be_terminal = true;
        let (node, s) = t.find("this is more", must_be_terminal).unwrap();
        assert_eq!(s, "this is more");
        assert!(node.is_terminal);
        assert_eq!(node.value.unwrap(), 2);
    }

    #[test]
    fn find_terminal_fail() {
        let mut t = Trie::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is even more", 3);
        let must_be_terminal = true;
        let pref = t.find("this is more wo", must_be_terminal);
        assert!(pref.is_none())
    }

    #[test]
    fn iter() {
        let mut t = Trie::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is even more", 3);
        let all_strs: Vec<_> = t.iter().map(|pair| pair.0).collect();
        assert!(all_strs.contains(&"this is words".to_string()));
        assert!(all_strs.contains(&"this is more".to_string()));
        assert!(all_strs.contains(&"this is even more".to_string()));
        assert_eq!(all_strs.len(), 3);
    }

    #[test]
    fn count_terminals_test() {
        let mut t = Trie::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is even more", 3);
        assert_eq!(t.count_terminals(), 3);
    }
}
