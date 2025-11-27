use aoc::common;
use aoc::io;

fn solve<const PART: usize>(input: &str) -> usize {
    let (mut lst1, mut lst2): (Vec<usize>, Vec<usize>) = input
        .lines()
        .map(|line| io::tokenize_nums(line, " "))
        .fold((vec![], vec![]), |(mut v1, mut v2), nums| {
            v1.push(nums[0]);
            v2.push(nums[1]);
            (v1, v2)
        });

    if PART == 1 {
        lst1.sort_unstable();
        lst2.sort_unstable();
        lst1.iter()
            .zip(lst2.iter())
            .map(|(a, b)| a.max(b) - a.min(b))
            .sum()
    } else {
        lst1.iter()
            .map(|n| lst2.iter().filter(|x| *x == n).count() * n)
            .sum()
    }
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, solve::<1>, true);
        common::timed(&input, solve::<2>, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(solve::<1>(sample_input), 11);
        assert_eq!(solve::<2>(sample_input), 31);
    }
}
