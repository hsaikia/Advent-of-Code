use std::{collections::HashMap, time::Instant};

use aoc::io;
use num::Integer;

const INPUT: [(&str, &str); 2] = [
    ("Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

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

fn part1(ins: &[char], lm: &HashMap<&str, &str>, rm: &HashMap<&str, &str>) {
    let sol = solve("AAA", &["ZZZ"], ins, lm, rm);
    println!("Part1 answer : {}", sol);
}

fn part2(ins: &[char], lm: &HashMap<&str, &str>, rm: &HashMap<&str, &str>) {
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

    println!("Part2 Answer : {}", ans);
}

fn main() {
    for (file, input) in INPUT {
        println!("{}", file);

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

        let start = Instant::now();
        part1(&ins, &lm, &rm);
        let duration = start.elapsed();
        println!("Time elapsed in Part 1 is: {:?}", duration);

        let start = Instant::now();
        part2(&ins, &lm, &rm);
        let duration = start.elapsed();
        println!("Time elapsed in Part 2 is: {:?}", duration);
    }
}
