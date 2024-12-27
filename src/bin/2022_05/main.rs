use aoc::common;

fn part1(stacks: &[Vec<char>], instructions: &Vec<Vec<usize>>) -> String {
    let mut stacks = stacks.to_owned();
    // Perform instructions
    for ins in instructions {
        for _ in 0..ins[0] {
            let x = stacks[ins[1] - 1].pop().unwrap();
            stacks[ins[2] - 1].push(x);
        }
    }

    stacks.iter().filter_map(|v| v.last()).collect::<String>()
}

fn part2(stacks: &[Vec<char>], instructions: &Vec<Vec<usize>>) -> String {
    let mut stacks = stacks.to_owned();
    // Perform instructions
    for ins in instructions {
        let l = stacks[ins[1] - 1].len();
        let tail = stacks[ins[1] - 1][l - ins[0]..].to_vec();

        stacks[ins[2] - 1].extend(tail);
        stacks[ins[1] - 1].truncate(l - ins[0]);
    }

    stacks.iter().filter_map(|v| v.last()).collect::<String>()
}

fn process_and_solve<const PART1: bool>(input: &str) -> String {
    let mut reading_config = true;
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<Vec<usize>> = Vec::new();

    for line in input.split('\n') {
        if line.is_empty() {
            reading_config = false;
            continue;
        }

        if reading_config && !line.contains('[') {
            continue;
        }

        if reading_config {
            let chars = line.chars().collect::<Vec<char>>();
            //println!("{:?}", chars);
            let mut pos = 1;
            let mut stack_idx = 0;
            while pos < chars.len() {
                if stack_idx == stacks.len() {
                    stacks.push(vec![]);
                }

                if chars[pos] != ' ' {
                    stacks[stack_idx].push(chars[pos]);
                }
                stack_idx += 1;
                pos += 4;
            }
        } else {
            instructions.push(
                line.split(' ')
                    .flat_map(str::parse::<usize>)
                    .collect::<Vec<usize>>(),
            );
        }
    }

    // Reverse stack elements
    for stack in &mut stacks {
        stack.reverse();
    }

    if PART1 {
        return part1(&stacks, &instructions);
    }

    part2(&stacks, &instructions)
}

fn main() {
    let input = common::get_input();
    common::timed(&input, process_and_solve::<true>, true);
    common::timed(&input, process_and_solve::<false>, false);
}
