use crate::trie::Node;

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
pub fn dec_stats(parent: &mut IriTrie, ch: char, child: &IriTrie) {
    println!("char {ch}");
    let mut par_desc = parent.value.as_mut().unwrap_or(&mut NodeStats::new()).desc;
    let child_own = child
        .value
        .as_ref()
        .unwrap()
        .own
        .unwrap_or(Default::default());
    let child_desc = child.value.as_ref().unwrap().desc;
    println!(
        "par_desc {:#?}\n\tchild_own {:#?}\n\tchild_desc {:#?}",
        par_desc, child_own, child_desc
    );
    par_desc.s -= child_own.s + child_desc.s;
    par_desc.p -= child_own.p + child_desc.p;
    par_desc.o -= child_own.o + child_desc.o;
}

#[cfg(test)]
mod tests {
    use crate::trie::TraverseFns;

    use super::*;

    #[test]
    fn remove_fn_dec_stats() {
        let pos = TriplePos::S;
        let stats = NodeStats::new_terminal(pos);
        let mut t = Node::new();
        t.insert_fn(
            "a",
            stats,
            TraverseFns {
                any: Some(&inc_stats(pos)),
                terminal: None,
            },
        );
        t.insert_fn(
            "abc",
            stats,
            TraverseFns {
                any: Some(&inc_stats(pos)),
                terminal: None,
            },
        );
        println!("{}", t.pp(true));
        t.remove_fn("abc", true, Some(&dec_stats));
        println!("{}", t.pp(true));
        assert!(false);
    }
}
