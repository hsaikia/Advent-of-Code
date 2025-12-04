use aoc::{common, grid::Grid};

fn remove_if_possible(grid: &mut Grid<char>) -> Option<usize> {
    let positions = grid.positions(&'@');
    let mut to_be_removed = Vec::new();
    for pos in positions.iter() {
        let nbs = grid.adjacent_8(pos);
        //dbg!(pos, &nbs);
        if nbs.iter().filter(|x| grid.get(x) == '@').count() < 4 {
            to_be_removed.push(pos);
        }
    }

    if to_be_removed.is_empty() {
        return None;
    }

    for idx in to_be_removed.iter() {
        grid.set(idx, '.');
    }

    Some(to_be_removed.len())
}

fn solve<const PART: usize>(input: &str) -> usize {
    let mut ans = 0;
    let mut grid = Grid::from_str(input, |c| c);
    if PART == 1 {
        if let Some(x) = remove_if_possible(&mut grid) {
            ans = x;
        }
    } else {
        while let Some(x) = remove_if_possible(&mut grid) {
            ans += x;
        }
    }
    ans
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
        let sample_input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        assert_eq!(solve::<1>(sample_input), 13);
        assert_eq!(solve::<2>(sample_input), 43);
    }
}
