use aoc::{common, grid::Grid};
use itertools::Itertools;

fn solve2(input: &str) -> usize {
    const OPP_DIRS: [[(i32, i32); 2]; 2] = [[(1, 1), (-1, -1)], [(-1, 1), (1, -1)]];
    let grid = Grid::from_str(input, |c| c);

    grid.positions(&'A')
        .iter()
        .filter(|pos| {
            for dirs in OPP_DIRS {
                for ch in ['M', 'S'] {
                    if grid
                        .adjacent_in_dirs(pos, &dirs)
                        .iter()
                        .filter(|x| grid.get(x) == ch)
                        .count()
                        != 1
                    {
                        return false;
                    }
                }
            }
            true
        })
        .count()
}

fn solve1(input: &str) -> usize {
    const DIRS: [(i32, i32); 8] = [
        (-1, 0),
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 1),
        (1, -1),
        (1, 1),
        (-1, -1),
    ];

    let grid = Grid::from_str(input, |c| c);
    grid.positions(&'X')
        .iter()
        .cartesian_product(DIRS.iter())
        .map(|(x_pos, dir)| {
            let mut cnt = 0;
            let mut curr_pos = *x_pos;
            for ch in ['M', 'A', 'S'] {
                if let Some(pos) = grid.cell_in_direction(&curr_pos, dir) {
                    if grid.get(&pos) != ch {
                        break;
                    }
                    cnt += 1;
                    curr_pos = pos;
                } else {
                    break;
                }
            }
            cnt
        })
        .filter(|cnt| *cnt == 3)
        .count()
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
        let sample_input =
            "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
        assert_eq!(solve1(sample_input), 18);
        assert_eq!(solve2(sample_input), 9);
    }
}
