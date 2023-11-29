use aoc::io;

const FILES: [&str; 2] = [
    "./src/bin/2022_01/sample_input.txt",
    "./src/bin/2022_01/input.txt",
];

fn part1(input_lines: &Vec<String>) {
    let mut curr: usize = 0;
    let mut best: usize = 0;

    for line in input_lines {
        if line.is_empty() {
            best = best.max(curr);
            curr = 0;
        } else {
            curr += line.parse::<usize>().unwrap();
        }
    }

    println!("Part 1 Answer : {best}");
}

fn part2(input_lines: &Vec<String>) {
    let mut curr: usize = 0;
    let mut best: Vec<usize> = Vec::new();

    for line in input_lines {
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
    for filename in FILES {
        println!("Input file {filename}");
        if let Ok(lines) = io::read_lines(filename) {
            let input_lines = lines.flatten().collect::<Vec<String>>();
            part1(&input_lines);
            part2(&input_lines);
        }
    }
}
