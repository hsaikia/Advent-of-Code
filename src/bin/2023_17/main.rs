use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use aoc::{
    common,
    grid::{CellDir, CellIndex, Grid},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
enum CardDir {
    Up,
    Down,
    Right,
    Left,
}

const DIRS: [CardDir; 4] = [CardDir::Up, CardDir::Down, CardDir::Right, CardDir::Left];

impl CardDir {
    fn next(&self) -> Vec<CardDir> {
        match self {
            CardDir::Up => {
                vec![CardDir::Left, CardDir::Right]
            }
            CardDir::Down => {
                vec![CardDir::Left, CardDir::Right]
            }
            CardDir::Left => {
                vec![CardDir::Up, CardDir::Down]
            }
            CardDir::Right => {
                vec![CardDir::Up, CardDir::Down]
            }
        }
    }

    fn to_dir(self) -> CellDir {
        match self {
            CardDir::Up => (-1, 0),
            CardDir::Down => (1, 0),
            CardDir::Left => (0, -1),
            CardDir::Right => (0, 1),
        }
    }
}

fn get_next_directions2(dir: &CardDir, hops: usize) -> Vec<(CardDir, usize)> {
    let mut next_dir: Vec<(CardDir, usize)> = Vec::new();
    if hops < 4 {
        next_dir.push((*dir, hops + 1));
    } else if hops < 10 {
        for ndx in dir.next() {
            next_dir.push((ndx, 1));
        }
        next_dir.push((*dir, hops + 1));
    } else {
        for ndx in dir.next() {
            next_dir.push((ndx, 1));
        }
    }
    next_dir
}

fn get_next_directions1(dir: &CardDir, hops: usize) -> Vec<(CardDir, usize)> {
    let mut next_dir: Vec<(CardDir, usize)> = Vec::new();

    for ndx in dir.next() {
        next_dir.push((ndx, 1));
    }

    if hops < 3 {
        next_dir.push((*dir, hops + 1));
    }
    next_dir
}

fn dijkstra<const PART1: bool>(start: CellIndex, g: &Grid<u32>) -> u32 {
    let mut distances: HashMap<(CellIndex, usize, usize), u32> = HashMap::new();
    let mut pq: BinaryHeap<Reverse<(CellIndex, usize, usize, u32)>> = BinaryHeap::new();

    for i in 0..g.rows {
        for j in 0..g.cols {
            for dir in 0..4 {
                distances.insert(((i, j), dir, 0), u32::MAX);
            }
        }
    }

    pq.push(Reverse((start, CardDir::Right as usize, 0, 0)));
    pq.push(Reverse((start, CardDir::Down as usize, 0, 0)));

    while let Some(Reverse((c, dir, hops, dist))) = pq.pop() {
        if c.0 == g.rows - 1 && c.1 == g.cols - 1 {
            return dist;
        }

        let next_dir = if PART1 {
            get_next_directions1(&DIRS[dir], hops)
        } else {
            get_next_directions2(&DIRS[dir], hops)
        };
        for nd in &next_dir {
            let (dx, dy) = nd.0.to_dir();
            if let Some(nc) = g.cell_in_direction(c.0, c.1, dx, dy) {
                let new_dist = dist + g.get(nc.0, nc.1).unwrap();

                if let Some(dist1) = distances.get_mut(&(nc, nd.0 as usize, nd.1)) {
                    if *dist1 > new_dist {
                        *dist1 = new_dist;
                        pq.push(Reverse((nc, nd.0 as usize, nd.1, new_dist)));
                        //println!("  Adding {:?} D {}", nc, new_dist);
                    }
                } else {
                    distances.insert((nc, nd.0 as usize, nd.1), new_dist);
                    pq.push(Reverse((nc, nd.0 as usize, nd.1, new_dist)));
                    //println!("  Adding {:?} D {}", nc, new_dist);
                }
            }
        }
    }

    u32::MAX
}

fn part1(input: &str) -> u32 {
    let g = Grid::from_str(input, |c| c.to_digit(10).unwrap());
    dijkstra::<true>((0, 0), &g)
}

fn part2(input: &str) -> u32 {
    let g = Grid::from_str(input, |c| c.to_digit(10).unwrap());
    dijkstra::<false>((0, 0), &g)
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    common::timed(&input, part2, false);
}
