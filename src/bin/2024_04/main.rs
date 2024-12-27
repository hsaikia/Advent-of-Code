use aoc::{common, grid::Grid};

#[allow(clippy::similar_names)]
fn solve2(input: &str) -> usize {
    const DIRS: [(i32, i32); 4] = [(1, 1), (-1, 1), (-1, -1), (1, -1)];
    const ADJ_DIRS: [(usize, usize); 4] = [(0, 1), (1, 2), (2, 3), (3, 0)];
    let mut ans = 0;
    let grid = Grid::from_str(input, |c| c);

    let a_positions = grid.positions(&'A');

    for a_pos in &a_positions {
        for (i1, i2) in &ADJ_DIRS {
            let m_pos = [DIRS[*i1], DIRS[*i2]];
            let s_pos = [DIRS[(*i1 + 2) % 4], DIRS[(*i2 + 2) % 4]];

            let num_desired_m_pos = grid
                .adjacent_in_dir(a_pos, &m_pos)
                .iter()
                .filter(|x| grid.get(x) == 'M')
                .count();
            let num_desired_s_pos = grid
                .adjacent_in_dir(a_pos, &s_pos)
                .iter()
                .filter(|x| grid.get(x) == 'S')
                .count();

            if num_desired_m_pos == 2 && num_desired_s_pos == 2 {
                ans += 1;
            }
        }
    }

    ans
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
    let mut ans = 0;
    let grid = Grid::from_str(input, |c| c);

    for x_pos in &grid.positions(&'X') {
        for dir in DIRS {
            let mut pos = vec![*x_pos];
            let mut count = 0;
            for ch in ['M', 'A', 'S'] {
                pos = grid.adjacent_in_dir(&pos[0], &[dir]);
                if pos.is_empty() || grid.get(&pos[0]) != ch {
                    break;
                }
                count += 1;
            }

            if count == 3 {
                ans += 1;
            }
        }
    }

    ans
}

fn main() {
    let input = common::get_input();
    //println!("{input:?}");
    common::timed(&input, solve1, true);
    common::timed(&input, solve2, false);
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
