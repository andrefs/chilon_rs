use crate::Node;

// Represents occurrences as subject, predicate or object
#[derive(Debug, Default)]
pub struct Stats {
    pub s: u32,
    pub p: u32,
    pub o: u32,
    pub total: u32,
}

// Each node keeps its own stats (if terminal) and its descendants stats
#[derive(Debug)]
pub struct NodeStats {
    pub own: Option<Stats>,
    pub desc: Stats,
}
pub enum TriplePos {
    S,
    P,
    O,
}

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
    }
    pub fn inc_p(&mut self) {
        self.p += 1;
    }
    pub fn inc_o(&mut self) {
        self.o += 1;
    }
}
