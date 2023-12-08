use std::{collections::HashMap, time::Instant};

use aoc::io;
use num::Integer;

const INPUT: [(&str, &str); 1] = [("Input", include_str!("input.txt"))];

fn solve_for_one(
    start: &str,
    ins: &[char],
    dst: &[&str],
    lm: &HashMap<&str, &str>,
    rm: &HashMap<&str, &str>,
) -> usize {
    let mut steps = 0;
    let mut idx = 0;
    let mut curr = start;

    loop {
        let is = ins[idx];
        if is == 'L' {
            curr = lm.get(curr).unwrap();
        }
        if is == 'R' {
            curr = rm.get(curr).unwrap();
        }
        idx = (idx + 1) % ins.len();
        steps += 1;

        if dst.contains(&curr) {
            break;
        }
    }

    steps
}

fn part1(ins: &[char], lm: &HashMap<&str, &str>, rm: &HashMap<&str, &str>) {
    let sol = solve_for_one("AAA", ins, &["ZZZ"], lm, rm);
    println!("Part1 answer : {}", sol);
}

fn part2(ins: &[char], lm: &HashMap<&str, &str>, rm: &HashMap<&str, &str>) {
    let mut curr: Vec<&str> = Vec::new();
    let mut dst: Vec<&str> = Vec::new();

    for k in lm.keys() {
        if k.chars().collect::<Vec<_>>()[2] == 'A' {
            curr.push(k);
        }
        if k.chars().collect::<Vec<_>>()[2] == 'Z' {
            dst.push(k);
        }
    }

    let mut ans: usize = 1;
    for x in curr {
        let sol = solve_for_one(x, ins, &dst, lm, rm);
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
        let lines = input.lines().collect::<Vec<&str>>();
        let ins = lines[0].chars().collect::<Vec<_>>();

        for line in &lines[2..] {
            if line.is_empty() {
                continue;
            }
            let tokens = io::tokenize(line, " = ");
            lm.insert(tokens[0], &tokens[1][1..4]);
            rm.insert(tokens[0], &tokens[1][6..9]);
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
