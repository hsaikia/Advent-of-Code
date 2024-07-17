use aoc::common;

fn part1(input: &str) -> i32 {
    input.chars().filter(|&c| c == '(').count() as i32 - input.chars().filter(|&c| c == ')').count() as i32
}

fn part2(input: &str) -> usize {
    let mut ret = 1;
    let mut floor = 0;
    for (i, e) in input.chars().enumerate() {
        if e == '(' {
            floor += 1;
        } else {
            floor -= 1; 
        }

        if floor == -1 {
            ret = i + 1;
            break;
        }
    }
    ret
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    common::timed(&input, part2, false);
}
