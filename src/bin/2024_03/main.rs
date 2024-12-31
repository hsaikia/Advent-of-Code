use aoc::common;
use aoc::io;
use itertools::Itertools;

fn compute(input: &str) -> usize {
    let mut ans = 0;
    for pattern in io::tokenize(input, "mul(").iter().skip(1) {
        if let Some((num1_str, num2_str_plus)) = pattern.split_once(',') {
            if let Some(n1) = io::try_parse_num::<usize>(num1_str) {
                if let Some((num2_str, _)) = num2_str_plus.split_once(')') {
                    if let Some(n2) = io::try_parse_num::<usize>(num2_str) {
                        ans += n1 * n2;
                    }
                }
            }
        }
    }
    ans
}

fn solve<const PART: usize>(input: &str) -> usize {
    if PART == 1 {
        compute(input)
    } else {
        let extreme_checkpoints = [(0, "do()"), (input.len(), "don't()")];
        let checkpoints: Vec<(usize, &str)> = extreme_checkpoints
            .into_iter()
            .chain(input.match_indices("do()"))
            .chain(input.match_indices("don't()"))
            .sorted_by_key(|(idx, _)| *idx)
            .collect();

        checkpoints.windows(2).fold(0, |mut acc, interval| {
            if interval[0].1 == "do()" {
                acc += compute(&input[interval[0].0..interval[1].0]);
            }
            acc
        })
    }
}

fn main() {
    let input = common::get_input();
    println!("{input:?}");
    common::timed(&input, solve::<1>, true);
    common::timed(&input, solve::<2>, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input1 =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let sample_input2 =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(solve::<1>(sample_input1), 161);
        assert_eq!(solve::<2>(sample_input2), 48);
    }
}
