use std::collections::HashMap;

use aoc::{common, io};
use num::Integer;

fn solve(
    start: &str,
    dst: &[&str],
    ins: &[char],
    lm: &HashMap<&str, &str>,
    rm: &HashMap<&str, &str>,
) -> usize {
    let mut steps = 0;
    let mut curr = start;

    loop {
        let is = ins[steps % ins.len()];
        if is == 'L' {
            curr = lm.get(curr).unwrap();
        }
        if is == 'R' {
            curr = rm.get(curr).unwrap();
        }
        steps += 1;

        if dst.contains(&curr) {
            break;
        }
    }

    steps
}

fn part1(ins: &[char], lm: &HashMap<&str, &str>, rm: &HashMap<&str, &str>) -> usize {
    solve("AAA", &["ZZZ"], ins, lm, rm)
}

fn part2(ins: &[char], lm: &HashMap<&str, &str>, rm: &HashMap<&str, &str>) -> usize {
    let mut starts: Vec<&str> = Vec::new();
    let mut dst: Vec<&str> = Vec::new();

    for k in lm.keys() {
        if k.ends_with('A') {
            starts.push(k);
        }

        if k.ends_with('Z') {
            dst.push(k);
        }
    }

    let mut ans: usize = 1;
    for start in starts {
        let sol = solve(start, &dst, ins, lm, rm);
        //println!("{} = {}", x, sol);
        ans = ans.lcm(&sol);
    }
    ans
}

fn process<const PART1: bool>(input: &str) -> usize {
    let mut lm: HashMap<&str, &str> = HashMap::new();
    let mut rm: HashMap<&str, &str> = HashMap::new();
    let line_batches = io::line_batches(input);
    let ins = line_batches[0][0].chars().collect::<Vec<_>>();

    for line in &line_batches[1] {
        let (from, left_right) = line.split_once(" = ").unwrap();
        let (left, right) = left_right[1..left_right.len() - 1]
            .split_once(", ")
            .unwrap();

        lm.insert(from, left);
        rm.insert(from, right);
    }

    if PART1 {
        return part1(&ins, &lm, &rm);
    }
    part2(&ins, &lm, &rm)
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, process::<true>, true);
        common::timed(&input, process::<false>, false);
    }
}
