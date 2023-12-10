use std::collections::VecDeque;

use aoc::grid::Grid;

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

fn lr_classification(ch: char, dir: &Dir) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
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

fn directed_path(path: &Vec<(usize, usize)>) -> Vec<Dir> {
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

fn grow(cluster: &mut Grid<i32>, cluster_id: i32, replace_id: i32) {
    let mut q = VecDeque::new();

    for i in 0..cluster.rows {
        for j in 0..cluster.cols {
            if cluster.get(i, j).unwrap() == cluster_id {
                q.push_back((i, j));
            }
        }
    }

    let mut visited = Grid::<bool>::new(cluster.rows, cluster.cols, false);
    while !q.is_empty() {
        let x = q.pop_front().unwrap();

        if visited.get(x.0, x.1).unwrap() {
            continue;
        }

        visited.set(x.0, x.1, true);

        cluster.set(x.0, x.1, cluster_id);

        for n in cluster.adjacent_4(x.0, x.1) {
            if cluster.get(n.0, n.1).unwrap() == replace_id {
                q.push_back(n);
            }
        }
    }
}

fn solve(input: &str) {
    let mut ans: usize = 0;

    let original = Grid::from_str(input, |c| c);

    let grid = Grid::<Vec<(i32, i32)>>::from_str(input, |c| match c {
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
    let mut distances = Grid::<usize>::new(grid.rows, grid.cols, 0);
    //let mut path = Grid::<char>::new(grid.rows, grid.cols, '.');

    let mut d = 0;

    let mut half_path1 = Vec::new();
    let mut half_path2 = Vec::new();

    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if grid.get(i, j).unwrap().len() == 4 {
                half_path1.push((i, j));

                println!("Start ({},{})", i, j);
                let mut queue = VecDeque::<(usize, usize, usize, usize)>::new();
                let nbs = grid.adjacent_4(i, j);
                for nb in nbs {
                    let ndirs = grid.get(nb.0, nb.1).unwrap();
                    let nns = grid.adjacent_in_dir(nb.0, nb.1, &ndirs);
                    //println!("{:?} is connected in dir {:?}", nb, ndirs);

                    let is_connected = nns.contains(&(i, j));

                    if is_connected {
                        //println!("Adding {:?}", nb);
                        queue.push_back((nb.0, nb.1, 1, d));
                        d = 1 - d;
                    }
                }

                //println!("{:?}", queue);

                assert!(queue.len() == 2);

                visited.set(i, j, true);

                while !queue.is_empty() {
                    let c = queue.pop_front().unwrap();
                    if c.3 == 0 {
                        half_path1.push((c.0, c.1));
                    } else {
                        half_path2.push((c.0, c.1));
                    }

                    ans = ans.max(c.2);
                    distances.set(c.0, c.1, c.2);

                    // if visited.get(c.0, c.1).unwrap() {
                    //     continue;
                    // }

                    visited.set(c.0, c.1, true);

                    //println!("Visiting node {:?}", c);

                    let dirs = grid.get(c.0, c.1).unwrap();
                    let nc = grid.adjacent_in_dir(c.0, c.1, &dirs);

                    for n in nc {
                        if !visited.get(n.0, n.1).unwrap() {
                            // Path
                            //path.set(n.0, n.1, dir_ch(dir));
                            queue.push_back((n.0, n.1, c.2 + 1, c.3));
                        }
                    }
                }
                distances.set(i, j, 1);
                break;
            }
        }
    }

    let mut cluster = Grid::new(grid.rows, grid.cols, 0);

    cluster.print();

    half_path1.extend(half_path2.iter().rev().skip(1));
    let directed_path = directed_path(&half_path1);
    // println!("HP1 {:?}", half_path1);

    for i in 0..grid.rows {
        for j in 0..grid.cols {
            if distances.get(i, j).unwrap() > 0 {
                cluster.set(i, j, 1)
            }
        }
    }

    cluster.print();

    for (dir, p) in directed_path.iter().zip(half_path1.iter()) {
        let (l, r) = lr_classification(original.get(p.0, p.1).unwrap(), dir);
        for (a, b) in cluster.adjacent_in_dir(p.0, p.1, &l) {
            if cluster.get(a, b).unwrap() == 0 {
                cluster.set(a, b, 2);
            }
        }
        for (a, b) in cluster.adjacent_in_dir(p.0, p.1, &r) {
            if cluster.get(a, b).unwrap() == 0 {
                cluster.set(a, b, 3);
            }
        }
    }

    cluster.print();

    grow(&mut cluster, 2, 0);
    grow(&mut cluster, 3, 0);

    original.print();
    cluster.print();

    let mut cnts: [usize; 4] = [0; 4];

    for i in 0..grid.rows {
        for j in 0..grid.cols {
            cnts[cluster.get(i, j).unwrap() as usize] += 1;
        }
    }

    // Find out from the visualization
    println!("{:?}", cnts);
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        solve(input.1);
    }
}
