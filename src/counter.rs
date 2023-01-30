#[derive(Default)]
pub struct Counter {
    pub prev: usize,
    pub cur: usize,
}

impl Counter {
    pub fn delta(&self) -> usize {
        self.cur - self.prev
    }

    pub fn inc(&mut self) {
        self.cur += 1;
    }

    pub fn lap(&mut self) {
        self.prev = self.cur;
    }
}
