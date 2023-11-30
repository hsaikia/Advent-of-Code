const INPUT: [(&str, &str); 2] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

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

fn part1(input_lines: &str) {
    let mut ans = 0;
    let register = register_history(input_lines);

    const CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

    for cycle in CYCLES {
        ans += cycle as i32 * register[cycle - 1];
    }

    println!("Part 1 Answer : {ans}");
}

fn part2(input_lines: &str) {
    let mut ans: Vec<String> = Vec::new();
    let register = register_history(input_lines);

    const W: usize = 40;
    const H: usize = 6;

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
        ans.push(crt_row);
    }

    for s in &ans {
        println!("{s}");
    }
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        part1(input.1);
        part2(input.1);
    }
}
