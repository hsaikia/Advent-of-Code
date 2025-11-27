use aoc::common;

fn solve(input_lines: &str, marker_size: usize) -> usize {
    for line in input_lines.split('\n') {
        for (i, marker) in line
            .chars()
            .collect::<Vec<char>>()
            .windows(marker_size)
            .enumerate()
        {
            //println!("Checking marker : {:?}", marker);
            if (1..marker_size).any(|j| marker[j..].contains(&marker[j - 1])) {
                continue;
            }

            return i + marker_size;
        }
    }
    0
}

fn part1(input: &str) -> usize {
    solve(input, 4)
}

fn part2(input: &str) -> usize {
    solve(input, 14)
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, part1, true);
        common::timed(&input, part2, false);
    }
}
