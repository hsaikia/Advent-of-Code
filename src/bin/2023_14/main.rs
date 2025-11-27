use aoc::{common, grid::Grid};
use std::collections::HashMap;

fn simulate(grid: &mut Grid<char>, cycles: usize) {
    if cycles == 0 {
        return;
    }

    let mut grid_new = Grid::new(grid.rows, grid.cols, '.');

    // North
    for c in 0..grid.cols {
        let mut idx = 0;
        for r in 0..grid.rows {
            let cidx = (r, c);
            if grid.get(&cidx) == 'O' {
                grid_new.set(&(idx, c), 'O');
                idx += 1;
            } else if grid.get(&cidx) == '#' {
                grid_new.set(&cidx, '#');
                idx = r + 1;
            }
        }
    }

    *grid = grid_new;
    let mut grid_new = Grid::new(grid.rows, grid.cols, '.');

    // West
    for r in 0..grid.rows {
        let mut idx = 0;
        for c in 0..grid.cols {
            let cidx = (r, c);
            if grid.get(&cidx) == 'O' {
                grid_new.set(&(r, idx), 'O');
                idx += 1;
            } else if grid.get(&cidx) == '#' {
                grid_new.set(&cidx, '#');
                idx = c + 1;
            }
        }
    }

    *grid = grid_new;
    let mut grid_new = Grid::new(grid.rows, grid.cols, '.');

    // South
    for c in 0..grid.cols {
        let mut idx = grid.rows - 1;
        for r in (0..grid.rows).rev() {
            let cidx = (r, c);
            if grid.get(&cidx) == 'O' {
                grid_new.set(&(idx, c), 'O');
                idx -= 1;
            } else if grid.get(&cidx) == '#' {
                grid_new.set(&cidx, '#');
                idx = r - 1;
            }
        }
    }

    *grid = grid_new;
    let mut grid_new = Grid::new(grid.rows, grid.cols, '.');

    // East
    for r in 0..grid.rows {
        let mut idx = grid.cols - 1;
        for c in (0..grid.cols).rev() {
            if grid.get(&(r, c)) == 'O' {
                grid_new.set(&(r, idx), 'O');
                idx -= 1;
            } else if grid.get(&(r, c)) == '#' {
                grid_new.set(&(r, c), '#');
                idx = c - 1;
            }
        }
    }

    *grid = grid_new;

    simulate(grid, cycles - 1);
}

fn period_and_offset(grid_original: &Grid<char>) -> (usize, usize) {
    let mut map: HashMap<u64, usize> = HashMap::new();
    let mut cycles = 0;
    let mut grid = grid_original.clone();

    loop {
        map.insert(grid.get_hash(), cycles);
        let mut grid_new = grid.clone();
        simulate(&mut grid_new, 1);
        cycles += 1;

        let hsh = grid_new.get_hash();
        if map.contains_key(&hsh) {
            let idx = map.get(&hsh).unwrap();
            return (cycles - idx, *idx);
        }

        grid = grid_new;
    }
}

fn north_load(grid: &Grid<char>) -> usize {
    let mut ans: usize = 0;
    for c in 0..grid.cols {
        for r in 0..grid.rows {
            if grid.get(&(r, c)) == 'O' {
                ans += grid.rows - r;
            }
        }
    }
    ans
}

fn part1(grid: &Grid<char>) -> usize {
    let mut ans: usize = 0;
    for c in 0..grid.cols {
        let mut num = grid.rows;
        for r in 0..grid.rows {
            if grid.get(&(r, c)) == 'O' {
                ans += num;
                num -= 1;
            } else if grid.get(&(r, c)) == '#' {
                num = grid.rows - r - 1;
            }
        }
    }
    ans
}

fn part2(grid: &Grid<char>) -> usize {
    let (period, offset_front) = period_and_offset(grid);
    let offset_back = (1_000_000_000 - offset_front) % period;
    let mut grid = grid.clone();
    simulate(&mut grid, period + offset_front + offset_back);
    north_load(&grid)
}

fn process_and_solve<const PART1: bool>(input: &str) -> usize {
    let grid = Grid::from_str(input, |c| c);
    if PART1 {
        return part1(&grid);
    }

    part2(&grid)
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, process_and_solve::<true>, true);
        common::timed(&input, process_and_solve::<false>, false);
    }
}
