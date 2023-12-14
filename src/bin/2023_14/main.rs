use aoc::grid::Grid;
use std::{collections::HashMap, time::Instant};

const INPUT: [(&str, &str); 2] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

fn simulate(grid: &mut Grid<char>, cycles: usize) {
    if cycles == 0 {
        return;
    }

    let mut grid_new = Grid::new(grid.rows, grid.cols, '.');

    // North
    for c in 0..grid.cols {
        let mut idx = 0;
        for r in 0..grid.rows {
            if grid.get(r, c).unwrap() == 'O' {
                grid_new.set(idx, c, 'O');
                idx += 1;
            } else if grid.get(r, c).unwrap() == '#' {
                grid_new.set(r, c, '#');
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
            if grid.get(r, c).unwrap() == 'O' {
                grid_new.set(r, idx, 'O');
                idx += 1;
            } else if grid.get(r, c).unwrap() == '#' {
                grid_new.set(r, c, '#');
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
            if grid.get(r, c).unwrap() == 'O' {
                grid_new.set(idx, c, 'O');
                idx -= 1;
            } else if grid.get(r, c).unwrap() == '#' {
                grid_new.set(r, c, '#');
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
            if grid.get(r, c).unwrap() == 'O' {
                grid_new.set(r, idx, 'O');
                idx -= 1;
            } else if grid.get(r, c).unwrap() == '#' {
                grid_new.set(r, c, '#');
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
            if grid.get(r, c).unwrap() == 'O' {
                ans += grid.rows - r;
            }
        }
    }
    ans
}

fn part1(grid: &Grid<char>) {
    let mut ans: usize = 0;
    for c in 0..grid.cols {
        let mut num = grid.rows;
        for r in 0..grid.rows {
            if grid.get(r, c).unwrap() == 'O' {
                ans += num;
                num -= 1;
            } else if grid.get(r, c).unwrap() == '#' {
                num = grid.rows - r - 1;
            }
        }
    }
    println!("North load after moving North {}", ans);
}

fn part2(grid: &Grid<char>) {
    let (period, offset_front) = period_and_offset(grid);
    let offset_back = (1000000000 - offset_front) % period;
    let mut grid = grid.clone();
    simulate(&mut grid, period + offset_front + offset_back);
    println!(
        "North Load after simulating 1000000000 times {}",
        north_load(&grid)
    );
}

fn main() {
    for (file, input) in INPUT {
        println!("{}", file);
        let grid = Grid::from_str(input, |c| c);

        let start = Instant::now();
        part1(&grid);
        let duration = start.elapsed();
        println!("Time elapsed in Part 1 is: {:?}", duration);

        let start = Instant::now();
        part2(&grid);
        let duration = start.elapsed();
        println!("Time elapsed in Part 2 is: {:?}", duration);
    }
}
