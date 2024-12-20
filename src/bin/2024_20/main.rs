use std::collections::VecDeque;

use aoc::{common, grid::Grid};

fn shortest_path(map: &Grid<char>) -> Vec<(usize, usize)> {
    let ans = Vec::new();
    let pos = map.positions('S')[0];
    let mut q = VecDeque::new();
    q.push_back(vec![pos]);
    let mut visited = Vec::new();

    while let Some(tp) = q.pop_front() {
        let lst = tp.last().unwrap();
        if visited.contains(lst) {
            continue;
        }
        visited.push(*lst);

        if map.get(lst) == 'E' {
            return tp;
        }

        let nx: Vec<(usize, usize)> = map
            .adjacent_4(lst)
            .into_iter()
            .filter(|x| map.get(x) == '.' || map.get(x) == 'E')
            .collect();

        for n in nx.iter() {
            let mut tp_tmp = tp.clone();
            tp_tmp.push(*n);
            q.push_back(tp_tmp);
        }
    }
    ans
}

fn solve<const PICOSECONDS: usize, const CHEAT_PICOSECONDS: usize>(input: &str) -> usize {
    let map = Grid::from_str(input, |c| c);
    let path = shortest_path(&map);
    let path_d: Vec<((usize, usize), usize)> =
        path.iter().enumerate().map(|(i, v)| (*v, i)).collect();

    let mut res: [usize; 10000] = [0; 10000];

    for (v1, l1) in path_d.iter() {
        for (v2, l2) in path_d.iter().skip(PICOSECONDS) {
            let dist = map.l1_distance(v1, v2);
            if dist <= CHEAT_PICOSECONDS && *l2 > *l1 + dist {
                let l = *l2 - *l1 - dist;
                res[l] += 1;
            }
        }
    }

    res.iter()
        .enumerate()
        .filter(|(k, _)| *k >= PICOSECONDS)
        .map(|(_, v)| v)
        .sum()
}

fn main() {
    let input = common::get_input();
    println!("{input:?}");
    common::timed(&input, solve::<100, 2>, true);
    common::timed(&input, solve::<100, 20>, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "###############\n#...#...#.....#\n#.#.#.#.#.###.#\n#S#...#.#.#...#\n#######.#.#.###\n#######.#.#...#\n#######.#.###.#\n###..E#...#...#\n###.#######.###\n#...###...#...#\n#.#####.#.###.#\n#.#...#.#.#...#\n#.#.#.#.#.#.###\n#...#...#...###\n###############";
        assert_eq!(solve::<2, 2>(sample_input), 44);
        assert_eq!(solve::<2, 20>(sample_input), 3081);
    }
}
