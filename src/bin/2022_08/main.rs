use aoc::{common, grid::Grid};
use itertools::iproduct;

#[allow(clippy::cast_possible_wrap)]
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

    for idx in iproduct!(0..grid.rows, 0..grid.cols) {
        let mut scores = [0; 4];
        let sweeps = grid.sweep_4(&idx);
        let h = grid.get(&idx);

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

    ans
}

fn get_grid_and_solve<const PART1: bool>(input: &str) -> i32 {
    let grid = Grid::from_str(input, |c| c.to_digit(10).unwrap());
    if PART1 {
        return i32::try_from(part1(&grid)).unwrap();
    }
    part2(&grid)
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, get_grid_and_solve::<true>, true);
        common::timed(&input, get_grid_and_solve::<false>, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input = "30373\n25512\n65332\n33549\n35390";
        assert_eq!(get_grid_and_solve::<true>(sample_input), 21);
        assert_eq!(get_grid_and_solve::<false>(sample_input), 8);
    }
}
