use std::collections::VecDeque;

use aoc::{
    common,
    grid::{CellDir, CellIndex, Grid},
};
use itertools::iproduct;

#[derive(Debug)]
enum Dir {
    PX,
    NX,
    PY,
    NY,
}

#[derive(Hash, Clone, Default, Debug, PartialEq)]
enum Cluster {
    #[default]
    Empty,
    Path,
    Side1,
    Side2,
}

fn lr_classification(ch: char, dir: &Dir) -> (Vec<CellDir>, Vec<CellDir>) {
    let mut l = Vec::new();
    let mut r = Vec::new();

    match ch {
        '|' => match dir {
            Dir::PX => {
                l.extend(vec![(-1, 1), (0, 1), (1, 1)]);
                r.extend(vec![(-1, -1), (0, -1), (1, -1)]);
            }
            Dir::NX => {
                r.extend(vec![(-1, 1), (0, 1), (1, 1)]);
                l.extend(vec![(-1, -1), (0, -1), (1, -1)]);
            }
            _ => panic!("Wrong combo!"),
        },
        '-' => match dir {
            Dir::NY => {
                l.extend(vec![(1, -1), (1, 0), (1, 1)]);
                r.extend(vec![(-1, -1), (-1, 0), (-1, 1)]);
            }
            Dir::PY => {
                r.extend(vec![(1, -1), (1, 0), (1, 1)]);
                l.extend(vec![(-1, -1), (-1, 0), (-1, 1)]);
            }
            _ => panic!("Wrong combo!"),
        },
        'L' => match dir {
            Dir::NX => {
                l.extend(vec![(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1)]);
                r.extend(vec![(-1, 1)]);
            }
            Dir::PY => {
                r.extend(vec![(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1)]);
                l.extend(vec![(-1, 1)]);
            }
            _ => panic!("Wrong combo!"),
        },
        'J' => match dir {
            Dir::NX => {
                l.extend(vec![(-1, -1)]);
                r.extend(vec![(-1, 1), (0, 1), (1, 1), (1, 0), (1, -1)]);
            }
            Dir::NY => {
                r.extend(vec![(-1, -1)]);
                l.extend(vec![(-1, 1), (0, 1), (1, 1), (1, 0), (1, -1)]);
            }
            _ => panic!("Wrong combo!"),
        },
        '7' => match dir {
            Dir::NY => {
                l.extend(vec![(1, -1)]);
                r.extend(vec![(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)]);
            }
            Dir::PX => {
                r.extend(vec![(1, -1)]);
                l.extend(vec![(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1)]);
            }
            _ => panic!("Wrong combo!"),
        },
        'F' => match dir {
            Dir::PX => {
                l.extend(vec![(1, 1)]);
                r.extend(vec![(-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)]);
            }
            Dir::PY => {
                r.extend(vec![(1, 1)]);
                l.extend(vec![(-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)]);
            }
            _ => panic!("Wrong combo!"),
        },
        _ => (),
    }

    (l, r)
}

fn is_cluster_id_at_border(cluster: &mut Grid<Cluster>, cluster_id: Cluster) -> bool {
    for (i, j) in iproduct!(0..cluster.rows, [0, cluster.cols - 1]) {
        if cluster.get(&(i, j)) == cluster_id {
            return true;
        }
    }

    for (i, j) in iproduct!([0, cluster.rows - 1], 0..cluster.cols) {
        if cluster.get(&(i, j)) == cluster_id {
            return true;
        }
    }
    false
}

fn directed_path(path: &Vec<CellIndex>) -> Vec<Dir> {
    let mut ret: Vec<Dir> = Vec::new();

    for i in 0..path.len() {
        let a = path[i];
        let b = path[(i + 1) % path.len()];

        if a.0 == b.0 && a.1 + 1 == b.1 {
            ret.push(Dir::PY);
        } else if a.0 == b.0 && a.1 == b.1 + 1 {
            ret.push(Dir::NY);
        } else if a.0 + 1 == b.0 && a.1 == b.1 {
            ret.push(Dir::PX);
        } else if a.0 == b.0 + 1 && a.1 == b.1 {
            ret.push(Dir::NX);
        } else {
            panic!("Difference between two successive nodes is more than 1");
        }
    }

    ret
}

fn solve<const PART1: bool>(input: &str) -> usize {
    let original = Grid::from_str(input, |c| c);
    let grid = Grid::<Vec<CellDir>>::from_str(input, |c| match c {
        'F' => vec![(1, 0), (0, 1)],
        'L' => vec![(-1, 0), (0, 1)],
        'J' => vec![(-1, 0), (0, -1)],
        '7' => vec![(1, 0), (0, -1)],
        '|' => vec![(1, 0), (-1, 0)],
        '-' => vec![(0, 1), (0, -1)],
        'S' => vec![(1, 0), (-1, 0), (0, 1), (0, -1)],
        _ => vec![],
    });

    let mut visited = Grid::<bool>::new(grid.rows, grid.cols, false);
    let mut d = true;

    let mut path = Vec::new();
    let mut half_path2 = Vec::new();

    for idx in iproduct!(0..grid.rows, 0..grid.cols) {
        if grid.get(&idx).len() == 4 {
            // Add start to path
            path.push(idx);

            let mut queue = VecDeque::<(CellIndex, bool)>::new();
            let nbs = grid.adjacent_4(&idx);
            for nidx in &nbs {
                let ndirs = grid.get(nidx);
                let nns = grid.adjacent_in_dir(nidx, &ndirs);
                if nns.contains(&idx) {
                    queue.push_back((*nidx, d));
                    d = !d;
                }
            }

            assert!(queue.len() == 2);

            visited.set(&idx, true);
            while !queue.is_empty() {
                let (cidx, lr) = queue.pop_front().unwrap();
                if lr {
                    path.push(cidx);
                } else {
                    half_path2.push(cidx);
                }
                visited.set(&cidx, true);

                let dirs = grid.get(&cidx);
                let nc = grid.adjacent_in_dir(&cidx, &dirs);
                for nidx in &nc {
                    if !visited.get(nidx) {
                        queue.push_back((*nidx, lr));
                    }
                }
            }
        }
    }

    // Since we added the start position to the first half path
    assert!(path.len() == half_path2.len() + 1);

    if PART1 {
        return half_path2.len();
    }

    let mut cluster = Grid::new(grid.rows, grid.cols, Cluster::Empty);

    path.extend(half_path2.iter().rev().skip(1));
    let directed_path = directed_path(&path);

    for idx in &path {
        cluster.set(idx, Cluster::Path);
    }

    for (dir, p) in directed_path.iter().zip(path.iter()) {
        let (l, r) = lr_classification(original.get(p), dir);
        for idx in cluster.adjacent_in_dir(p, &l) {
            if cluster.get(&idx) == Cluster::Empty {
                cluster.set(&idx, Cluster::Side1);
            }
        }
        for idx in cluster.adjacent_in_dir(p, &r) {
            if cluster.get(&idx) == Cluster::Empty {
                cluster.set(&idx, Cluster::Side2);
            }
        }
    }

    cluster.flood_fill(Cluster::Side1, Cluster::Empty);
    cluster.flood_fill(Cluster::Side2, Cluster::Empty);

    // Find out which cluster is at the border
    // The other one is basically the inner one
    if is_cluster_id_at_border(&mut cluster, Cluster::Side1) {
        return cluster.count(&Cluster::Side2);
    }
    cluster.count(&Cluster::Side1)
}

fn main() {
    let input = common::get_input();
    common::timed(&input, solve::<true>, true);
    common::timed(&input, solve::<false>, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input0 = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
        assert_eq!(solve::<true>(sample_input0), 4);

        let sample_input1 = "7-F7-\n.FJ|7\nSJLL7\n|F--J\nLJ.LJ";
        assert_eq!(solve::<true>(sample_input1), 8);

        let sample_input2 = ".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...";

        assert_eq!(solve::<false>(sample_input2), 8);

        let sample_input3 = "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(solve::<false>(sample_input3), 10);

        let sample_input4 = "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";

        assert_eq!(solve::<false>(sample_input4), 4);
    }
}
