use aoc::io;

const FILES: [&str; 2] = [
    "./src/bin/2022_05/sample_input.txt",
    "./src/bin/2022_05/input.txt",
];

fn part1(stacks: &[Vec<char>], instructions: &Vec<Vec<usize>>) {
    let mut stacks = stacks.to_owned();
    // Perform instructions
    for ins in instructions {
        for _ in 0..ins[0] {
            let x = stacks[ins[1] - 1].pop().unwrap();
            stacks[ins[2] - 1].push(x);
        }
    }

    let ans = stacks.iter().flat_map(|v| v.last()).collect::<String>();
    println!("Part 1 Answer {}", ans);
}

fn part2(stacks: &[Vec<char>], instructions: &Vec<Vec<usize>>) {
    let mut stacks = stacks.to_owned();
    // Perform instructions
    for ins in instructions {
        let l = stacks[ins[1] - 1].len();
        let tail = stacks[ins[1] - 1][l - ins[0]..].to_vec();

        stacks[ins[2] - 1].extend(tail);
        stacks[ins[1] - 1].truncate(l - ins[0]);
    }

    let ans = stacks.iter().flat_map(|v| v.last()).collect::<String>();
    println!("Part 2 Answer {}", ans);
}

fn main() {
    for filename in FILES {
        println!("Input file {filename}");
        if let Ok(lines) = io::read_lines(filename) {
            let input_lines = lines.flatten().collect::<Vec<String>>();

            let mut reading_config = true;
            let mut stacks: Vec<Vec<char>> = Vec::new();
            let mut instructions: Vec<Vec<usize>> = Vec::new();

            for line in input_lines {
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
                            .flat_map(|s| s.parse::<usize>())
                            .collect::<Vec<usize>>(),
                    );
                }
            }

            // Reverse stack elements
            for stack in &mut stacks {
                stack.reverse();
            }

            part1(&stacks, &instructions);
            part2(&stacks, &instructions);
        }
    }
}
