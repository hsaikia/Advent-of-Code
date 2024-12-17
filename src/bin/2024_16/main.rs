use std::collections::{HashMap, HashSet, VecDeque};

use aoc::{common, grid::Grid};
use num::abs;

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn solve<const PART: usize>(input: &str) -> usize {
    let map = Grid::from_str(input, |c| c);
    let spos = map.positions('S')[0];
    let epos = map.positions('E')[0];

    let mut q = VecDeque::new();
    q.push_back((vec![spos], 3, 0));

    let mut lowest_cost = usize::MAX;
    let mut visited: HashMap<((usize, usize), usize), usize> = HashMap::new();
    let mut best_seats: HashSet<(usize, usize)> = HashSet::new();

    while let Some((tp, di, s)) = q.pop_front() {
        if *tp.last().unwrap() == epos {
            if lowest_cost >= s {
                if PART == 2 {
                    if lowest_cost > s {
                        best_seats.clear();
                    }

                    for x in tp.iter() {
                        best_seats.insert(*x);
                    }
                }
                lowest_cost = s;
            }
            continue;
        }

        if let Some(v) = visited.get_mut(&(*tp.last().unwrap(), di)) {
            if *v < s {
                continue;
            } else {
                *v = s;
            }
        } else {
            visited.insert((*tp.last().unwrap(), di), s);
        }

        for (i, d) in DIRS.iter().enumerate() {
            let ss = if i == di {
                1
            } else if abs(i as i32 - di as i32) == 2 {
                2001
            } else {
                1001
            };
            let nx: Vec<(usize, usize)> = map
                .adjacent_in_dir(tp.last().unwrap(), &[*d])
                .into_iter()
                .filter(|x| map.get(x) == '.' || map.get(x) == 'E')
                .collect();
            if nx.is_empty() {
                continue;
            }

            let mut tmp = tp.clone();
            tmp.push(nx[0]);
            q.push_back((tmp, i, ss + s));
        }
    }

    if PART == 1 {
        lowest_cost
    } else {
        best_seats.len()
    }
}

fn main() {
    let input = common::get_input();
    println!("{input:?}");
    common::timed(&input, solve::<1>, true);
    common::timed(&input, solve::<2>, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "###############\n#.......#....E#\n#.#.###.#.###.#\n#.....#.#...#.#\n#.###.#####.#.#\n#.#.#.......#.#\n#.#.#####.###.#\n#...........#.#\n###.#.#####.#.#\n#...#.....#.#.#\n#.#.#.###.#.#.#\n#.....#...#.#.#\n#.###.#.#.#.#.#\n#S..#.....#...#\n###############";
        assert_eq!(solve::<1>(sample_input), 7036);
        assert_eq!(solve::<2>(sample_input), 45);
        let sample_input = "#################\n#...#...#...#..E#\n#.#.#.#.#.#.#.#.#\n#.#.#.#...#...#.#\n#.#.#.#.###.#.#.#\n#...#.#.#.....#.#\n#.#.#.#.#.#####.#\n#.#...#.#.#.....#\n#.#.#####.#.###.#\n#.#.#.......#...#\n#.#.###.#####.###\n#.#.#...#.....#.#\n#.#.#.#####.###.#\n#.#.#.........#.#\n#.#.#.#########.#\n#S#.............#\n#################";
        assert_eq!(solve::<1>(sample_input), 11048);
        assert_eq!(solve::<2>(sample_input), 64);
    }
}
