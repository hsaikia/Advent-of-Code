use std::time::Instant;

use aoc::io;
use itertools::Itertools;

const INPUT: [(&str, &str); 1] = [
    //("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

// [a. b)
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct Range {
    a: usize,
    b: usize,
}

impl Range {
    fn new(a: usize, b: usize) -> Self {
        Range { a, b }
    }

    fn contains(&self, x: usize) -> bool {
        x >= self.a && x < self.b
    }

    fn idx_in(&self, x: usize) -> Option<usize> {
        if self.contains(x) {
            return Some(x - self.a);
        }
        None
    }

    fn idx_from(&self, idx: usize) -> usize {
        self.a + idx
    }

    fn intersect(&self, other: &Range) -> Option<Range> {
        let max_a = self.a.max(other.a);
        let min_b = self.b.min(other.b);
        if max_a < min_b {
            return Some(Range { a: max_a, b: min_b });
        }
        None
    }

    fn difference(&self, other: &Range) -> Vec<Range> {
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

    fn remap(&mut self, current_a: usize, mapped_a: usize) {
        self.b = self.b + mapped_a - current_a;
        self.a = self.a + mapped_a - current_a;
    }
}

fn part1(input: &str) {
    let mut v: Vec<Option<usize>> = Vec::new();
    let mut w: Vec<Option<usize>> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            if !v.is_empty() && v.len() == w.len() {
                for (i, ww) in w.iter().enumerate() {
                    if let Some(xx) = ww {
                        v[i] = Some(*xx);
                    }
                }

                w.clear();
            }
            continue;
        }

        let tokens = io::tokenize(line, " ");

        if tokens[0] == "seeds:" {
            for token in tokens.iter().skip(1) {
                v.push(Some(io::parse_num::<usize>(token).unwrap()));
            }
        } else if tokens[1] == "map:" {
            w.resize(v.len(), None);
        } else {
            let range = tokens
                .iter()
                .map(|s| io::parse_num::<usize>(s).unwrap())
                .collect::<Vec<_>>();
            let rs = Range::new(range[1], range[1] + range[2]);
            let rd = Range::new(range[0], range[0] + range[2]);

            for (i, e) in v.iter().enumerate() {
                if let Some(x) = e {
                    if let Some(idx) = rs.idx_in(*x) {
                        w[i] = Some(rd.idx_from(idx));
                    }
                }
            }
        }
    }

    for (i, ww) in w.iter().enumerate() {
        if let Some(xx) = ww {
            v[i] = Some(*xx);
        }
    }

    println!("Answer Part 1 : {:?}", v.iter().min().unwrap());
}

fn part2(input: &str) {
    let mut ranges: Vec<(Range, Option<(usize, usize)>)> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            for (range, remap) in &mut ranges {
                if let Some((s_a, d_a)) = remap {
                    range.remap(*s_a, *d_a);
                    *remap = None;
                }
            }
            continue;
        }

        let tokens = io::tokenize(line, " ");

        if tokens[0] == "seeds:" {
            let mut rr = Vec::new();
            for token in tokens.iter().skip(1) {
                rr.push(io::parse_num::<usize>(token).unwrap());
            }

            let mut i = 0;
            while i < rr.len() {
                ranges.push((Range::new(rr[i], rr[i] + rr[i + 1]), None));
                i += 2;
            }
        } else if tokens.len() == 3 {
            let range = tokens
                .iter()
                .map(|s| io::parse_num::<usize>(s).unwrap())
                .collect::<Vec<_>>();
            let rs = Range::new(range[1], range[1] + range[2]);
            let rd_a = range[0];

            // Check intersection of current ranges with rs
            // Remap the intersections to rd
            // Push the differences back in

            let mut new_ranges: Vec<(Range, Option<(usize, usize)>)> = Vec::new();

            for elem in &ranges {
                let r = &elem.0;
                if let Some(x) = r.intersect(&rs) {
                    for y in r.difference(&x) {
                        new_ranges.push((y, None));
                    }
                    new_ranges.push((x, Some((rs.a, rd_a))));
                } else {
                    new_ranges.push(elem.clone());
                }
            }

            ranges = new_ranges;
        }
    }

    for (range, remap) in &mut ranges {
        if let Some((s_a, d_a)) = remap {
            range.remap(*s_a, *d_a);
            *remap = None;
        }
    }

    println!(
        "Answer Part 2 : {:?}",
        ranges
            .iter()
            .map(|x| x.0.a)
            .sorted()
            .take(1)
            .collect::<Vec<_>>()[0]
    );
}

fn main() {
    for (file, input) in INPUT {
        println!("{}", file);
        let start = Instant::now();
        part1(input);
        let duration = start.elapsed();
        println!("Time elapsed in Part 1 is: {:?}", duration);
        let start = Instant::now();
        part2(input);
        let duration = start.elapsed();
        println!("Time elapsed in Part 1 is: {:?}", duration);
    }
}
