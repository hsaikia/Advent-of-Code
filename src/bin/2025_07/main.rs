use std::collections::HashMap;

use aoc::{
    common::{self, HashMapCount},
    grid::{CellIndex, Grid},
};
use itertools::Itertools;

fn solve(input: &str) -> (usize, usize) {
    let mut ans1 = 0;
    let mut ans2 = 0;
    let grid = Grid::from_str(input, |c| c);
    let mut pos = grid.positions(&'S');
    pos[0].0 += 1;
    let tachyons = grid.positions(&'^');
    let empty_spaces = grid.positions(&'.');

    let mut space_time = HashMap::new();
    space_time.insert(pos[0], 1);

    while !space_time.is_empty() {
        let mut new_space_time: HashMap<CellIndex, usize> = HashMap::new();

        for (p, t) in space_time {
            if empty_spaces.contains(&p) {
                let new_p = (p.0 + 1, p.1);
                new_space_time.insert_with_count(&new_p, t);
            }
            if tachyons.contains(&p) {
                if p.1 > 0 {
                    let new_p = (p.0 + 1, p.1 - 1);
                    new_space_time.insert_with_count(&new_p, t);
                }
                if p.1 + 1 < grid.cols {
                    let new_p = (p.0 + 1, p.1 + 1);
                    new_space_time.insert_with_count(&new_p, t);
                }
            }
        }

        if new_space_time.is_empty() {
            break;
        }

        ans1 += tachyons
            .iter()
            .filter(|x| new_space_time.keys().contains(x))
            .count();
        ans2 = new_space_time.values().sum::<usize>();
        space_time = new_space_time;
    }

    //grid.print();
    (ans1, ans2)
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, solve, true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";
        assert_eq!(solve(sample_input), (21, 40));
    }
}
