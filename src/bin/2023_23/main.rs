use std::collections::{HashMap, HashSet, VecDeque};

use aoc::{
    common::{self, HashMapVector},
    grid::{CardinalDirection, CellIndex, Grid},
};
use itertools::iproduct;

#[derive(Clone, Debug, Default, PartialEq, Hash)]
enum Cell {
    Forest,
    #[default]
    Path,
    Slope(CardinalDirection),
}

fn get_neighbors(g: &Grid<Cell>, lst: &CellIndex, slopes: bool) -> Vec<CellIndex> {
    match g.get(lst) {
        Cell::Path => g
            .adjacent_4(lst.0, lst.1)
            .into_iter()
            .filter(|idx| g.get(idx) != Cell::Forest)
            .collect::<Vec<_>>(),
        Cell::Slope(cd) => {
            if slopes {
                let dir = cd.to_dir();
                vec![g.cell_in_direction(lst.0, lst.1, dir.0, dir.1).unwrap()]
            } else {
                g.adjacent_4(lst.0, lst.1)
                    .into_iter()
                    .filter(|idx| g.get(idx) != Cell::Forest)
                    .collect::<Vec<_>>()
            }
        }
        _ => vec![],
    }
}

fn junctions(g: &Grid<Cell>, slopes: bool) -> Vec<CellIndex> {
    let mut ret = Vec::new();
    for (i, j) in iproduct!(0..g.rows, 0..g.cols) {
        let ns = get_neighbors(g, &(i, j), slopes);
        if ns.len() > 2 {
            ret.push((i, j));
        }
    }
    ret
}

// Find direct paths to other junctions
fn bfs(
    g: &Grid<Cell>,
    start: CellIndex,
    dst: CellIndex,
    junctions: &[CellIndex],
    slopes: bool,
) -> Option<usize> {
    let mut q: VecDeque<(CellIndex, usize)> = VecDeque::new();
    let mut seen: HashSet<CellIndex> = HashSet::new();
    q.push_back((start, 0));

    while !q.is_empty() {
        let (idx, l) = q.pop_front().unwrap();

        if seen.contains(&idx) {
            continue;
        }

        seen.insert(idx);

        if idx == dst {
            return Some(l);
        }

        // If wrong junction found, continue
        if junctions.contains(&idx) && idx != start {
            continue;
        }

        let ncs = get_neighbors(g, &idx, slopes);
        for nc in ncs {
            q.push_back((nc, l + 1));
        }
    }
    None
}

fn solve(map: &HashMap<CellIndex, Vec<(CellIndex, usize)>>, s: CellIndex, e: CellIndex) -> usize {
    let mut paths: Vec<(HashSet<CellIndex>, CellIndex, usize)> = Vec::new();
    let mut path = HashSet::new();
    path.insert(s);
    paths.push((path, s, 0));

    let mut ans = 0;

    loop {
        //println!("Path lens {:?}", paths.iter().map(|x| x.len()).collect::<Vec<_>>());
        println!("Num Paths {}", paths.len());
        let mut new_paths = Vec::new();
        for (p, lst, l) in &paths {
            if *lst == e {
                ans = ans.max(*l);
                continue;
            }

            if let Some(ncs) = map.get(lst) {
                for (nc, ll) in ncs {
                    if p.contains(nc) {
                        continue;
                    }
                    let mut p_tmp = p.clone();
                    p_tmp.insert(*nc);
                    new_paths.push((p_tmp, *nc, l + ll));
                }
            }
        }
        if new_paths == paths {
            break;
        }
        paths = new_paths;
    }
    ans
}

fn part_solve<const PART1: bool>(input: &str) -> usize {
    let g = Grid::from_str(input, |ch| match ch {
        '#' => Cell::Forest,
        '.' => Cell::Path,
        '>' => Cell::Slope(CardinalDirection::East),
        '<' => Cell::Slope(CardinalDirection::West),
        '^' => Cell::Slope(CardinalDirection::North),
        'v' => Cell::Slope(CardinalDirection::South),
        _ => panic!("Bad cell"),
    });

    let start_col = g.values[0]
        .iter()
        .enumerate()
        .filter_map(|(i, c)| if *c == Cell::Path { Some(i) } else { None })
        .collect::<Vec<_>>()[0];
    let end_col = g.values[g.rows - 1]
        .iter()
        .enumerate()
        .filter_map(|(i, c)| if *c == Cell::Path { Some(i) } else { None })
        .collect::<Vec<_>>()[0];

    let start = (0, start_col);
    let end = (g.rows - 1, end_col);

    let mut js = junctions(&g, PART1);
    js.push(start);
    js.push(end);
    let mut map: HashMap<CellIndex, Vec<(CellIndex, usize)>> = HashMap::new();

    for i in 0..js.len() {
        for j in 0..js.len() {
            if i == j {
                continue;
            }
            let opt_dist = bfs(&g, js[i], js[j], &js, PART1);
            if let Some(dist) = opt_dist {
                map.add_to_vector_hashmap(&js[i], (js[j], dist));
            }
        }
    }

    solve(&map, start, end)
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part_solve::<true>, true);
    common::timed(&input, part_solve::<false>, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input = "#.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#";
        assert_eq!(part_solve::<true>(sample_input), 94);
        assert_eq!(part_solve::<false>(sample_input), 154);
    }
}
