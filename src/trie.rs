use std::{
    borrow::Borrow,
    collections::{BTreeMap, VecDeque},
    fmt::Debug,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<T: Clone + Debug> {
    pub value: Option<T>,
    pub is_terminal: bool,
    pub children: BTreeMap<char, Node<T>>,
}

impl<T: Debug + Clone> Node<T> {
    pub fn pp(&self, print_value: bool) -> String {
        let mut res = "".to_string();

        let mut root_children = self.children.iter().collect::<Vec<_>>();
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
            let (ch, node, indent, new_line) = stack.pop().unwrap();

            if new_line {
                res.push('\n');
                res.push_str(&" ".repeat(indent));
            }
            res.push(*ch);
            if node.is_terminal {
                res.push('·');
            }
            if print_value && node.value.is_some() {
                res.push_str(
                    format!(
                        "{}{:?}",
                        if node.is_terminal { " " } else { "  " },
                        node.value.as_ref().unwrap()
                    )
                    .as_str(),
                );
            }

            let child_new_line = (print_value && node.value.is_some())
                || node.children.len() > 1
                || node.is_terminal;

            let mut children = node.children.iter().collect::<Vec<_>>();
            children.sort_by(|(ch1, _), (ch2, _)| ch2.cmp(ch1));

            for (ch, node) in children {
                stack.push((ch, node, indent + 1, child_new_line));
            }
        }
        res.push('\n');
        res
    }

    pub fn new() -> Node<T> {
        Node {
            value: None,
            is_terminal: false,
            children: BTreeMap::new(),
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    pub fn count_nodes(&self) -> u32 {
        return self
            .children
            .iter()
            .fold(self.children.len() as u32, |acc, (_, v)| {
                acc + v.count_nodes()
            });
    }

    pub fn count_terminals(&self) -> u32 {
        return self.children.iter().fold(0, |acc, (_, v)| {
            acc + if v.is_terminal { 1 } else { 0 } + v.count_terminals()
        });
    }

    pub fn get_mut(&mut self, ch: char) -> Option<&mut Node<T>> {
        return self.children.get_mut(&ch);
    }

    pub fn insert<S: ?Sized>(&mut self, key: &S, value: T)
    where
        S: Borrow<str>,
    {
        self.insert_fn(
            key,
            value,
            &InsertFnVisitors {
                node: None::<&dyn Fn(&mut Node<T>)>,
                terminal: None::<&dyn Fn(&mut Node<T>)>,
            },
        )
    }

    pub fn insert_fn<S: ?Sized>(&mut self, key: &S, value: T, visitors: &InsertFnVisitors<T>)
    where
        S: Borrow<str>,
    {
        let k: &str = key.borrow();

        if k.is_empty() {
            self.is_terminal = true;
            if let Some(f) = visitors.terminal {
                f(self);
            } else {
                self.value = Some(value);
            }
            return;
        }

        let first_char = k.chars().next().unwrap();
        let rest = &k[first_char.len_utf8()..];

        if self.children.contains_key(&first_char) {
            let child_node = self.children.get_mut(&first_char).unwrap();
            child_node.insert_fn(rest, value, visitors);
            if let Some(f) = visitors.node {
                f(self);
            }
            return;
        }

        let mut new_node = Node {
            is_terminal: false,
            children: BTreeMap::new(),
            value: None,
        };
        new_node.insert_fn(rest, value, visitors);
        self.children.insert(first_char, new_node);
        if let Some(f) = visitors.node {
            f(self);
        }
    }

    pub fn remove<S: ?Sized>(&mut self, key: &S, remove_subtree: bool) -> Option<T>
    where
        S: Borrow<str>,
    {
        self.remove_fn(
            key,
            remove_subtree,
            None::<&dyn Fn(&mut Node<T>, char, Option<&Node<T>>) -> u32>,
        )
    }
    pub fn remove_fn<U, S: ?Sized>(
        &mut self,
        str_left: &S,
        remove_subtree: bool,
        cb: Option<&dyn Fn(&mut Node<T>, char, Option<&Node<T>>) -> U>,
    ) -> Option<T>
    where
        S: Borrow<str>,
    {
        self.remove_fn_aux(str_left, remove_subtree, cb).0
    }

    pub fn remove_fn_aux<U, S: ?Sized>(
        &mut self,
        str_left: &S,
        remove_subtree: bool,
        cb: Option<&dyn Fn(&mut Node<T>, char, Option<&Node<T>>) -> U>,
    ) -> (Option<T>, bool)
    where
        S: Borrow<str>,
    {
        let sl: &str = str_left.borrow();
        let first_char = sl.chars().next().unwrap();
        let rest = &sl[first_char.len_utf8()..];

        if self.children.is_empty() {
            return (None, false);
        }

        if !self.children.contains_key(&first_char) {
            return (None, false);
        }

        // if we are at the end of the string, remove the node
        if rest.is_empty() {
            let sub_node = self.children.get_mut(&first_char).unwrap();
            if sub_node.children.is_empty() || remove_subtree {
                let sub_node = self.children.remove(&first_char).unwrap();
                if let Some(f) = cb {
                    f(self, first_char, Some(&sub_node));
                }
                let bubble_up = self.children.is_empty() && !self.is_terminal;
                return (sub_node.value, bubble_up);
            }

            sub_node.is_terminal = false;
            sub_node.value = None;
            return (None, false);
        }

        // if we are not at the end of the string, recurse
        let res =
            self.children
                .get_mut(&first_char)
                .unwrap()
                .remove_fn_aux(rest, remove_subtree, cb);
        if res.1 {
            let old_node = self.children.remove(&first_char).unwrap();

            // call node visitor
            if let Some(f) = cb {
                f(self, first_char, Some(&old_node));
            }
            let bubble_up = !self.is_terminal && self.children.is_empty();
            return (res.0, bubble_up);
        }

        // call node visitor
        if let Some(f) = cb {
            f(self, first_char, None);
        }
        return (res.0, false);
    }

    pub fn contains_key(&self, s: &str) -> bool {
        self.find(s, true).is_some()
    }

    pub fn find(&self, s: &str, must_be_terminal: bool) -> Option<(&Node<T>, String)> {
        let lpo = LongestPrefOpts {
            must_be_terminal,
            must_match_fully: true,
        };
        self.longest_prefix_aux(s, lpo)
    }

    /// Returns the node corresponding to the longest prefix and the longest prefix String
    pub fn longest_prefix(&self, s: &str, must_be_terminal: bool) -> Option<(&Node<T>, String)> {
        let lpo = LongestPrefOpts {
            must_be_terminal,
            must_match_fully: false,
        };
        self.longest_prefix_aux(s, lpo)
    }

    fn longest_prefix_aux(&self, s: &str, lpo: LongestPrefOpts) -> Option<(&Node<T>, String)> {
        let mut last_term = None;
        let mut str_acc = "".to_string();
        let mut str_left = s;
        let mut cur_node = self;

        loop {
            if str_left.is_empty() {
                if !cur_node.is_terminal && lpo.must_be_terminal {
                    if lpo.must_match_fully {
                        return None;
                    } else {
                        return last_term;
                    }
                }
                return Some((cur_node, str_acc.clone()));
            }

            let first_char = str_left.chars().next().unwrap();
            str_left = &str_left[first_char.len_utf8()..];

            let next_node = cur_node.children.get(&first_char);
            if cur_node.children.is_empty() || next_node.is_none() {
                if lpo.must_match_fully {
                    return None;
                }
                if !cur_node.is_terminal && lpo.must_be_terminal {
                    return last_term;
                }
                return Some((cur_node, str_acc.clone()));
            }

            if cur_node.is_terminal {
                last_term = Some((cur_node, format!("{str_acc}")));
            }

            cur_node = next_node.unwrap();
            str_acc = format!("{str_acc}{first_char}");
        }
    }

    pub fn traverse(&self, f: &impl Fn(String, &T)) {
        self.traverse_aux("".to_owned(), f, &TraverseDirection::Down)
    }
    pub fn traverse_up(&self, f: &impl Fn(String, &T)) {
        self.traverse_aux("".to_owned(), f, &TraverseDirection::Up)
    }

    fn traverse_aux(
        &self,
        str_acc: String,
        f: &impl Fn(String, &T),
        direction: &TraverseDirection,
    ) {
        if let TraverseDirection::Down = direction {
            if let Some(v) = &self.value {
                f(str_acc.clone(), v);
            }
            for (c, n) in self.children.iter() {
                n.traverse_aux(format!("{}{}", str_acc, c), f, direction);
            }
        } else {
            for (c, n) in self.children.iter().rev() {
                n.traverse_aux(format!("{}{}", str_acc, c), f, direction);
            }
            if let Some(v) = &self.value {
                f(str_acc.clone(), v);
            }
        }
    }

    pub fn traverse_mut(&mut self, f: &impl Fn(String, &mut T)) {
        self.traverse_mut_aux("".to_owned(), f)
    }

    fn traverse_mut_aux(&mut self, str_acc: String, f: &impl Fn(String, &mut T)) {
        if let Some(v) = &mut self.value {
            f(str_acc.clone(), v);
        }
        for (c, n) in &mut self.children.iter_mut() {
            n.traverse_mut_aux(format!("{}{}", str_acc, c), f);
        }
    }
}

pub struct InsertFnVisitors<'a, T: Debug + Clone> {
    pub node: Option<&'a dyn Fn(&mut Node<T>)>,
    pub terminal: Option<&'a dyn Fn(&mut Node<T>)>,
}
enum MatchType {
    FullQuery,
    FullPath,
    Exact,
    Loose,
}

struct LongestPrefOpts {
    must_be_terminal: bool,
    must_match_fully: bool,
}

type FindResults<'a, T> = Option<(&'a Node<T>, String)>;

#[derive(Copy, Clone)]
pub enum TraverseDirection {
    Down,
    Up,
}

pub struct NodeIter<'a, T: Debug + Clone> {
    queue: VecDeque<(String, &'a Node<T>)>,
}

impl<T: Debug + Clone> Node<T> {
    pub fn iter(&self) -> NodeIter<'_, T> {
        NodeIter {
            queue: VecDeque::from([("".to_string(), self)]),
        }
    }
}

impl<'a, T: Debug + Clone> Iterator for NodeIter<'a, T> {
    type Item = (String, &'a Node<T>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.queue.is_empty() {
            return None;
        }
        let (s, n) = self.queue.pop_front().unwrap();
        for (k, v) in n.children.iter() {
            self.queue.push_front((format!("{s}{k}"), &v));
        }
        if n.is_terminal {
            return Some((s, n));
        }
        return self.next();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    #[test]
    fn pp() {
        let mut t = Node::new();
        t.insert("abc", 1);
        t.insert("de", 2);
        t.insert("df", 3);
        t.insert("abcxy", 3);
        assert_eq!(t.pp(false), "abc·\n   xy·\nd\n e·\n f·\n")
    }
    #[test]
    fn pp_print_value() {
        let mut t = Node::new();
        t.insert("abc", 1);
        t.insert("ade", 2);

        assert_eq!(t.pp(true), "a\n bc· 1\n de· 2\n");
    }

    #[test]
    fn pretty_print() {
        let t: Node<u8> = Node {
            is_terminal: false,
            value: None,
            children: BTreeMap::from([
                (
                    'a',
                    Node {
                        is_terminal: true,
                        value: None,
                        children: BTreeMap::from([(
                            'b',
                            Node {
                                is_terminal: false,
                                value: None,
                                children: BTreeMap::from([(
                                    'c',
                                    Node {
                                        is_terminal: true,
                                        value: None,
                                        children: BTreeMap::new(),
                                    },
                                )]),
                            },
                        )]),
                    },
                ),
                (
                    'd',
                    Node {
                        is_terminal: true,
                        value: None,
                        children: BTreeMap::new(),
                    },
                ),
                (
                    'e',
                    Node {
                        is_terminal: true,
                        value: None,
                        children: BTreeMap::new(),
                    },
                ),
            ]),
        };
        assert_eq!(t.pp(false), "a·\n bc·\nd·\ne·\n")
    }

    #[test]
    fn insert_1() {
        let mut t = Node::new();
        t.insert("a", 1);
        assert_eq!(t.pp(true), "a· 1\n");
    }

    #[test]
    fn insert_2() {
        let mut t = Node::new();
        t.insert("a", 1);
        t.insert("abc", 2);
        t.insert("bce", 3);
        t.insert("ac", 4);
        assert_eq!(t.pp(false), "a·\n bc·\n c·\nbce·\n");
    }

    fn node_vis(node: &mut Node<usize>) {
        for child in node.children.values_mut() {
            node_vis(child);
        }
        node.value = Some(
            if node.is_terminal { 1 } else { 0 }
                + node
                    .children
                    .values()
                    .map(|child| child.value.unwrap())
                    .sum::<usize>(),
        );
    }

    fn term_vis(node: &mut Node<usize>) {
        node.value = Some(1);
    }

    fn ins_vis() -> InsertFnVisitors<'static, usize> {
        InsertFnVisitors {
            node: Some(&node_vis),
            terminal: Some(&term_vis),
        }
    }

    #[test]

    fn insert_fn() {
        let visitors = ins_vis();

        let mut trie = Node::new();
        trie.insert_fn("abc", 0, &visitors);
        trie.insert_fn("ade", 0, &visitors);
        trie.insert_fn("ab", 0, &visitors);
    }

    #[test]
    fn insert_to_empty_trie() {
        let mut t = Node::new();
        t.insert("a", 1);

        assert_eq!(t.value, None);
        assert_eq!(t.is_terminal, false);
        let subt = t.children.get(&'a').unwrap();
        assert_eq!(subt.value, Some(1));
        assert_eq!(subt.is_terminal, true);
    }

    #[test]
    fn insert_single_char_string() {
        let mut t = Node::new();
        t.insert("a", 1);
        t.insert("ab", 2);
        t.insert("c", 3);
        t.insert("de", 4);
        t.insert("df", 4);
        assert_eq!(t.pp(false), "a·\n b·\nc·\nd\n e·\n f·\n");
    }

    #[test]
    fn delete_node_1() {
        let mut t = Node::new();
        t.insert("a", 1);
        t.insert("ab", 2);
        t.remove("ab", false);
        assert_eq!(t.pp(false), "a·\n");
    }

    #[test]
    fn delete_node_2() {
        let mut t = Node::new();
        t.insert("a", 1);
        t.insert("abcde", 2);
        t.remove("ab", true);
        assert_eq!(t.pp(false), "a·\n");
    }

    #[test]
    fn delete_node_3() {
        let mut t = Node::new();
        t.insert("a", 1);
        t.insert("abcde", 2);
        let res = t.remove("axyz", true);
        assert!(res.is_none());
    }

    #[test]
    fn delete_node_4() {
        let mut t = Node::new();
        t.insert("a", 1);
        t.insert("abc", 2);
        t.insert("abcde", 3);
        t.remove("abc", false);
        assert_eq!(t.pp(false), "a·\n bcde·\n");
    }

    fn upd_stats_visitor(node: &mut Node<usize>, ch: char, _: Option<&Node<usize>>) {
        let visitors = ins_vis();
        visitors.node.unwrap()(node);
    }

    #[test]
    fn remove_callback() {
        let mut t = Node::new();
        let visitors = ins_vis();
        t.insert_fn("a", 1, &visitors);
        t.insert_fn("abcde", 2, &visitors);
        t.insert_fn("abc", 3, &visitors);
        t.remove_fn("abcd", true, Some(&upd_stats_visitor));

        assert_eq!(t.pp(true), "a· 2\n b  1\n  c· 1\n");
    }

    #[test]
    fn remove() {
        let mut t = Node::new();
        t.insert("a", 1);
        t.insert("abc", 2);
        t.insert("abcd", 3);

        // TODO remove returns true/false
        t.remove("ab", false);
        assert!(t.contains_key("a"));
        assert!(t.contains_key("abc"));
        assert!(t.contains_key("abcd"));

        t.remove("abc", true);
        assert!(t.contains_key("a"));
        assert!(!t.contains_key("abc"));
        assert!(!t.contains_key("abcd"));

        t.remove("a", false);
    }

    #[test]
    fn remove_non_terminal() {
        let mut t = Node::new();
        t.insert("a", 1);
        t.insert("abc", 2);
        t.remove("abc", false);

        assert_eq!(t.children.len(), 1);
        assert!(t.children.contains_key(&'a'));
        assert_eq!(t.children.get(&'a').unwrap().value, Some(1));
    }

    #[test]
    fn remove_subtree() {
        let mut t = Node::new();
        t.insert("a", 1);
        t.insert("abc", 2);
        t.remove("ab", true);

        assert_eq!(t.children.len(), 1);
        assert!(t.children.contains_key(&'a'));
        assert_eq!(t.children.get(&'a').unwrap().value, Some(1));
    }

    #[test]
    fn remove_non_existing() {
        let mut t = Node::new();
        t.insert("a", 1);
        t.insert("abc", 2);
        let expected = t.pp(false);
        t.remove("xyz", true);
        assert_eq!(t.pp(false), expected);
    }

    #[test]
    fn contains_key() {
        let mut t = Node::new();
        t.insert("a", 1);
        assert!(t.contains_key("a"));

        t.insert("abc", 2);
        assert!(!t.contains_key("b"));
        assert!(t.contains_key("abc"));
    }

    #[test]
    fn longest_prefix() {
        let mut t = Node::new();
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
        let mut t = Node::new();
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
        let mut t = Node::new();
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
        let mut t = Node::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is more words", 3);
        let must_be_terminal = true;
        let res = t.longest_prefix("this is", must_be_terminal);
        assert!(res.is_none());
    }

    #[test]
    fn find() {
        let mut t = Node::new();
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
        let mut t = Node::new();
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
        let mut t = Node::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is even more", 3);
        let must_be_terminal = false;
        let res = t.find("this is more rabelz", must_be_terminal);
        assert!(res.is_none());
    }

    #[test]
    fn find_terminal() {
        let mut t = Node::new();
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
        let mut t = Node::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is even more", 3);
        let must_be_terminal = true;
        let pref = t.find("this is more wo", must_be_terminal);
        assert!(pref.is_none())
    }

    #[test]
    fn iter() {
        let mut t = Node::new();
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
    fn count_nodes() {
        let mut t = Node::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is even more", 3);

        assert_eq!(t.count_nodes(), 26);
    }
}
