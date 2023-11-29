use aoc::io;

const FILES: [&str; 2] = [
    "./src/bin/2022_25/sample_input.txt",
    "./src/bin/2022_25/input.txt",
];

fn encode(num: i64) -> String {
    let mut num_snafu: String = String::new();
    let mut num = num;
    while num > 0 {
        let x = num % 5;
        match x {
            0 => num_snafu.push('0'),
            1 => num_snafu.push('1'),
            2 => num_snafu.push('2'),
            3 => {
                num_snafu.push('=');
                num += 2
            }
            4 => {
                num_snafu.push('-');
                num += 1
            }
            _ => (),
        }
        num /= 5;
    }
    num_snafu.chars().rev().collect()
}

fn decode(num_snafu: &str) -> i64 {
    let mut num: i64 = 0;
    const BASE: i64 = 5;
    for (i, x) in num_snafu.chars().rev().enumerate() {
        match x {
            '1' => num += BASE.pow(i as u32),
            '2' => num += 2 * BASE.pow(i as u32),
            '-' => num -= BASE.pow(i as u32),
            '=' => num -= 2 * BASE.pow(i as u32),
            _ => (),
        }
    }
    num
}

fn part1(input_lines: &Vec<String>) {
    let mut sum: i64 = 0;

    for line in input_lines {
        sum += decode(line);
    }

    println!("Decoded Sum : {}", sum);
    println!("Part 1 Answer : {}", encode(sum))
}

fn main() {
    for filename in FILES {
        println!("Input file {filename}");
        if let Ok(lines) = io::read_lines(filename) {
            let input_lines = lines.flatten().collect::<Vec<String>>();
            part1(&input_lines);
            //part2(&input_lines);
        }
    }
}
