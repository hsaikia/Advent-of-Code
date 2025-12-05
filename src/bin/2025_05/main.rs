use aoc::{
    common, io,
    range::{Range, RangeUnion},
};

fn solve<const PART: usize>(input: &str) -> usize {
    let mut ans = 0;
    let mut ru: RangeUnion<usize> = RangeUnion::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains("-") {
            let tokens = io::tokenize_nums::<usize>(line, "-");
            ru.add_range(Range::new(tokens[0], tokens[1] + 1));
        } else {
            let token = io::parse_num::<usize>(line);
            if ru.contains(token) {
                ans += 1;
            }
        }
    }

    if PART == 1 {
        ans
    } else {
        ru.spread()
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
        let sample_input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        assert_eq!(solve::<1>(sample_input), 3);
        assert_eq!(solve::<2>(sample_input), 14);
    }
}
