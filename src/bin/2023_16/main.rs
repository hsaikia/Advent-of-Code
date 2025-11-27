use std::collections::{HashSet, VecDeque};

use aoc::{
    common,
    grid::{CellDir, CellIndex, Grid},
};

fn solve(grid: &Grid<char>, start: CellIndex, start_dir: CellDir) -> usize {
    let mut vis = Grid::<Vec<CellDir>>::new(grid.rows, grid.cols, Vec::new());

    let mut queue: VecDeque<(CellIndex, CellDir)> = VecDeque::new();
    queue.push_back((start, start_dir));

    let mut cell_set: HashSet<usize> = HashSet::new();
    while !queue.is_empty() {
        let (cell_idx, d) = queue.pop_front().unwrap();
        cell_set.insert(grid.to_flat_idx(&cell_idx));

        let mut dirs = vis.get(&cell_idx).clone();
        if dirs.contains(&d) {
            continue;
        }
        dirs.push(d);
        vis.set(&cell_idx, dirs);

        let cell = grid.get(&cell_idx);

        let next_dirs = match cell {
            '-' => {
                vec![(0, 1), (0, -1)]
            }
            '|' => {
                vec![(1, 0), (-1, 0)]
            }
            '/' => {
                vec![(-d.1, -d.0)]
            }
            '\\' => {
                vec![(d.1, d.0)]
            }
            _ => {
                vec![d]
            }
        };

        for nd in &next_dirs {
            if let Some(nc) = grid.cell_in_direction(&cell_idx, nd) {
                queue.push_back((nc, *nd));
            }
        }
    }

    cell_set.len()
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input, |c| c);
    solve(&grid, (0, 0), (0, 1))
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_str(input, |c| c);
    let mut ans = 0;
    for i in 0..grid.rows {
        ans = ans.max(solve(&grid, (i, 0), (0, 1)));
        ans = ans.max(solve(&grid, (i, grid.cols - 1), (0, -1)));
    }
    for j in 0..grid.cols {
        ans = ans.max(solve(&grid, (0, j), (1, 0)));
        ans = ans.max(solve(&grid, (grid.rows - 1, j), (-1, 0)));
    }

    ans
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, part1, true);
        common::timed(&input, part2, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input = ".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|....";
        assert_eq!(part1(sample_input), 46);
        assert_eq!(part2(sample_input), 51);
    }
}
