use aoc::{common, grid::Grid};

fn part1(grid: &Grid<u32>) -> usize {
    // Check visibility for each internal tree
    let mut visible = Grid::<bool>::new(grid.rows, grid.cols, false);

    for i in 0..grid.rows {
        let mut best: [i32; 2] = [-1, -1];

        // left to right sweep
        for j in 0..grid.cols {
            let val = grid.get(&(i, j)) as i32;
            if val > best[0] {
                best[0] = val;
                visible.set(&(i, j), true);
            }
        }

        // right to left sweep
        for j in (0..grid.cols).rev() {
            let val = grid.get(&(i, j)) as i32;
            if val > best[1] {
                best[1] = val;
                visible.set(&(i, j), true);
            }
        }
    }

    for j in 0..grid.cols {
        let mut best: [i32; 2] = [-1, -1];

        // top to bottom sweep
        for i in 0..grid.rows {
            let val = grid.get(&(i, j)) as i32;
            if val > best[0] {
                best[0] = val;
                visible.set(&(i, j), true);
            }
        }

        // bottom to top sweep
        for i in (0..grid.rows).rev() {
            let val = grid.get(&(i, j)) as i32;
            if val > best[1] {
                best[1] = val;
                visible.set(&(i, j), true);
            }
        }
    }

    visible
        .values
        .iter()
        .map(|row| row.iter().filter(|&x| *x).count())
        .sum::<usize>()
}

fn part2(grid: &Grid<u32>) -> i32 {
    let mut ans = 0;
    for i in 0..grid.rows {
        for j in 0..grid.cols {
            let mut scores = [0; 4];
            let sweeps = grid.sweep_4(i, j);
            let h = grid.get(&(i, j));

            for (i, sweep) in sweeps.iter().enumerate() {
                for nxy in sweep {
                    let h1 = grid.get(nxy);
                    scores[i] += 1;

                    if h1 >= h {
                        break;
                    }
                }
            }

            ans = ans.max(scores.iter().product::<i32>());
        }
    }
    ans
}

fn get_grid_and_solve<const PART1: bool>(input: &str) -> i32 {
    let grid = Grid::from_str(input, |c| c.to_digit(10).unwrap());
    if PART1 {
        return part1(&grid) as i32;
    }
    part2(&grid)
}

fn main() {
    let input = common::get_input();
    common::timed(&input, get_grid_and_solve::<true>, true);
    common::timed(&input, get_grid_and_solve::<false>, false);
}
