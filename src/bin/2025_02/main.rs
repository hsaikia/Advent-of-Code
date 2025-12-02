use aoc::{common, io};

fn invalid<const PART: usize>(num: usize) -> bool {
    let s = num.to_string();
    let len = s.len();
    let lengths: Vec<usize> = if PART == 1 {
        if len % 2 == 0 {
            vec![len / 2]
        } else {
            vec![]
        }
    } else {
        (1..=len / 2).collect()
    };

    for l in lengths {
        if len % l == 0 {
            if s.as_bytes().chunks(l).all(|c| c == s[0..l].as_bytes()) {
                return true;
            }
        }
    }

    false
}

fn solve<const PART: usize>(input: &str) -> usize {
    let mut ans = 0;
    for line in input.split(",") {
        let nums: Vec<usize> = io::tokenize_nums(line, "-");
        for x in nums[0]..=nums[1] {
            if invalid::<PART>(x) {
                ans += x;
            }
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
        let sample_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,
        824824821-824824827,2121212118-2121212124";
        assert_eq!(solve::<1>(sample_input), 1227775554);
        assert_eq!(solve::<2>(sample_input), 4174379265);
    }
}
