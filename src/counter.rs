#[derive(Default)]
pub struct Counter {
    pub prev: usize,
    pub cur: usize,
}

impl Counter {
    pub fn delta(&self) -> usize {
        self.cur.saturating_sub(self.prev)
    }

    pub fn inc(&mut self) {
        self.cur += 1;
    }

    pub fn lap(&mut self) {
        self.prev = self.cur;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_after_inc_and_lap() {
        let mut c = Counter::default();
        c.inc();
        c.lap();
        c.inc();
        assert_eq!(c.delta(), 1);
    }

    #[test]
    fn delta_saturates_when_prev_gt_cur() {
        let c = Counter { prev: 5, cur: 2 };
        assert_eq!(c.delta(), 0);
    }
}
