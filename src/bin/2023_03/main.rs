use std::collections::HashMap;

use aoc::{common::HashMapVector, grid::Grid};
use itertools::Itertools;

const INPUT: [(&str, &str); 2] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

fn get_symbols(input: &str) -> Vec<char> {
    input
        .split('\n')
        .map(|l| {
            l.chars()
                .filter(|ch| !ch.is_ascii_digit() && *ch != '.')
                .collect::<Vec<_>>()
        })
        .concat()
        .into_iter()
        .unique()
        .collect()
}

fn solve(input: &str) {
    let grid = Grid::from_str(input, |c| c);
    let symbols = get_symbols(input);

    let mut ans1 = 0;
    // Map of each symbol to all part numbers surrounding it
    let mut part_numbers_map: HashMap<usize, Vec<u32>> = HashMap::new();
    for i in 0..grid.rows {
        let mut num = 0;
        let mut neighboring_symbols: Vec<(char, usize)> = Vec::new();

        for j in 0..grid.cols {
            if let Some(ch) = grid.get(i, j) {
                if ch.is_ascii_digit() {
                    num = 10 * num + ch.to_digit(10).unwrap();
                    let ncells = grid.adjacent_8(i, j);
                    for (x, y) in &ncells {
                        if let Some(ch) = grid.get(*x, *y) {
                            if symbols.contains(&ch) {
                                neighboring_symbols.push((ch, grid.to_flat_idx(*x, *y)));
                            }
                        }
                    }
                } else if num > 0 {
                    if !neighboring_symbols.is_empty() {
                        ans1 += num;

                        neighboring_symbols =
                            neighboring_symbols.into_iter().unique().collect::<Vec<_>>();

                        for (ch, idx) in &neighboring_symbols {
                            if *ch == '*' {
                                part_numbers_map.add_to_vector_hashmap(idx, num);
                            }
                        }

                        neighboring_symbols.clear();
                    }
                    num = 0;
                }
            }
        }

        if num > 0 && !neighboring_symbols.is_empty() {
            for (ch, idx) in &neighboring_symbols {
                if *ch == '*' {
                    part_numbers_map.add_to_vector_hashmap(idx, num);
                }
            }

            ans1 += num;
        }
    }

    let ans2 = part_numbers_map
        .values()
        .filter_map(|vals| {
            if vals.len() == 2 {
                Some(vals[0] * vals[1])
            } else {
                None
            }
        })
        .sum::<u32>();

    println!("Part 1 Answer : {}", ans1);
    println!("Part 2 Answer : {}", ans2);
}

fn main() {
    for (file, input) in INPUT {
        println!("{}", file);
        solve(input);
    }
}
