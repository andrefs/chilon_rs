use std::{
    borrow::Borrow,
    collections::BTreeMap,
    fmt::{Debug, Display},
    mem,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<T> {
    value: Option<T>,
    is_terminal: bool,
    children: BTreeMap<char, Node<T>>,
}

impl<T: Display> Node<T> {
    pub fn pp(&self, print_value: bool) -> String {
        self.pp_fn(0, print_value)
    }

    fn pp_fn(&self, indent: u8, print_value: bool) -> String {
        let mut res = "".to_string();
        // print value
        if print_value && self.value.is_some() {
            res.push_str(format!("  {}", self.value.as_ref().unwrap()).as_str());
        }
        if self.children.is_empty() || self.is_terminal {
            res.push('\n');
        }
        for (k, v) in self.children.iter() {
            if self.is_terminal {
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

    pub fn insert<Q: ?Sized>(&mut self, key: &Q, value: T) -> Option<T>
    where
        Q: Borrow<str>,
    {
        let k: &str = key.borrow();
        if k.is_empty() {
            self.is_terminal = true;
            return mem::replace(&mut self.value, Some(value));
        }

        let first_char = k.chars().next().unwrap();
        let rest = &k[first_char.len_utf8()..];

        if self.children.contains_key(&first_char) {
            return self
                .children
                .get_mut(&first_char)
                .unwrap()
                .insert(rest, value);
        }

        let mut new_node = Node {
            is_terminal: false,
            children: BTreeMap::new(),
            value: None,
        };
        let res = new_node.insert(rest, value);
        self.children.insert(first_char, new_node);
        res
    }

    pub fn remove(&mut self, key: &str, remove_subtree: bool) -> bool {
        let res = self.remove_fn(key, remove_subtree).1;
        res
    }

    fn remove_fn(&mut self, str_left: &str, remove_subtree: bool) -> (bool, bool) {
        let first_char = str_left.chars().next().unwrap();
        let rest = &str_left[first_char.len_utf8()..];

        if self.children.is_empty() {
            return (false, false);
        }

        if !self.children.contains_key(&first_char) {
            return (false, false);
        }

        if rest.is_empty() {
            let sub_node = self.children.get_mut(&first_char).unwrap();
            if sub_node.children.is_empty() || remove_subtree {
                let old_node = self.children.remove(&first_char);
                match old_node {
                    None => return (false, false),
                    Some(_) => {
                        let bubble_up = self.children.is_empty() && !self.is_terminal;
                        return (bubble_up, true);
                    }
                }
            }

            if !sub_node.is_terminal {
                return (false, false);
            }
            sub_node.is_terminal = false;
            return (true, true);
        } else {
            let (bubble_up, removed) = self
                .children
                .get_mut(&first_char)
                .unwrap()
                .remove_fn(rest, remove_subtree);
            if bubble_up {
                let removed = self.children.remove(&first_char).is_some();
                let bubble_up = removed && !self.is_terminal && self.children.is_empty();
                return (bubble_up, removed);
            }
            return (false, removed);
        }
    }
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
}
