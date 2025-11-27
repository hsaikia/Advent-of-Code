use std::collections::VecDeque;

use aoc::{common, grid::Grid, io};

fn shortest_path(map: &Grid<char>) -> Option<usize> {
    let mut ans = None;
    let pos = (0, 0);
    let mut q = VecDeque::new();
    q.push_back((pos, 0));
    let mut visited = Vec::new();

    while let Some((tp, l)) = q.pop_front() {
        if tp == (map.rows - 1, map.cols - 1) {
            ans = Some(l);
            break;
        }

        if visited.contains(&tp) {
            continue;
        }

        visited.push(tp);

        let nx: Vec<(usize, usize)> = map
            .adjacent_4(&tp)
            .into_iter()
            .filter(|x| map.get(x) == '.')
            .collect();

        for n in &nx {
            q.push_back((*n, l + 1));
        }
    }
    ans
}

fn solve<const PART: usize, const SIZE: usize, const PART_1_BYTES: usize>(input: &str) -> String {
    let mut byte_coord = Vec::new();
    for coord in input.lines() {
        let xy: Vec<usize> = io::tokenize_nums(coord, ",");
        byte_coord.push((xy[0], xy[1]));
    }

    let mut map: Grid<char> = Grid::new(SIZE, SIZE, '.');
    if PART == 1 {
        for coord in byte_coord.iter().take(PART_1_BYTES) {
            map.set(coord, '#');
        }
        return std::format!("{}", shortest_path(&map).unwrap());
    }
    for bytes in PART_1_BYTES.. {
        for coord in byte_coord.iter().take(bytes) {
            map.set(coord, '#');
        }

        if shortest_path(&map).is_none() {
            return std::format!("{},{}", byte_coord[bytes - 1].0, byte_coord[bytes - 1].1);
        }
    }

    String::new()
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, solve::<1, 71, 1024>, true);
        common::timed(&input, solve::<2, 71, 1024>, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0";
        assert_eq!(solve::<1, 7, 12>(sample_input), "22");
        assert_eq!(solve::<2, 7, 12>(sample_input), "6,1");
    }
}
