use aoc::common;
use aoc::io;
use num::pow;

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Concat,
}

impl Op {
    fn apply(&self, a: usize, b: usize) -> usize {
        match *self {
            Op::Add => a + b,
            Op::Mul => a * b,
            Op::Concat => {
                let mut f = 1;
                let mut b_tmp = b;
                while b_tmp > 0 {
                    b_tmp /= 10;
                    f *= 10;
                }
                a * f + b
            }
        }
    }
}

fn get_op<const BASE: i32>(permutation_idx: i32, bits: usize) -> Vec<Op> {
    let mut permutation_idx = permutation_idx;
    let mut ret = Vec::new();
    for _ in 0..bits {
        if permutation_idx % BASE == 0 {
            ret.push(Op::Add);
        } else if permutation_idx % BASE == 1 {
            ret.push(Op::Mul);
        } else if permutation_idx % BASE == 2 {
            ret.push(Op::Concat);
        }

        permutation_idx /= BASE;
    }
    ret
}

fn solve<const BASE: i32>(input: &str) -> usize {
    let mut ans = 0;
    for line in input.lines() {
        let s1 = io::tokenize(line, ": ");
        let res: usize = io::parse_num(s1[0]);
        let nums: Vec<usize> = io::tokenize_nums(s1[1], " ");
        let l = nums.len();
        for permutation in 0..pow(BASE, l - 1) {
            let ops = get_op::<BASE>(permutation, l - 1);
            let mut test = nums[0];
            for i in 1..l {
                test = ops[i - 1].apply(test, nums[i]);
            }
            if test == res {
                ans += res;
                break;
            }
        }
    }
    ans
}

fn main() {
    let input = common::get_input();
    // println!("{input:?}");
    common::timed(&input, solve::<2>, true);
    common::timed(&input, solve::<3>, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";
        assert_eq!(solve::<2>(sample_input), 3749);
        assert_eq!(solve::<3>(sample_input), 11387);
    }
}
