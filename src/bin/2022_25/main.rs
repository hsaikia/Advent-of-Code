use aoc::common;

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
                num += 2;
            }
            4 => {
                num_snafu.push('-');
                num += 1;
            }
            _ => (),
        }
        num /= 5;
    }
    num_snafu.chars().rev().collect()
}

fn decode(num_snafu: &str) -> i64 {
    const BASE: i64 = 5;
    let mut num: i64 = 0;
    for (i, x) in num_snafu.chars().rev().enumerate() {
        match x {
            '1' => num += BASE.pow(u32::try_from(i).unwrap()),
            '2' => num += 2 * BASE.pow(u32::try_from(i).unwrap()),
            '-' => num -= BASE.pow(u32::try_from(i).unwrap()),
            '=' => num -= 2 * BASE.pow(u32::try_from(i).unwrap()),
            _ => (),
        }
    }
    num
}

fn part1(input_lines: &str) -> String {
    let mut sum: i64 = 0;

    for line in input_lines.split('\n') {
        sum += decode(line);
    }

    // println!("Decoded Sum : {}", sum);
    encode(sum)
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
}
