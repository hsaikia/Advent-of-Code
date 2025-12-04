use aoc::{common, grid::Grid};

fn remove_if_possible(grid: &mut Grid<char>) -> Option<usize> {
    let to_be_removed: Vec<(usize, usize)> = grid
        .positions(&'@')
        .iter()
        .map(|idx| (idx, grid.adjacent_8(idx)))
        .filter(|(_idx, neighbors)| neighbors.iter().filter(|x| grid.get(x) == '@').count() < 4)
        .map(|(idx, _neighbors)| *idx)
        .collect();

    if to_be_removed.is_empty() {
        return None;
    }

    grid.set_all(&to_be_removed, '.');

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
