use aoc::{io, range::Range};
use itertools::Itertools;
use std::time::Instant;

const INPUT: [(&str, &str); 1] = [
    //("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

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
        println!("Time elapsed in Part 2 is: {:?}", duration);
    }
}
