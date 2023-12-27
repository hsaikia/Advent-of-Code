use aoc::{
    common,
    grid::{CellIndex, Grid},
};

use std::collections::{HashMap, VecDeque};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref ELEV: HashMap<char, usize> = {
        let mut mp: HashMap<char, usize> = HashMap::new();
        for (idx, ch) in ('a'..='z').enumerate() {
            mp.insert(ch, idx);
        }
        mp.insert('S', 0);
        mp.insert('E', 25);
        mp
    };
}

fn shortest_from(grid: &Grid<usize>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut bfs_queue: VecDeque<(CellIndex, usize)> = VecDeque::new();
    bfs_queue.push_back((start, 0));

    let mut visited = Grid::<bool>::new(grid.rows, grid.cols, false);

    while !bfs_queue.is_empty() {
        let (elem, l) = bfs_queue.pop_front().unwrap();
        if elem == end {
            return Some(l);
        }

        if visited.get(&elem) {
            continue;
        }

        visited.set(&elem, true);
        let val = grid.get(&elem);
        let adjacent = grid.adjacent_4(&elem);
        let next_cells = adjacent
            .iter()
            .filter(|&e| grid.get(e) <= val + 1)
            .collect::<Vec<&(usize, usize)>>();

        for cell in next_cells {
            bfs_queue.push_back((*cell, l + 1));
        }
    }

    None
}

fn part1(grid: &Grid<usize>, start: (usize, usize), end: (usize, usize)) -> usize {
    shortest_from(grid, start, end).unwrap()
}

fn part2(grid: &Grid<usize>, end: (usize, usize)) -> usize {
    let mut best = usize::MAX;
    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if grid.get(&(i, j)) == 0 {
                let steps = shortest_from(grid, (i, j), end);
                if steps.is_some() {
                    best = best.min(steps.unwrap());
                }
            }
        }
    }
    best
}

fn get_grid_and_solve<const PART1: bool>(input: &str) -> usize {
    let grid = Grid::from_str(input, |c| *ELEV.get(&c).unwrap());
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for (i, line) in input.lines().enumerate() {
        let si = line.chars().position(|c| c == 'S');
        if si.is_some() {
            start = (i, si.unwrap());
        }
        let ei = line.chars().position(|c| c == 'E');
        if ei.is_some() {
            end = (i, ei.unwrap());
        }
    }

    if PART1 {
        return part1(&grid, start, end);
    }

    part2(&grid, end)
}

fn main() {
    let input = common::get_input();
    common::timed(&input, get_grid_and_solve::<true>, true);
    common::timed(&input, get_grid_and_solve::<false>, false);
}
