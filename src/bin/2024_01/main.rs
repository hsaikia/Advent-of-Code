use aoc::common;
use aoc::io;

fn solve<const PART: usize>(input: &str) -> usize {
    let mut lst1 = Vec::new();
    let mut lst2 = Vec::new();
    for line in input.lines() {
        let nums: Vec<usize> = io::tokenize_nums(line, " ");
        lst1.push(nums[0]);
        lst2.push(nums[1]);
    }
    lst1.sort();
    lst2.sort();

    if PART == 1 {
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
    let input = common::get_input();
    common::timed(&input, solve::<1>, true);
    common::timed(&input, solve::<2>, false);
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
