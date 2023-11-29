use aoc::io;
use std::collections::HashMap;

const FILES: [&str; 2] = [
    "./src/bin/2022_07/sample_input.txt",
    "./src/bin/2022_07/input.txt",
];

fn process_commands(input_lines: &Vec<String>) -> HashMap<String, usize> {
    let mut curr_dir_path: Vec<String> = Vec::new();
    let mut curr_total_filesizes: usize = 0;
    let mut dir_size_map: HashMap<String, usize> = HashMap::new();
    let mut reading = false;

    const DIR_SEP: &str = "/";

    for line in input_lines {
        let tokens = line
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if reading && tokens[0] != "$" && tokens[0] != "dir" {
            curr_total_filesizes += tokens[0].parse::<usize>().unwrap();
        }
        if tokens[0] == "$" {
            reading = false;
            if tokens[1] == "cd" {
                let curr_dir = curr_dir_path.join(DIR_SEP);

                if let std::collections::hash_map::Entry::Vacant(e) = dir_size_map.entry(curr_dir) {
                    e.insert(curr_total_filesizes);
                    curr_total_filesizes = 0;
                }

                if tokens[2] == ".." {
                    curr_dir_path.pop();
                } else {
                    curr_dir_path.push(tokens[2].clone());
                }
            } else if tokens[1] == "ls" {
                reading = true;
            }
        }
    }

    let curr_dir = curr_dir_path.join(DIR_SEP);
    dir_size_map.entry(curr_dir).or_insert(curr_total_filesizes);

    let dir_size_tmp = dir_size_map.clone();

    for (sub_dir, files_size) in &dir_size_tmp {
        for dir in dir_size_tmp.keys() {
            if sub_dir.len() > dir.len() && sub_dir[0..dir.len()] == *dir {
                dir_size_map
                    .entry(dir.to_string())
                    .and_modify(|val| *val += files_size);
            }
        }
    }

    dir_size_map
}

fn part1(dir_size_map: &HashMap<String, usize>) {
    let mut sum = 0;
    for v in dir_size_map.values() {
        if *v <= 100000 {
            sum += *v;
        }
    }

    println!("Part 1 Answer : {}", sum);
}

fn part2(dir_size_map: &HashMap<String, usize>) {
    const TOTAL_DISK_SPACE: usize = 70000000;
    const UNUSED_SPACE_REQUIRED: usize = 30000000;

    let unused_space = TOTAL_DISK_SPACE - dir_size_map.get("/").unwrap();
    let minimimum_space_to_free = UNUSED_SPACE_REQUIRED - unused_space;

    let mut diff = usize::MAX;
    for dir_size in dir_size_map.values() {
        if *dir_size > minimimum_space_to_free {
            diff = diff.min(dir_size.abs_diff(minimimum_space_to_free));
        }
    }

    println!("Part 2 Answer : {}", minimimum_space_to_free + diff);
}

fn main() {
    for filename in FILES {
        println!("Input file {filename}");
        if let Ok(lines) = io::read_lines(filename) {
            let input_lines = lines.flatten().collect::<Vec<String>>();
            let dir_size_map = process_commands(&input_lines);
            part1(&dir_size_map);
            part2(&dir_size_map);
        }
    }
}
