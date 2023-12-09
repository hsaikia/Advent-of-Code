use itertools::Itertools;

const INPUT: [(&str, &str); 2] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

fn solve(input: &str, part1: bool) {
    let mut ans: i64 = 0;

    for line in input.split('\n') {
        let nums = line
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let mut x = nums.clone();
        if !part1 {
            x.reverse();
        }

        let mut cnt = 0;
        loop {
            cnt += *x.last().unwrap();
            if x.iter().filter(|&x| *x != 0).count() == 0 {
                break;
            }
            x = x
                .iter()
                .tuple_windows()
                .map(|(a, b)| *b - *a)
                .collect::<Vec<_>>();
        }
        ans += cnt;
    }

    println!("Answer Part {}: {}", if part1 { 1 } else { 2 }, ans);
}

fn main() {
    for (file, input) in INPUT {
        println!("{}", file);
        solve(input, true);
        solve(input, false);
    }
}
