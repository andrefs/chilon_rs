use std::{
    borrow::Borrow,
    collections::BTreeMap,
    fmt::{Debug, Display},
    mem,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<T> {
    pub value: Option<T>,
    pub is_terminal: bool,
    pub children: BTreeMap<char, Node<T>>,
}

impl<T: Debug> Node<T> {
    pub fn pp(&self, print_value: bool) -> String {
        self.pp_fn(0, print_value)
    }

    fn pp_fn(&self, indent: u8, print_value: bool) -> String {
        let mut res = "".to_string();
        // print value
        //if print_value && self.value.is_some() {
        if self.value.is_some() {
            res.push_str(format!("  {:?}", self.value.as_ref().unwrap()).as_str());
        }
        let count = self.children.len();
        if self.children.is_empty() || self.is_terminal || count > 1 || self.value.is_some() {
            res.push('\n');
        }
        for (k, v) in self.children.iter() {
            if self.is_terminal || count > 1 {
                res.push_str(&" ".repeat(indent.into()));
            }

            res.push_str(&k.to_string());
            res.push_str(v.pp_fn(indent + 1, print_value).as_str());
        }

        return res;
    }
}

impl<T: Debug> Node<T> {
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

    pub fn insert<S: ?Sized>(&mut self, key: &S, value: T) -> Option<T>
    where
        S: Borrow<str>,
    {
        self.insert_fn(key, value, None::<&dyn Fn(&mut Node<T>) -> u32>)
    }

    pub fn insert_fn<U, S: ?Sized>(
        &mut self,
        key: &S,
        value: T,
        cb: Option<&dyn Fn(&mut Node<T>) -> U>,
    ) -> Option<T>
    where
        S: Borrow<str>,
    {
        let k: &str = key.borrow();

        if k.is_empty() {
            self.is_terminal = true;
            let old_val = mem::replace(&mut self.value, Some(value));
            if let Some(f) = cb {
                f(self);
            }

            return old_val;
        }

        let first_char = k.chars().next().unwrap();
        let rest = &k[first_char.len_utf8()..];

        if self.children.contains_key(&first_char) {
            let child_node = self.children.get_mut(&first_char).unwrap();
            let res = child_node.insert_fn(rest, value, cb);
            if let Some(f) = cb {
                f(self);
            }
            return res;
        }

        let mut new_node = Node {
            is_terminal: false,
            children: BTreeMap::new(),
            value: None,
        };
        let res = new_node.insert_fn(rest, value, cb);
        self.children.insert(first_char, new_node);
        if let Some(f) = cb {
            f(self);
        }
        res
    }

    pub fn remove<S: ?Sized>(&mut self, key: &S, remove_subtree: bool) -> bool
    where
        S: Borrow<str>,
    {
        let res = self.remove_fn(
            key,
            remove_subtree,
            None::<&dyn Fn(&mut Node<T>, char, Option<&Node<T>>) -> u32>,
        );
        res
    }

    pub fn remove_fn<U, S: ?Sized>(
        &mut self,
        str_left: &S,
        remove_subtree: bool,
        cb: Option<&dyn Fn(&mut Node<T>, char, Option<&Node<T>>) -> U>,
    ) -> bool
    where
        S: Borrow<str>,
    {
        let sl: &str = str_left.borrow();
        let first_char = sl.chars().next().unwrap();
        let rest = &sl[first_char.len_utf8()..];

        if self.children.is_empty() {
            return false;
        }

        if !self.children.contains_key(&first_char) {
            return false;
        }

        if rest.is_empty() {
            let sub_node = self.children.get_mut(&first_char).unwrap();
            if sub_node.children.is_empty() || remove_subtree {
                let sub_node = self.children.remove(&first_char).unwrap();
                if let Some(f) = cb {
                    f(self, first_char, Some(&sub_node));
                }
                let bubble_up = self.children.is_empty() && !self.is_terminal;
                return bubble_up;
            }

            if !sub_node.is_terminal {
                return false;
            }
            sub_node.is_terminal = false;
            return true;
        } else {
            let bubble_up =
                self.children
                    .get_mut(&first_char)
                    .unwrap()
                    .remove_fn(rest, remove_subtree, cb);
            if bubble_up {
                let old_node = self.children.remove(&first_char).unwrap();

                if let Some(f) = cb {
                    f(self, first_char, Some(&old_node));
                }
                let bubble_up = !self.is_terminal && self.children.is_empty();
                return bubble_up;
            } else {
                if let Some(f) = cb {
                    f(self, first_char, None);
                }
            }
            return false;
        }
    }

    pub fn contains_key(&self, s: &str) -> bool {
        self.find(s, true).is_some()
    }

    pub fn find(&self, s: &str, must_be_terminal: bool) -> Option<&Node<T>> {
        let lpo = LongestPrefOpts {
            must_be_terminal,
            must_match_fully: true,
        };
        let last_term = None;
        let res = self.longest_prefix_aux(s, "".to_string(), last_term, lpo);
        return if let Some((node, _)) = res {
            Some(node)
        } else {
            None
        };
    }

    pub fn longest_prefix(&mut self, s: &str, must_be_terminal: bool) -> String {
        let lpo = LongestPrefOpts {
            must_be_terminal,
            must_match_fully: false,
        };
        let last_term = None;

        let res = self.longest_prefix_aux(s, "".to_string(), last_term, lpo);
        return if let Some((_, prefix)) = res {
            prefix
        } else {
            "".to_owned()
        };
    }

    fn longest_prefix_aux<'a, S>(
        &'a self,
        str_left: &S,
        str_acc: String,
        mut last_terminal: FindResults<'a, T>,
        opts: LongestPrefOpts,
    ) -> FindResults<T>
    where
        S: ?Sized + Borrow<str>,
    {
        let sl: &str = str_left.borrow();

        if sl.is_empty() {
            if !self.is_terminal && opts.must_be_terminal {
                if opts.must_match_fully {
                    return None;
                } else {
                    return last_terminal;
                }
            }
            return Some((self, str_acc.clone()));
        }

        let first_char = sl.chars().next().unwrap();
        let rest = &sl[first_char.len_utf8()..];

        let next_node = self.children.get(&first_char);
        if self.children.is_empty() || next_node.is_none() {
            if !self.is_terminal && opts.must_be_terminal {
                if opts.must_match_fully {
                    return None;
                } else {
                    return last_terminal;
                }
            }
            return Some((self, str_acc.clone()));
        }

        if self.is_terminal {
            last_terminal = Some((self, format!("{str_acc}{first_char}")));
        }

        return next_node.unwrap().longest_prefix_aux(
            rest,
            format!("{str_acc}{first_char}"),
            last_terminal,
            opts,
        );
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

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
        assert_eq!(t.pp(false), "a\n bc\nd\ne\n")
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
        println!("{:#?}", t);
        assert_eq!(t.pp(false), "a\n b\nc\nde\n")
    }

    #[test]
    fn remove() {
        let mut t = Node::new();
        t.insert("a", 1);
        t.insert("abc", 2);
        t.insert("abcd", 3);

        assert!(!t.remove("ab", false));
        //assert!(t.contains_key("a"));
        //assert!(t.contains_key("abc"));
        //assert!(t.contains_key("abcd"));

        assert!(t.remove("abc", true));
        //assert!(t.contains_key("a"));
        //assert!(!t.contains_key("abc"));
        //assert!(!t.contains_key("abcd"));

        assert!(t.remove("a", false));
    }

    #[test]
    fn remove_non_terminal() {
        let mut t = Node::new();
        t.insert("a", 1);
        t.insert("abc", 2);
        t.remove("abc", false);
        println!("{}", t.pp(true));
        let expected = "a\n";
        assert_eq!(t.pp(false), expected);
    }
    #[test]
    fn remove_subtree() {
        let mut t = Node::new();
        t.insert("a", 1);
        t.insert("abc", 2);
        t.remove("ab", true);
        let expected = "a\n";
        assert_eq!(t.pp(false), expected);
    }
    #[test]
    fn remove_non_existing() {
        let mut t = Node::new();
        t.insert("a", 1);
        t.insert("abc", 2);
        let expected = t.pp(false);
        t.remove("xyz", true);
        println!("{}", t.pp(true));
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
        let res = t.longest_prefix("this is more wo", must_be_terminal);
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
        let res = t.longest_prefix("this is weeks", must_be_terminal);
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
        let expected: Vec<char> = "this is more ".chars().collect();
        assert_eq!(res.chars().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn longest_prefix_fail() {
        let mut t = Node::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is more words", 3);
        let must_be_terminal = true;
        let res = t.longest_prefix("this is", must_be_terminal);
        println!("RES: '{res}'");
        assert!(res.is_empty());
    }

    #[test]
    fn find() {
        let mut t = Node::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is even more", 3);
        let must_be_terminal = false;
        let res = t.find("this is more", must_be_terminal);
        assert_eq!(res.unwrap().value.unwrap(), 2)
    }

    #[test]
    fn find_terminal() {
        let mut t = Node::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is even more", 3);
        let must_be_terminal = true;
        let res = t.find("this is more", must_be_terminal);
        assert_eq!(res.unwrap().value.unwrap(), 2);
    }

    #[test]
    fn find_terminal_fail() {
        let mut t = Node::new();
        t.insert("this is words", 1);
        t.insert("this is more", 2);
        t.insert("this is even more", 3);
        let must_be_terminal = true;
        let pref = t.find("this is more wo", must_be_terminal);
        println!("{:#?}", pref);
        assert!(pref.is_none())
    }
}
