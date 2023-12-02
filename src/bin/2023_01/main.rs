const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const INPUT: [(&str, &str); 3] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Sample Input", include_str!("sample_input2.txt")),
    ("Input", include_str!("input.txt")),
];

fn solve(input: &str, all: bool) {
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
            return;
        }

        ans += 10 * digit1[0].1;
        ans += digit2[0].1;
    }

    println!("Answer with all digits ({}) : {}", all, ans);
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        solve(input.1, false);
        solve(input.1, true);
    }
}
