use aoc::io;

use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

const FILES: [&str; 2] = [
    "./src/bin/2022_03/sample_input.txt",
    "./src/bin/2022_03/input.txt",
];

lazy_static! {
    static ref PRIO: HashMap<char, usize> = {
        let mut range: Vec<_> = ('a'..='z').collect();
        let range1: Vec<_> = ('A'..='Z').collect();
        range.extend(range1);

        let mut mp: HashMap<char, usize> = HashMap::new();

        for (idx, ch) in range.iter().enumerate() {
            mp.insert(*ch, idx + 1);
        }
        mp
    };
}

fn part1(input_lines: &Vec<String>) {
    let mut priority_sum: usize = 0;

    for line in input_lines {
        let rs = line.split_at(line.len() / 2);
        let chars: Vec<_> = rs.0.chars().filter(|c| rs.1.contains(*c)).collect();
        priority_sum += PRIO[&chars[0]];
    }

    println!("Part 1 Answer : {priority_sum}");
}

fn part2(input_lines: &Vec<String>) {
    let mut priority_sum: usize = 0;

    let n = input_lines.len();
    for i in 0..(n / 3) {
        let chars: Vec<_> = input_lines[3 * i]
            .chars()
            .filter(|c| input_lines[3 * i + 1].contains(*c))
            .filter(|c| input_lines[3 * i + 2].contains(*c))
            .collect();
        priority_sum += PRIO[&chars[0]];
    }

    println!("Part 2 Answer : {priority_sum}");
}

fn main() {
    for filename in FILES {
        println!("Input file {filename}");
        if let Ok(lines) = io::read_lines(filename) {
            let input_lines = lines.flatten().collect::<Vec<String>>();
            part1(&input_lines);
            part2(&input_lines);
        }
    }
}
