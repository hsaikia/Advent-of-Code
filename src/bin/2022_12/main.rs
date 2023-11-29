use aoc::grid::Grid;
use aoc::io;

use std::collections::{HashMap, VecDeque};

#[macro_use]
extern crate lazy_static;

const FILES: [&str; 2] = [
    "./src/bin/2022_12/sample_input.txt",
    "./src/bin/2022_12/input.txt",
];

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
    let mut bfs_queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
    bfs_queue.push_back((start.0, start.1, 0));

    let mut visited = Grid::<bool>::new(grid.rows, grid.cols, false);

    while !bfs_queue.is_empty() {
        let elem = bfs_queue.pop_front().unwrap();
        if elem.0 == end.0 && elem.1 == end.1 {
            return Some(elem.2);
        }

        if visited.get(elem.0, elem.1).unwrap() {
            continue;
        }

        visited.set(elem.0, elem.1, true);
        let val = grid.get(elem.0, elem.1).unwrap();
        let adjacent = grid.adjacent_4(elem.0, elem.1);
        let next_cells = adjacent
            .iter()
            .filter(|&e| grid.get(e.0, e.1).unwrap() <= val + 1)
            .collect::<Vec<&(usize, usize)>>();

        for cell in next_cells {
            bfs_queue.push_back((cell.0, cell.1, elem.2 + 1));
        }
    }

    None
}

fn part1(grid: &Grid<usize>, start: (usize, usize), end: (usize, usize)) {
    println!(
        "Part 1 : Fewest possible steps {}",
        shortest_from(grid, start, end).unwrap()
    );
}

fn part2(grid: &Grid<usize>, end: (usize, usize)) {
    let mut best = usize::MAX;
    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if grid.get(i, j).unwrap() == 0 {
                let steps = shortest_from(grid, (i, j), end);
                if steps.is_some() {
                    best = best.min(steps.unwrap());
                }
            }
        }
    }
    println!("Part 2 : Fewest possible steps {}", best);
}

fn main() {
    for filename in FILES {
        println!("Input file {filename}");
        if let Ok(lines) = io::read_lines(filename) {
            let input_lines = lines.flatten().collect::<Vec<String>>();

            let mut grid = Grid::<usize>::new(input_lines.len(), input_lines[0].len(), 0);
            let mut start: (usize, usize) = (0, 0);
            let mut end: (usize, usize) = (0, 0);

            for (i, line) in input_lines.iter().enumerate() {
                let si = line.chars().position(|c| c == 'S');
                if si.is_some() {
                    start = (i, si.unwrap());
                }
                let ei = line.chars().position(|c| c == 'E');
                if ei.is_some() {
                    end = (i, ei.unwrap());
                }
                grid.set_row(
                    i,
                    line.chars()
                        .map(|c| *ELEV.get(&c).unwrap())
                        .collect::<Vec<usize>>(),
                );
            }

            part1(&grid, start, end);
            part2(&grid, end);
        }
    }
}
