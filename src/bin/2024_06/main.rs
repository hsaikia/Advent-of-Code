use aoc::{common, grid::Grid};
use itertools::Itertools;

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn loop_positions(grid: &Grid<char>) -> (Vec<(usize, usize)>, bool) {
    let mut pos = grid.positions('^')[0];
    let mut dir = 0;
    let mut visited: Vec<(usize, usize, usize)> = Vec::new();
    let mut looping = false;

    loop {
        if !visited.contains(&(pos.0, pos.1, dir)) {
            visited.push((pos.0, pos.1, dir));
        } else {
            looping = true;
            break;
        }

        let next_positions = grid.adjacent_in_dir(&pos, &[DIRS[dir]]);
        if next_positions.is_empty() {
            break;
        } else if grid.get(&next_positions[0]) == '#' {
            dir = (dir + 1) % 4;
        } else {
            pos = next_positions[0];
        }
    }

    let unique_visited: Vec<(usize, usize)> = visited
        .into_iter()
        .map(|(x, y, _)| (x, y))
        .sorted()
        .dedup()
        .collect();
    (unique_visited, looping)
}

fn solve(input: &str) -> (usize, usize) {
    let grid = Grid::from_str(input, |c| c);
    let (positions, _) = loop_positions(&grid);
    let mut ans2 = 0;
    for pos in positions.iter() {
        if grid.get(pos) == '^' {
            continue;
        }
        let mut grid2 = grid.clone();
        grid2.set(pos, '#');
        let ret = loop_positions(&grid2);
        if ret.1 {
            ans2 += 1;
        }
    }
    (positions.len(), ans2)
}

fn main() {
    let input = common::get_input();
    //println!("{input:?}");
    common::timed(&input, solve, true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input =
            "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        assert_eq!(solve(sample_input), (41, 6));
    }
}
