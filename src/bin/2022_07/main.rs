use aoc::common;
use std::collections::HashMap;

fn process_and_solve<const PART1: bool>(input_lines: &str) -> usize {
    const DIR_SEP: &str = "/";
    let mut curr_dir_path: Vec<String> = Vec::new();
    let mut curr_total_filesizes: usize = 0;
    let mut dir_size_map: HashMap<String, usize> = HashMap::new();
    let mut reading = false;

    for line in input_lines.split('\n') {
        let tokens = line
            .split(' ')
            .map(std::string::ToString::to_string)
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

    if PART1 {
        return part1(&dir_size_map);
    }

    part2(&dir_size_map)
}

fn part1(dir_size_map: &HashMap<String, usize>) -> usize {
    let mut sum = 0;
    for v in dir_size_map.values() {
        if *v <= 100_000 {
            sum += *v;
        }
    }

    sum
}

fn part2(dir_size_map: &HashMap<String, usize>) -> usize {
    const TOTAL_DISK_SPACE: usize = 70_000_000;
    const UNUSED_SPACE_REQUIRED: usize = 30_000_000;

    let unused_space = TOTAL_DISK_SPACE - dir_size_map.get("/").unwrap();
    let minimimum_space_to_free = UNUSED_SPACE_REQUIRED - unused_space;

    let mut diff = usize::MAX;
    for dir_size in dir_size_map.values() {
        if *dir_size > minimimum_space_to_free {
            diff = diff.min(dir_size.abs_diff(minimimum_space_to_free));
        }
    }

    minimimum_space_to_free + diff
}

fn main() {
    let input = common::get_input();
    common::timed(&input, process_and_solve::<true>, true);
    common::timed(&input, process_and_solve::<false>, false);
}
