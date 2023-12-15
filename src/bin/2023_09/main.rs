use aoc::common;
use itertools::Itertools;

fn solve<const PART1: bool>(input: &str) -> i64 {
    let mut ans: i64 = 0;

    for line in input.split('\n') {
        let nums = line
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let mut x = nums.clone();
        if !PART1 {
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
    ans
}

fn main() {
    let input = common::get_input();
    common::timed(&input, solve::<true>, true);
    common::timed(&input, solve::<false>, false);
}
