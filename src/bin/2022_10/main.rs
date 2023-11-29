use aoc::io;

const FILES: [&str; 2] = [
    "./src/bin/2022_10/sample_input.txt",
    "./src/bin/2022_10/input.txt",
];

fn register_history(input_lines: &Vec<String>) -> Vec<i32> {
    let mut register: Vec<i32> = vec![1];

    for line in input_lines {
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

fn part1(input_lines: &Vec<String>) {
    let mut ans = 0;
    let register = register_history(input_lines);

    const CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

    for cycle in CYCLES {
        //println!("Cycle {} has X {}", cycle, register[cycle - 1]);
        ans += cycle as i32 * register[cycle - 1];
    }

    println!("Part 1 Answer : {ans}");
}

fn part2(input_lines: &Vec<String>) {
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
    for filename in FILES {
        println!("Input file {filename}");
        if let Ok(lines) = io::read_lines(filename) {
            let input_lines = lines.flatten().collect::<Vec<String>>();
            part1(&input_lines);
            part2(&input_lines);
        }
    }
}
