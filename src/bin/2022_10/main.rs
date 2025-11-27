use aoc::common;

fn register_history(input_lines: &str) -> Vec<i32> {
    let mut register: Vec<i32> = vec![1];

    for line in input_lines.split('\n') {
        let instr = line.split(' ').collect::<Vec<&str>>();
        let val = *register.last().unwrap();
        if instr[0] == "noop" {
            register.push(val);
        } else if instr[0] == "addx" {
            let dx = instr[1].parse::<i32>().unwrap();
            register.push(val);
            register.push(val + dx);
        }
    }
    register
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
fn part1(input_lines: &str) -> i32 {
    const CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let mut ans = 0;
    let register = register_history(input_lines);

    for cycle in CYCLES {
        ans += cycle as i32 * register[cycle - 1];
    }

    ans
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
fn part2(input_lines: &str) -> common::GridDisplay {
    const W: usize = 40;
    const H: usize = 6;
    let mut ans = common::GridDisplay { rows: Vec::new() };
    let register = register_history(input_lines);

    for row in 0..H {
        let mut crt_row = String::new();
        for col in 0..W {
            let idx = W * row + col;
            if col as i32 - 1 == register[idx]
                || col as i32 == register[idx]
                || col as i32 + 1 == register[idx]
            {
                crt_row.push('#');
            } else {
                crt_row.push('.');
            }
        }
        ans.rows.push(crt_row);
    }

    ans
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, part1, true);
        common::timed(&input, part2, false);
    }
}
