// [a. b)
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Range {
    pub a: usize,
    pub b: usize,
}

impl Range {
    pub fn new(a: usize, b: usize) -> Self {
        Range { a, b }
    }

    pub fn contains(&self, x: usize) -> bool {
        x >= self.a && x < self.b
    }

    pub fn idx_in(&self, x: usize) -> Option<usize> {
        if self.contains(x) {
            return Some(x - self.a);
        }
        None
    }

    pub fn idx_from(&self, idx: usize) -> usize {
        self.a + idx
    }

    pub fn intersect(&self, other: &Range) -> Option<Range> {
        let max_a = self.a.max(other.a);
        let min_b = self.b.min(other.b);
        if max_a < min_b {
            return Some(Range { a: max_a, b: min_b });
        }
        None
    }

    pub fn difference(&self, other: &Range) -> Vec<Range> {
        let mut ret = Vec::new();
        if let Some(intersection) = self.intersect(other) {
            if self.a < intersection.a {
                ret.push(Range {
                    a: self.a,
                    b: intersection.a,
                });
            }

            if intersection.b < self.b {
                ret.push(Range {
                    a: intersection.b,
                    b: self.b,
                });
            }
        }
        ret
    }

    pub fn remap(&mut self, current_a: usize, mapped_a: usize) {
        self.b = self.b + mapped_a - current_a;
        self.a = self.a + mapped_a - current_a;
    }
}
