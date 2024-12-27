use std::ops::BitXor;

use aoc::{common, io};
use itertools::Itertools;
use num::pow;

#[derive(Clone, Copy, Debug)]
enum Op {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

const OPCODES: [Op; 8] = [
    Op::Adv,
    Op::Bxl,
    Op::Bst,
    Op::Jnz,
    Op::Bxc,
    Op::Out,
    Op::Bdv,
    Op::Cdv,
];

fn combo_operand(a: usize, b: usize, c: usize, op: usize) -> usize {
    if op == 0 || op == 1 || op == 2 || op == 3 {
        return op;
    }
    if op == 4 {
        return a;
    }

    if op == 5 {
        return b;
    }

    if op == 6 {
        return c;
    }
    panic!("{op} is not a valid combo operator");
}

fn literal_operand(op: usize) -> usize {
    op
}

fn generate(instructions: &[(Op, usize)], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut a = a;
    let mut b = b;
    let mut c = c;
    let mut output = Vec::new();
    let mut idx = 0;
    loop {
        if idx >= instructions.len() {
            break;
        }

        let opcode = instructions[idx].0;
        let operand = instructions[idx].1;

        // operand
        match opcode {
            Op::Adv => {
                a = a / pow(2, combo_operand(a, b, c, operand));
                idx += 1;
            }
            Op::Bxl => {
                b = b.bitxor(literal_operand(operand));
                idx += 1;
            }
            Op::Bst => {
                b = combo_operand(a, b, c, operand) % 8;
                idx += 1;
            }
            Op::Jnz => {
                if a != 0 {
                    idx = literal_operand(operand) / 2;
                } else {
                    idx += 1;
                }
            }
            Op::Bxc => {
                b = b.bitxor(c);
                idx += 1;
            }
            Op::Out => {
                output.push(combo_operand(a, b, c, operand) % 8);
                idx += 1;
            }
            Op::Bdv => {
                b = a / pow(2, combo_operand(a, b, c, operand));
                idx += 1;
            }
            Op::Cdv => {
                c = a / pow(2, combo_operand(a, b, c, operand));
                idx += 1;
            }
        }
    }
    output
}

fn solve<const PART: usize>(input: &str) -> String {
    let batches: Vec<&str> = input.split("\n\n").collect();
    let registers: Vec<&str> = batches[0].split('\n').collect();
    let a_s = io::tokenize(registers[0], " ");
    let a: usize = io::parse_num(a_s[2]);
    let b_s = io::tokenize(registers[1], " ");
    let b: usize = io::parse_num(b_s[2]);
    let c_s = io::tokenize(registers[2], " ");
    let c: usize = io::parse_num(c_s[2]);

    let ins_s: Vec<&str> = batches[1].split(' ').collect();
    let ins: Vec<usize> = io::tokenize_nums(ins_s[1], ",");

    let mut instructions: Vec<(Op, usize)> = Vec::new();
    let mut ptr = 0;
    loop {
        if ptr >= ins.len() {
            break;
        }
        instructions.push((OPCODES[ins[ptr]], ins[ptr + 1]));
        ptr += 2;
    }

    if PART == 1 {
        let output = generate(&instructions, a, b, c);
        output.iter().join(",")
    } else {
        let mut candidates = vec![0];
        for d in (0..ins.len()).rev() {
            let mut new_candidates: Vec<usize> = Vec::new();

            for candidate in candidates {
                for test in candidate * 8..candidate * 8 + 8 {
                    let output: Vec<usize> = generate(&instructions, test, b, c);
                    if output == ins[d..] {
                        new_candidates.push(test);
                    }
                }
            }

            new_candidates.sort_unstable();
            new_candidates.dedup();
            candidates = new_candidates;
        }

        candidates[0].to_string()
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
        let sample_input1 = "Register A: 729\nRegister B: 0\nRegister C: 0\n\nProgram: 0,1,5,4,3,0";
        assert_eq!(solve::<1>(sample_input1), "4,6,3,5,6,3,5,2,1,0");
        let sample_input2 =
            "Register A: 2024\nRegister B: 0\nRegister C: 0\n\nProgram: 0,3,5,4,3,0";
        assert_eq!(solve::<2>(sample_input2), "117440");
        let sample_input3 =
            "Register A: 117440\nRegister B: 0\nRegister C: 0\n\nProgram: 0,3,5,4,3,0";
        assert_eq!(solve::<1>(sample_input3), "0,3,5,4,3,0");
    }
}
