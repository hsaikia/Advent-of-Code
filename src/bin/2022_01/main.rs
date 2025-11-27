use aoc::common;

fn part1(input: &str) -> usize {
    let mut curr: usize = 0;
    let mut best: usize = 0;

    for line in input.split('\n') {
        if line.is_empty() {
            best = best.max(curr);
            curr = 0;
        } else {
            curr += line.parse::<usize>().unwrap();
        }
    }
    best
}

fn part2(input: &str) -> usize {
    let mut curr: usize = 0;
    let mut best: Vec<usize> = Vec::new();

    for line in input.split('\n') {
        if line.is_empty() {
            best.push(curr);
            best.sort_by(|a, b| b.cmp(a));
            best.truncate(3);
            curr = 0;
        } else {
            curr += line.parse::<usize>().unwrap();
        }
    }
    best.iter().sum::<usize>()
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, part1, true);
        common::timed(&input, part2, false);
    }
}
