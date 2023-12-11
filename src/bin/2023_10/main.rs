use std::collections::VecDeque;

use aoc::grid::{CellDir, CellIndex, Grid};

const INPUT: [(&str, &str); 4] = [
    ("Sample Input 1", include_str!("sample_input.txt")),
    ("Sample Input 2", include_str!("sample_input2.txt")),
    ("Sample Input 3", include_str!("sample_input3.txt")),
    ("Input", include_str!("input.txt")),
];

#[derive(Debug)]
enum Dir {
    PX,
    NX,
    PY,
    NY,
}

#[derive(Clone, Default, Debug, PartialEq)]
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
    for i in 0..cluster.rows {
        for j in [0, cluster.cols - 1] {
            if cluster.get(i, j).unwrap() == cluster_id {
                return true;
            }
        }
    }

    for i in [0, cluster.rows - 1] {
        for j in 0..cluster.cols {
            if cluster.get(i, j).unwrap() == cluster_id {
                return true;
            }
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

fn solve(input: &str) {
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

    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if grid.get(i, j).unwrap().len() == 4 {
                // Add start to path
                path.push((i, j));

                let mut queue = VecDeque::<(usize, usize, bool)>::new();
                let nbs = grid.adjacent_4(i, j);
                for (nx, ny) in nbs {
                    let ndirs = grid.get(nx, ny).unwrap();
                    let nns = grid.adjacent_in_dir(nx, ny, &ndirs);
                    if nns.contains(&(i, j)) {
                        queue.push_back((nx, ny, d));
                        d = !d;
                    }
                }

                assert!(queue.len() == 2);

                visited.set(i, j, true);
                while !queue.is_empty() {
                    let (x, y, lr) = queue.pop_front().unwrap();
                    if lr {
                        path.push((x, y));
                    } else {
                        half_path2.push((x, y));
                    }
                    visited.set(x, y, true);

                    let dirs = grid.get(x, y).unwrap();
                    let nc = grid.adjacent_in_dir(x, y, &dirs);
                    for (nx, ny) in nc {
                        if !visited.get(nx, ny).unwrap() {
                            queue.push_back((nx, ny, lr));
                        }
                    }
                }
            }
        }
    }

    // Since we added the start position to the first half path
    assert!(path.len() == half_path2.len() + 1);

    println!("Answer Part1 {}", half_path2.len());

    let mut cluster = Grid::new(grid.rows, grid.cols, Cluster::Empty);

    path.extend(half_path2.iter().rev().skip(1));
    let directed_path = directed_path(&path);

    for (cx, cy) in &path {
        cluster.set(*cx, *cy, Cluster::Path);
    }

    for (dir, p) in directed_path.iter().zip(path.iter()) {
        let (l, r) = lr_classification(original.get(p.0, p.1).unwrap(), dir);
        for (a, b) in cluster.adjacent_in_dir(p.0, p.1, &l) {
            if cluster.get(a, b).unwrap() == Cluster::Empty {
                cluster.set(a, b, Cluster::Side1);
            }
        }
        for (a, b) in cluster.adjacent_in_dir(p.0, p.1, &r) {
            if cluster.get(a, b).unwrap() == Cluster::Empty {
                cluster.set(a, b, Cluster::Side2);
            }
        }
    }

    cluster.flood_fill(Cluster::Side1, Cluster::Empty);
    cluster.flood_fill(Cluster::Side2, Cluster::Empty);

    // Find out which cluster is at the border
    // The other one is basically the inner one
    if is_cluster_id_at_border(&mut cluster, Cluster::Side1) {
        println!("Answer Part2 {}", cluster.count(&Cluster::Side2));
    } else {
        println!("Answer Part2 {}", cluster.count(&Cluster::Side1));
    }
}

fn main() {
    for (file, input) in INPUT {
        println!("{}", file);
        solve(input);
    }
}