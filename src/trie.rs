use core::fmt;
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
        if self.children.is_empty() {
            if print_value {
                res.push_str(format!("  {}", self.value.as_ref().unwrap()).as_str());
            }
            res.push('\n');
            return res;
        }
        let child_count = self.children.iter().count();
        for (k, v) in self.children.iter() {
            if self.is_terminal || child_count > 1 {
                if indent != 0 {
                    res.push('\n');
                }
                res.push_str(&" ".repeat(indent.into()));
            }

            res.push_str(&k.to_string());
            res.push_str(v.pp_fn(indent + 1, print_value).as_str());
        }

        return res;
    }
}

impl<T> Node<T> {
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
}
