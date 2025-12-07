use std::collections::{HashMap, HashSet};

use aoc::{
    common,
    grid::{CellIndex, Grid},
};

fn solve2(input: &str) -> usize {
    let mut ans = 0;
    let grid = Grid::from_str(input, |c| c);
    let mut pos = grid.positions(&'S');
    pos[0].0 += 1;
    let tcs = grid.positions(&'^');
    let emt = grid.positions(&'.');

    let mut post = HashMap::new();
    post.insert(pos[0], 1);

    while !post.is_empty() {
        let mut new_post: HashMap<CellIndex, usize> = HashMap::new();

        for (p, t) in post {
            if emt.contains(&p) {
                let new_p = (p.0 + 1, p.1);
                if let Some(x) = new_post.get_mut(&new_p) {
                    *x += t;
                } else {
                    new_post.insert(new_p, t);
                }
            }
            if tcs.contains(&p) {
                if p.1 > 0 {
                    let new_p = (p.0 + 1, p.1 - 1);
                    if let Some(x) = new_post.get_mut(&new_p) {
                        *x += t;
                    } else {
                        new_post.insert(new_p, t);
                    }
                }
                if p.1 + 1 < grid.cols {
                    let new_p = (p.0 + 1, p.1 + 1);
                    if let Some(x) = new_post.get_mut(&new_p) {
                        *x += t;
                    } else {
                        new_post.insert(new_p, t);
                    }
                }
            }
        }

        if new_post.is_empty() {
            break;
        }

        ans = new_post.values().sum::<usize>();
        post = new_post;
    }

    //grid.print();
    ans
}

fn solve<const PART: usize>(input: &str) -> usize {
    let mut ans = 0;
    let grid = Grid::from_str(input, |c| c);
    let mut pos = grid.positions(&'S');
    pos[0].0 += 1;
    let tcs = grid.positions(&'^');
    let emt = grid.positions(&'.');

    while !pos.is_empty() {
        let cnt_emt: Vec<CellIndex> = emt.iter().filter(|x| pos.contains(x)).map(|x| *x).collect();
        let cnt: Vec<CellIndex> = tcs.iter().filter(|x| pos.contains(x)).map(|x| *x).collect();
        let mut new_pos = HashSet::new();

        for c in &cnt_emt {
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
        common::timed(&input, solve::<1>, true);
        common::timed(&input, solve2, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............";
        assert_eq!(solve::<1>(sample_input), 21);
        assert_eq!(solve2(sample_input), 40);
    }
}
