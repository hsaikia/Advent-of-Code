use aoc::{common, grid::Grid};

fn solve<const GALAXY_EXPANSION: usize>(input: &str) -> usize {
    let mut ans: usize = 0;

    let grid = Grid::<char>::from_str(input, |c| c);

    let empty_rows = (0..grid.rows)
        .filter(|r| grid.find_in_row(*r, '#').is_empty())
        .collect::<Vec<_>>();
    let empty_cols = (0..grid.cols)
        .filter(|c| grid.find_in_col(*c, '#').is_empty())
        .collect::<Vec<_>>();

    let star_positions = grid.positions('#');

    for (i, (r1, c1)) in star_positions.iter().enumerate() {
        for (r2, c2) in star_positions.iter().skip(i + 1) {
            let (r1, r2) = common::minmax(r1, r2);
            let (c1, c2) = common::minmax(c1, c2);
            let mut d = r2 + c2 - r1 - c1;

            d += (r1 + 1..r2).filter(|r| empty_rows.contains(r)).count() * (GALAXY_EXPANSION - 1);
            d += (c1 + 1..c2).filter(|c| empty_cols.contains(c)).count() * (GALAXY_EXPANSION - 1);

            ans += d;
        }
    }

    ans
}

fn main() {
    let input = common::get_input();
    common::timed(&input, solve::<2>, true);
    common::timed(&input, solve::<1000000>, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input = "...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....";
        assert_eq!(solve::<2>(sample_input), 374);
        assert_eq!(solve::<10>(sample_input), 1030);
        assert_eq!(solve::<100>(sample_input), 8410);
        assert_eq!(solve::<1000000>(sample_input), 82000210);
    }
}
