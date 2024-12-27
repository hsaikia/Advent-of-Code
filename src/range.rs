use std::fmt::{Debug, Display};

use itertools::Itertools;
use num::{traits::NumAssign, Integer, Num};

/// A collection of ranges that dynamically maintain mutual exclusivity
#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct RangeUnion<T: Integer + Default + Ord + Num + NumAssign + Copy + Clone + Display> {
    pub ranges: Vec<Range<T>>,
}

impl<T: Integer + Default + Ord + Num + NumAssign + Copy + Clone + Display> Debug
    for RangeUnion<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .ranges
            .iter()
            .map(|r| format!("[{}, {}]", r.a, r.b))
            .join(" | ");
        write!(f, "{s}")
    }
}

impl<T: Integer + Default + Ord + Num + NumAssign + Copy + Clone + Display> Default
    for RangeUnion<T>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Integer + Default + Ord + Num + NumAssign + Copy + Clone + Display> RangeUnion<T> {
    #[must_use]
    pub fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    /// # Panics
    /// If ranges are empty
    pub fn merge(&mut self) {
        self.ranges.sort_by(|r1, r2| r1.a.cmp(&r2.a));
        let mut new_ranges = vec![*self
            .ranges
            .first()
            .expect("first() called on empty vector of ranges")];
        for range in self.ranges.iter().skip(1) {
            if let Some(last) = new_ranges.last_mut() {
                if last.b < range.a {
                    new_ranges.push(*range);
                } else {
                    last.b = last.b.max(range.b);
                }
            }
        }
        self.ranges = new_ranges;
    }

    pub fn add_range(&mut self, range: Range<T>) {
        self.ranges.push(range);
        self.merge();
    }

    #[must_use]
    pub fn spread(&self) -> T {
        let mut ans: T = T::default();
        for r in &self.ranges {
            ans += r.spread();
        }
        ans
    }

    pub fn contains(&self, x: T) -> bool {
        for r in &self.ranges {
            if r.contains(x) {
                return true;
            }
        }
        false
    }

    #[must_use]
    pub fn intersect(&self, range: &Range<T>) -> RangeUnion<T> {
        RangeUnion::<T> {
            ranges: self
                .ranges
                .iter()
                .filter_map(|r| r.intersect(range))
                .collect(),
        }
    }

    #[must_use]
    pub fn difference(&self, range: &Range<T>) -> RangeUnion<T> {
        RangeUnion::<T> {
            ranges: self
                .ranges
                .iter()
                .flat_map(|r| r.difference(range))
                .collect_vec(),
        }
    }
}

/// A 1D Range of the form [a, b) i.e., inclusive at the front, exclusive at the back
#[derive(PartialEq, Eq, Clone, Hash, Copy)]
pub struct Range<T: Integer + Default + Ord + Num + NumAssign + Copy + Display> {
    pub a: T,
    pub b: T,
}

impl<T: Integer + Default + Ord + Num + NumAssign + Copy + Display> Debug for Range<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{})", self.a, self.b)
    }
}

impl<T: Integer + Default + Ord + Num + NumAssign + Copy + Display> Range<T> {
    pub fn new(a: T, b: T) -> Self {
        Range { a, b }
    }

    pub fn contains(&self, x: T) -> bool {
        x >= self.a && x < self.b
    }

    pub fn idx_in(&self, x: T) -> Option<T> {
        if self.contains(x) {
            return Some(x - self.a);
        }
        None
    }

    pub fn idx_from(&self, idx: T) -> T {
        self.a + idx
    }

    pub fn spread(&self) -> T {
        self.b - self.a
    }

    pub fn intersect(&self, other: &Range<T>) -> Option<Range<T>> {
        let max_a = self.a.max(other.a);
        let min_b = self.b.min(other.b);
        if max_a < min_b {
            return Some(Range { a: max_a, b: min_b });
        }
        None
    }

    pub fn difference(&self, other: &Range<T>) -> Vec<Range<T>> {
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

    pub fn remap(&mut self, current_a: T, mapped_a: T) {
        self.b = self.b + mapped_a - current_a;
        self.a = self.a + mapped_a - current_a;
    }
}
