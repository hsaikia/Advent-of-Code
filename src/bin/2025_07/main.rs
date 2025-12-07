use std::collections::{HashMap, HashSet};

use aoc::{
    common,
    grid::{CellIndex, Grid},
};

fn solve2(input: &str) -> usize {
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
                if let Some(x) = new_space_time.get_mut(&new_p) {
                    *x += t;
                } else {
                    new_space_time.insert(new_p, t);
                }
            }
            if tachyons.contains(&p) {
                if p.1 > 0 {
                    let new_p = (p.0 + 1, p.1 - 1);
                    if let Some(x) = new_space_time.get_mut(&new_p) {
                        *x += t;
                    } else {
                        new_space_time.insert(new_p, t);
                    }
                }
                if p.1 + 1 < grid.cols {
                    let new_p = (p.0 + 1, p.1 + 1);
                    if let Some(x) = new_space_time.get_mut(&new_p) {
                        *x += t;
                    } else {
                        new_space_time.insert(new_p, t);
                    }
                }
            }
        }

        if new_space_time.is_empty() {
            break;
        }

        ans2 = new_space_time.values().sum::<usize>();
        space_time = new_space_time;
    }

    //grid.print();
    ans2
}

fn solve1(input: &str) -> usize {
    let mut ans = 0;
    let grid = Grid::from_str(input, |c| c);
    let mut pos = grid.positions(&'S');
    pos[0].0 += 1;
    let tachyons = grid.positions(&'^');
    let empty_spaces = grid.positions(&'.');

    while !pos.is_empty() {
        let cnt_empty_spaces: Vec<CellIndex> = empty_spaces
            .iter()
            .filter(|x| pos.contains(x))
            .map(|x| *x)
            .collect();
        let cnt: Vec<CellIndex> = tachyons
            .iter()
            .filter(|x| pos.contains(x))
            .map(|x| *x)
            .collect();
        let mut new_pos = HashSet::new();

        for c in &cnt_empty_spaces {
            new_pos.insert((c.0 + 1, c.1));
        }

        for c in &cnt {
            if c.1 > 0 {
                new_pos.insert((c.0 + 1, c.1 - 1));
            }
            if c.1 + 1 < grid.cols {
                new_pos.insert((c.0 + 1, c.1 + 1));
            }
        }
        pos = new_pos.into_iter().collect::<Vec<CellIndex>>();
        ans += cnt.len();
    }

    //grid.print();
    ans
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, solve1, true);
        common::timed(&input, solve2, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";
        assert_eq!(solve1(sample_input), 21);
        assert_eq!(solve2(sample_input), 40);
    }
}
