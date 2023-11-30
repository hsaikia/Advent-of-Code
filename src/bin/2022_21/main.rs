use aoc::io;
use std::{cmp::Ordering, collections::HashMap};

const INPUT: [(&str, &str); 2] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

#[derive(Debug)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Yell<'a> {
    Call(&'a str, Operator, &'a str),
    Yield(i64),
}

fn find<'a>(
    monkey_name: &'a str,
    ops: &HashMap<&'a str, Yell<'a>>,
    values: &mut HashMap<&'a str, Option<i64>>,
) -> i64 {
    if let Some(x) = values.get(monkey_name) {
        return x.unwrap();
    }

    if let Some(opt) = ops.get(monkey_name) {
        match opt {
            Yell::Call(x, op, y) => {
                let ret = match op {
                    Operator::Plus => find(x, ops, values) + find(y, ops, values),
                    Operator::Minus => find(x, ops, values) - find(y, ops, values),
                    Operator::Multiply => find(x, ops, values) * find(y, ops, values),
                    Operator::Divide => find(x, ops, values) / find(y, ops, values),
                };
                values.entry(monkey_name).and_modify(|val| *val = Some(ret));
                return ret;
            }
            Yell::Yield(value) => return *value,
        }
    }
    panic!("Should not reach here! {} Monkey not found!", monkey_name);
}

fn part1(input_lines: &str) {
    let mut ops: HashMap<&str, Yell> = HashMap::new();

    for line in input_lines.split('\n') {
        let tokens = io::tokenize(line, " ");

        match tokens.len().cmp(&2) {
            Ordering::Greater => {
                match tokens[2] {
                    "+" => ops.insert(
                        &tokens[0][..tokens[0].len() - 1],
                        Yell::Call(tokens[1], Operator::Plus, tokens[3]),
                    ),
                    "-" => ops.insert(
                        &tokens[0][..tokens[0].len() - 1],
                        Yell::Call(tokens[1], Operator::Minus, tokens[3]),
                    ),
                    "*" => ops.insert(
                        &tokens[0][..tokens[0].len() - 1],
                        Yell::Call(tokens[1], Operator::Multiply, tokens[3]),
                    ),
                    "/" => ops.insert(
                        &tokens[0][..tokens[0].len() - 1],
                        Yell::Call(tokens[1], Operator::Divide, tokens[3]),
                    ),
                    _ => panic!("Wrong Operator!"),
                };
            }
            Ordering::Equal => {
                ops.insert(
                    &tokens[0][..tokens[0].len() - 1],
                    Yell::Yield(tokens[1].parse::<i64>().unwrap()),
                );
            }
            Ordering::Less => (),
        }
    }

    //println!("{:?}", ops);
    let mut values: HashMap<&str, Option<i64>> = HashMap::new();
    println!("Part 1 Answer : {}", find("root", &ops, &mut values));
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        part1(input.1);
        //part2(input.1);
    }
}
