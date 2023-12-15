use aoc::common;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn solve(input: &str, all: bool) -> usize {
    let mut ans: usize = 0;
    let mut digit_map: Vec<(String, usize)> = Vec::new();
    for i in 1..=9 {
        let s = i.to_string();
        digit_map.push((s, i));
    }

    if all {
        for (i, digit) in DIGITS.iter().enumerate() {
            digit_map.push((digit.to_string(), i + 1));
        }
    }

    for line in input.split('\n') {
        let mut digit1 = digit_map
            .iter()
            .filter_map(|(x, i)| line.find(x).map(|idx| (idx, i)))
            .collect::<Vec<_>>();
        digit1.sort_by(|a, b| a.0.cmp(&b.0));

        let mut digit2 = digit_map
            .iter()
            .filter_map(|(x, i)| line.rfind(x).map(|idx| (idx, i)))
            .collect::<Vec<_>>();
        digit2.sort_by(|a, b| b.0.cmp(&a.0));

        if digit1.is_empty() || digit2.is_empty() {
            println!("Input doesn't match algorithm.");
            return 0;
        }

        ans += 10 * digit1[0].1;
        ans += digit2[0].1;
    }

    ans
}

fn part1(input: &str) -> usize {
    solve(input, false)
}

fn part2(input: &str) -> usize {
    solve(input, true)
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    common::timed(&input, part2, false);
}
