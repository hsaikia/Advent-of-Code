use aoc::common;
use aoc::grid::Grid;
use itertools::Itertools;

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
fn solve<const PART: i32>(input: &str) -> usize {
    let map = Grid::from_str(input, |c| c);
    let mut locations = Vec::new();
    let symbols = ('a'..='z').chain('A'..='Z').chain('0'..='9');

    for sym in symbols {
        let pos = map.positions(&sym);
        if pos.len() < 2 {
            continue;
        }

        if PART == 2 {
            for p in &pos {
                locations.push(*p);
            }
        }

        for p1 in &pos {
            for p2 in &pos {
                if p1 == p2 {
                    continue;
                }
                let dx = p1.0 as i32 - p2.0 as i32;
                let dy = p1.1 as i32 - p2.1 as i32;

                let mut curr_pos = *p1;
                while let Some(antinode_pos) = map.cell_in_direction(&curr_pos, &(dx, dy)) {
                    locations.push(antinode_pos);
                    curr_pos = antinode_pos;

                    if PART == 1 {
                        break;
                    }
                }
            }
        }
    }

    locations.iter().sorted().dedup().count()
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, solve::<1>, true);
        common::timed(&input, solve::<2>, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............";
        assert_eq!(solve::<1>(sample_input), 14);
        assert_eq!(solve::<2>(sample_input), 34);
    }
}
