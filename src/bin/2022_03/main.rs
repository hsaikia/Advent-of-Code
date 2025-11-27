use aoc::common;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

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

fn part1(input_lines: &str) -> usize {
    let mut priority_sum: usize = 0;

    for line in input_lines.split('\n') {
        let rs = line.split_at(line.len() / 2);
        let chars: Vec<_> = rs.0.chars().filter(|c| rs.1.contains(*c)).collect();
        priority_sum += PRIO[&chars[0]];
    }
    priority_sum
}

fn part2(input: &str) -> usize {
    let mut priority_sum: usize = 0;
    let input_lines = input.split('\n').collect::<Vec<&str>>();

    let n = input_lines.len();
    for i in 0..(n / 3) {
        let chars: Vec<_> = input_lines[3 * i]
            .chars()
            .filter(|c| input_lines[3 * i + 1].contains(*c))
            .filter(|c| input_lines[3 * i + 2].contains(*c))
            .collect();
        priority_sum += PRIO[&chars[0]];
    }
    priority_sum
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, part1, true);
        common::timed(&input, part2, false);
    }
}
