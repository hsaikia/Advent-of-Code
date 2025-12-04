use std::collections::HashMap;

use aoc::common;

fn find(
    bank: &[u64],
    start: usize,
    amount: usize,
    cache: &mut HashMap<(usize, usize), u64>,
) -> Option<u64> {
    if let Some(x) = cache.get(&(start, amount)) {
        return Some(*x);
    }

    if start >= bank.len() {
        return None;
    }

    let ret = if amount == 1 {
        Some((start..bank.len()).map(|i| bank[i]).max().unwrap())
    } else {
        let solution_take = find(bank, start + 1, amount - 1, cache)
            .map(|x| x + 10u64.pow(amount as u32 - 1) * bank[start]);
        let solution_skip = find(bank, start + 1, amount, cache);

        if let Some(x) = solution_take {
            if let Some(y) = solution_skip {
                Some(x.max(y))
            } else {
                Some(x)
            }
        } else if let Some(y) = solution_skip {
            Some(y)
        } else {
            None
        }
    };

    if let Some(x) = ret {
        cache.insert((start, amount), x);
    }
    ret
}

fn solve<const PART: usize>(input: &str) -> u64 {
    let mut ans = 0;
    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }
        let batteries = line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(0) as u64)
            .collect::<Vec<u64>>();

        let mut cache: HashMap<(usize, usize), u64> = HashMap::new();
        if let Some(x) = find(&batteries, 0, if PART == 1 { 2 } else { 12 }, &mut cache) {
            ans += x;
        }
    }
    ans
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
        let sample_input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        assert_eq!(solve::<1>(sample_input), 357);
        assert_eq!(solve::<2>(sample_input), 3121910778619);
    }
}
