use std::collections::HashMap;

use aoc::{
    common::{self, HashMapVector},
    grid::Grid,
};
use itertools::Itertools;

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

fn solve<const PART1: bool>(input: &str) -> u32 {
    let grid = Grid::from_str(input, |c| c);
    let symbols = get_symbols(input);

    let mut ans1 = 0;
    // Map of each symbol to all part numbers surrounding it
    let mut part_numbers_map: HashMap<usize, Vec<u32>> = HashMap::new();
    for i in 0..grid.rows {
        let mut num = 0;
        let mut neighboring_symbols: Vec<(char, usize)> = Vec::new();

        for j in 0..grid.cols {
            let idx = (i, j);
            let ch = grid.get(&idx);
            if ch.is_ascii_digit() {
                num = 10 * num + ch.to_digit(10).unwrap();
                let ncells = grid.adjacent_8(&idx);
                for nidx in &ncells {
                    let ch1 = grid.get(nidx);
                    if symbols.contains(&ch1) {
                        neighboring_symbols.push((ch1, grid.to_flat_idx(nidx)));
                    }
                }
            } else if num > 0 {
                if !neighboring_symbols.is_empty() {
                    ans1 += num;

                    neighboring_symbols =
                        neighboring_symbols.into_iter().unique().collect::<Vec<_>>();

                    for (ch, key) in &neighboring_symbols {
                        if *ch == '*' {
                            part_numbers_map.add_to_vector_hashmap(key, num);
                        }
                    }

                    neighboring_symbols.clear();
                }
                num = 0;
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

    if PART1 {
        return ans1;
    }

    part_numbers_map
        .values()
        .filter_map(|vals| {
            if vals.len() == 2 {
                Some(vals[0] * vals[1])
            } else {
                None
            }
        })
        .sum::<u32>()
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, solve::<true>, true);
        common::timed(&input, solve::<false>, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        assert_eq!(solve::<true>(sample_input), 4361);
        assert_eq!(solve::<false>(sample_input), 467835);
    }
}
