use std::collections::VecDeque;

use aoc::{grid::Grid, io};

const INPUT: [(&str, &str); 2] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

fn solve(input: &str) {
    let mut ans: usize = 0;
    let lines = input.split('\n').collect::<Vec<_>>();

    let mut special = Vec::new();
    for line in &lines {
        for ch in line.chars() {
            if !ch.is_ascii_digit() && ch != '.' {
                special.push(ch);
            }
        }
    }

    let mut grid = Grid::<char>::new(lines.len(), lines[0].len(), '.');

    for (i, line) in lines.iter().enumerate() {
        grid.set_row(i, line.chars().collect::<Vec<_>>());
    }

    let mut candidates: Grid<bool> = Grid::<bool>::new(lines.len(), lines[0].len(), false);

    let mut seeds: VecDeque<(usize, usize)> = VecDeque::new();

    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if let Some(val) = grid.get(i, j) {
                if val.is_ascii_digit() {
                    let ns = grid.adjacent_8(i, j);
                    for (x, y) in ns {
                        if let Some(val1) = grid.get(x, y) {
                            if special.contains(&val1) {
                                seeds.push_back((i, j));
                                candidates.set(i, j, true);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    while !seeds.is_empty() {
        let x = seeds.pop_front().unwrap();
        let sides = grid.adjacent_2_row(x.0, x.1);
        for (i, j) in sides {
            if !candidates.get(i, j).unwrap() && grid.get(i, j).unwrap().is_ascii_digit() {
                candidates.set(i, j, true);
                seeds.push_back((i, j));
            }
        }
    }

    for i in 0..grid.rows {
        let mut parsed_line = String::new();
        for j in 0..grid.cols {
            if candidates.get(i, j).unwrap() {
                parsed_line.push(grid.get(i, j).unwrap());
            } else {
                parsed_line.push(' ');
            }
        }
        let parsed_line = (0..grid.cols).map(|j| if let Some(true) = candidates.get(i, j) { grid.get(i, j).unwrap() } else {' '}).collect::<String>();
        ans += io::tokenize(&parsed_line, " ").iter().flat_map(|token| io::parse_num::<usize>(token)).sum::<usize>();
    }

    println!("Part1 Answer : {}", ans);
}

fn solve2(input: &str) {
    let mut ans: usize = 0;
    let lines = input.split('\n').collect::<Vec<_>>();

    let mut grid = Grid::<char>::new(lines.len(), lines[0].len(), '.');

    for (i, line) in lines.iter().enumerate() {
        grid.set_row(i, line.chars().collect::<Vec<_>>());
    }

    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if let Some(val) = grid.get(i, j) {
                if val == '*' {
                    let mut seeds: VecDeque<(usize, usize)> = VecDeque::new();
                    let neighbors = grid.adjacent_8(i, j);
                    let mut candidates: Grid<bool> = Grid::<bool>::new(lines.len(), lines[0].len(), false);
                    for (x, y) in &neighbors {
                        if grid.get(*x, *y).unwrap().is_ascii_digit() {
                            seeds.push_back((*x, *y));
                            candidates.set(*x, *y, true);
                        }
                    }

                    while !seeds.is_empty() {
                        let seed = seeds.pop_front().unwrap();
                        let sides = grid.adjacent_2_row(seed.0, seed.1);
                        for (i, j) in sides {
                            if !candidates.get(i, j).unwrap() && grid.get(i, j).unwrap().is_ascii_digit() {
                                candidates.set(i, j, true);
                                seeds.push_back((i, j));
                            }
                        }
                    }

                    let mut nums = Vec::new();

                    for i in 0..grid.rows {
                        let mut parsed_line = String::new();
                        for j in 0..grid.cols {
                            if candidates.get(i, j).unwrap() {
                                parsed_line.push(grid.get(i, j).unwrap());
                            } else {
                                parsed_line.push(' ');
                            }
                        }

                        nums.extend(io::tokenize(&parsed_line, " ").iter().flat_map(|token| io::parse_num::<usize>(token)));
                    }

                    if nums.len() == 2 {
                        //println!("{:?}", nums);
                        ans += nums[0] * nums[1];
                    }

                    
                }
            }
        }
    }

    println!("Part2 Answer : {}", ans);
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        solve(input.1);
        solve2(input.1);
    }
}
