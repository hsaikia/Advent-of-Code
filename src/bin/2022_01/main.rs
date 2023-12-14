use std::env;

fn part1(input: &str) {
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

    println!("Part 1 Answer : {best}");
}

fn part2(input: &str) {
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

    println!("Part 2 Answer : {}", best.iter().sum::<usize>());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let input = std::fs::read_to_string(filepath).unwrap();
    part1(&input);
    part2(&input);
}
