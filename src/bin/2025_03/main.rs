use std::collections::HashMap;

use aoc::common;

fn find(bank: &[u64], i: usize, l: usize, mp: &mut HashMap<(usize, usize), u64>) -> Option<u64> {
    if let Some(x) = mp.get(&(i, l)) {
        return Some(*x);
    }

    if i >= bank.len() {
        return None;
    }

    if l == 1 {
        let mx = (i..bank.len()).map(|x| bank[x]).max().unwrap();
        mp.insert((i, l), mx);
        return Some(mx);
    }

    let x1 = if let Some(x) = find(bank, i + 1, l - 1, mp) {
        Some(x + 10u64.pow(l as u32 - 1) * bank[i])
    } else {
        None
    };

    let x2 = if let Some(x) = find(bank, i + 1, l, mp) {
        Some(x)
    } else {
        None
    };
    //dbg!(i, l, x1, x2);
    if let Some(x) = x1 {
        if let Some(y) = x2 {
            mp.insert((i, l), x.max(y));
            return Some(x.max(y));
        } else {
            mp.insert((i, l), x);
            return Some(x);
        }
    } else {
        if let Some(y) = x2 {
            mp.insert((i, l), y);
            return Some(y);
        } else {
            return None;
        }
    }
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
        // let mut mx = 0;
        // for (i, b1) in batteries.iter().enumerate() {
        //     for b2 in batteries.iter().skip(i + 1) {
        //         mx = mx.max(10 * b1 + b2);
        //     }
        // }
        let mut mp: HashMap<(usize, usize), u64> = HashMap::new();
        if let Some(mx) = find(&batteries, 0, if PART == 1 { 2 } else { 12 }, &mut mp) {
            //dbg!(line, mx);
            ans = ans + mx;
        }
    }
    ans
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, solve::<1>, true);
        common::timed(&input, solve::<2>, true);
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
