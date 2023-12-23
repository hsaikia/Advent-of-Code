use aoc::{common, io, range::Range};
use itertools::Itertools;

fn part1(input: &str) -> Option<usize> {
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
                v.push(Some(io::parse_num::<usize>(token)));
            }
        } else if tokens[1] == "map:" {
            w.resize(v.len(), None);
        } else {
            let range = tokens
                .iter()
                .map(|s| io::parse_num::<usize>(s))
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

    *v.iter().min().unwrap()
}

fn part2(input: &str) -> usize {
    type Remap = Option<(usize, usize)>;
    let mut ranges: Vec<(Range<usize>, Remap)> = Vec::new();

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
                rr.push(io::parse_num::<usize>(token));
            }

            let mut i = 0;
            while i < rr.len() {
                ranges.push((Range::new(rr[i], rr[i] + rr[i + 1]), None));
                i += 2;
            }
        } else if tokens.len() == 3 {
            let range = tokens
                .iter()
                .map(|s| io::parse_num::<usize>(s))
                .collect::<Vec<_>>();
            let rs = Range::new(range[1], range[1] + range[2]);
            let rd_a = range[0];

            // Check intersection of current ranges with rs
            // Remap the intersections to rd after batch is processed
            // Push the differences back in

            let mut new_ranges: Vec<(Range<usize>, Remap)> = Vec::new();

            for elem in &ranges {
                let r = &elem.0;
                if let Some(x) = r.intersect(&rs) {
                    for y in r.difference(&x) {
                        new_ranges.push((y, None));
                    }
                    new_ranges.push((x, Some((rs.a, rd_a))));
                } else {
                    new_ranges.push(*elem);
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

    ranges
        .iter()
        .map(|x| x.0.a)
        .sorted()
        .take(1)
        .collect::<Vec<_>>()[0]
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    common::timed(&input, part2, false);
}
