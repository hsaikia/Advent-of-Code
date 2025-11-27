use std::collections::HashMap;

use aoc::{common, io};

// Part 2 is reverse engineering the output.
// All zPP outputs should be output from a A XOR B connection where either A = xPP XOR yPP or B = xPP XOR yPP
// Correct all anomalies starting from lowest to highest bit order
const ANS_PART2: [&str; 8] = ["qmd", "tnt", "z06", "hwk", "z31", "hpc", "z37", "cgr"];

fn value_of_register(val: &HashMap<&str, usize>, register: char) -> usize {
    let mut ans: String = String::new();
    for b1 in '0'..='9' {
        for b2 in '0'..='9' {
            let s: String = [register, b1, b2].iter().collect();
            //println!("Looking for {}", s);
            if let Some(x) = val.get(s.as_str()) {
                //println!("{} => {}", s, x);
                if *x == 1 {
                    ans += "1";
                } else {
                    ans += "0";
                }
            } else {
                ans = ans.chars().rev().collect();
                return usize::from_str_radix(ans.as_str(), 2).unwrap();
            }
        }
    }
    0
}

fn bad_bits(val: &HashMap<&str, usize>) {
    let mut ans: Vec<Vec<usize>> = vec![vec![], vec![], vec![]];
    for (i, register) in ['x', 'y', 'z'].iter().enumerate() {
        for b1 in '0'..='9' {
            let mut stop = false;
            for b2 in '0'..='9' {
                let s: String = [*register, b1, b2].iter().collect();
                //println!("Looking for {}", s);
                if let Some(x) = val.get(s.as_str()) {
                    //println!("{} => {}", s, x);
                    if *x == 1 {
                        ans[i].push(1);
                    } else {
                        ans[i].push(0);
                    }
                } else {
                    stop = true;
                    break;
                }
            }

            if stop {
                break;
            }
        }
    }

    let mut rem = 0;
    for (b, (x, y)) in ans[0].iter().zip(ans[1].iter()).enumerate() {
        //println!("X {} Y {} Z {} R {}", x, y, ans[2][b], rem);
        let mut z = x + y + rem;
        if z == 0 || z == 1 {
            rem = 0;
        } else if z == 2 {
            z = 0;
            rem = 1;
        } else if z == 3 {
            z = 1;
            rem = 1;
        }

        // Use this for debugging Part 2
        if z != ans[2][b] {
            println!(
                "Byte {} is corrupt. Calculated Z = {} Actual Z = {}",
                b, z, ans[2][b]
            );
        }
    }
}

fn solve<const PART: usize>(input: &str) -> String {
    if PART == 2 {
        let mut test = ANS_PART2;
        test.sort_unstable();
        return test.join(",");
    }

    let batches = io::line_batches(input);
    let mut val: HashMap<&str, usize> = HashMap::new();
    for line in &batches[0] {
        let init = io::tokenize(line, ": ");
        val.insert(init[0], io::parse_num::<usize>(init[1]));
    }

    let n = batches[1].len();
    println!("total conn {n}");

    loop {
        let mut all_done = true;
        for line in &batches[1] {
            let expr = io::tokenize(line, " -> ");
            let tokens = io::tokenize(expr[0], " ");
            //println!("Calculating value for {}", expr[1]);
            if tokens[1] == "AND" {
                if let Some(v1) = val.get(tokens[0]) {
                    if let Some(v2) = val.get(tokens[2]) {
                        val.insert(expr[1], v1 & v2);
                    } else {
                        all_done = false;
                    }
                } else {
                    all_done = false;
                }
            } else if tokens[1] == "OR" {
                if let Some(v1) = val.get(tokens[0]) {
                    if let Some(v2) = val.get(tokens[2]) {
                        val.insert(expr[1], v1 | v2);
                    } else {
                        all_done = false;
                    }
                } else {
                    all_done = false;
                }
            } else if tokens[1] == "XOR" {
                if let Some(v1) = val.get(tokens[0]) {
                    if let Some(v2) = val.get(tokens[2]) {
                        val.insert(expr[1], v1 ^ v2);
                    } else {
                        all_done = false;
                    }
                } else {
                    all_done = false;
                }
            }
        }
        if all_done {
            break;
        }
    }

    if PART == 2 {
        bad_bits(&val);
        let x = value_of_register(&val, 'x');
        let y = value_of_register(&val, 'y');
        println!("X = {} Y = {} Calculated Z = {}", x, y, x + y);
    }

    value_of_register(&val, 'z').to_string()
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
        let sample_input1 = "x00: 1\nx01: 1\nx02: 1\ny00: 0\ny01: 1\ny02: 0\n\nx00 AND y00 -> z00\nx01 XOR y01 -> z01\nx02 OR y02 -> z02";
        assert_eq!(solve::<1>(sample_input1), "4");
        let sample_input2 = "x00: 1\nx01: 0\nx02: 1\nx03: 1\nx04: 0\ny00: 1\ny01: 1\ny02: 1\ny03: 1\ny04: 1\n\ny02 OR x01 -> tnw\nx00 OR x03 -> fst\nvdt OR tnw -> bfw\nbfw AND frj -> z10\nffh OR nrd -> bqk\ny00 AND y03 -> djm\ny03 OR y00 -> psh\nbqk OR frj -> z08\ntnw OR fst -> frj\ngnj AND tgd -> z11\nbfw XOR mjb -> z00\nx03 OR x00 -> vdt\ngnj AND wpb -> z02\nx04 AND y00 -> kjc\ndjm OR pbm -> qhw\nnrd AND vdt -> hwm\nkjc AND fst -> rvg\ny04 OR y02 -> fgs\ny01 AND x02 -> pbm\nntg OR kjc -> kwq\npsh XOR fgs -> tgd\nqhw XOR tgd -> z09\npbm OR djm -> kpj\nx03 XOR y03 -> ffh\nx00 XOR y04 -> ntg\nbfw OR bqk -> z06\nnrd XOR fgs -> wpb\nfrj XOR qhw -> z04\nbqk OR frj -> z07\ny03 OR x01 -> nrd\nhwm AND bqk -> z03\ntgd XOR rvg -> z12\ntnw OR pbm -> gnj\nntg XOR fgs -> mjb\nkwq OR kpj -> z05\ntgd XOR rvg -> z01";
        assert_eq!(solve::<1>(sample_input2), "2024");
    }
}
