use aoc::common;
use aoc::io;
use itertools::Itertools;

fn is_num(s: &str) -> bool {
    s.chars().all(|x| x.is_ascii_digit())
}

// the ....mul(X,Y)...... part
fn solve_muls(input: &str) -> usize {
    let mut ans = 0;
    for pattern in io::tokenize(input, "mul(") {
        let tokens1 = io::tokenize(pattern, ",");
        let num1 = tokens1.first().unwrap();
        // check if num1 has all digits
        if is_num(num1) {
            let n1: usize = io::parse_num(num1);
            let tokens2 = io::tokenize(tokens1[1], ")");
            let num2 = tokens2.first().unwrap();
            if is_num(num2) {
                let n2: usize = io::parse_num(num2);
                ans += n1 * n2;
            }
        }
    }
    ans
}

fn solve<const PART: usize>(input: &str) -> usize {
    let mut ans = 0;

    if PART == 1 {
        ans = solve_muls(input);
    } else {
        let indices = input
            .match_indices("do()")
            .chain(input.match_indices("don't()"))
            .sorted_by_key(|x| x.0);

        let mut last_idx = 0;
        let mut yes: bool = true;
        for (idx, ins) in indices {
            if yes {
                ans += solve_muls(&input[last_idx..idx]);
            }
            last_idx = idx;
            yes = ins == "do()";
        }

        if yes {
            ans += solve_muls(&input[last_idx..]);
        }
    }

    ans
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
