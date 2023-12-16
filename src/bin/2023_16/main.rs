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
        let ((x, y), d) = queue.pop_front().unwrap();

        //println!("Processing cell {:?} from Dir {:?}", (x, y), d);

        cell_set.insert(grid.to_flat_idx(x, y));
        //dis.set(x, y, '#');

        let mut dirs = vis.get(x, y).unwrap().clone();
        if dirs.contains(&d) {
            continue;
        }
        dirs.push(d);
        vis.set(x, y, dirs);

        let cell = grid.get(x, y).unwrap();

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
        //println!("{:?}", next_dirs);
        for nd in &next_dirs {
            if let Some(nc) = grid.cell_in_direction(x, y, nd.0, nd.1) {
                queue.push_back((nc, *nd));
            }
        }
    }

    //dis.print();
    cell_set.len()
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input, |c| c);
    solve(&grid, (0, 0), (0, 1))
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_str(input, |c| c);
    let mut mx = 0;
    for i in 0..grid.rows {
        mx = mx.max(solve(&grid, (i, 0), (0, 1)));
        mx = mx.max(solve(&grid, (i, grid.cols - 1), (0, -1)));
    }
    for j in 0..grid.cols {
        mx = mx.max(solve(&grid, (0, j), (1, 0)));
        mx = mx.max(solve(&grid, (grid.rows - 1, j), (-1, 0)));
    }

    mx
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    common::timed(&input, part2, false);
}
