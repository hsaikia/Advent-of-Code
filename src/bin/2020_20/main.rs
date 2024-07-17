use std::collections::HashMap;

use aoc::{
    common::{self, HashMapVector},
    grid::{CardinalDirection, Grid},
    io,
};
use itertools::{iproduct, Itertools};

#[derive(Clone, Debug)]
enum Order {
    Straight,
    Reverse,
}

fn border(g: &Grid<char>, dir: &CardinalDirection) -> Vec<char> {
    match *dir {
        CardinalDirection::North => g.values[0].clone(),
        CardinalDirection::South => g.values[g.rows - 1].clone(),
        CardinalDirection::West => (0..g.rows).map(|r| g.values[r][0]).collect_vec(),
        CardinalDirection::East => (0..g.rows).map(|r| g.values[r][g.cols - 1]).collect_vec(),
    }
}

fn matching_borders(
    g1: (&Grid<char>, &CardinalDirection),
    g2: (&Grid<char>, &CardinalDirection),
) -> Option<Order> {
    let val1 = border(g1.0, g1.1);
    let val2 = border(g2.0, g2.1);

    if val1 == val2 {
        return Some(Order::Straight);
    }

    if val1 == val2.into_iter().rev().collect_vec() {
        return Some(Order::Reverse);
    }

    None
}

fn part1(input: &str) -> usize {
    let grid_strs = io::line_batches(input);
    let n = grid_strs.len();
    println!("Cells {}", n);
    let mut grid_with_ids: Vec<(Grid<char>, usize)> = Vec::new();

    for g in grid_strs {
        let id: usize = io::parse_num(g[0]);
        let g = Grid::from_str(&g[1..].join("\n"), |c| c);
        grid_with_ids.push((g, id));
    }

    //println!("{:?}", grid_with_ids);

    let mut matches: HashMap<usize, Vec<(CardinalDirection, usize, CardinalDirection, Order)>> =
        HashMap::new();

    let cds = [
        CardinalDirection::North,
        CardinalDirection::East,
        CardinalDirection::South,
        CardinalDirection::West,
    ];

    for (i, dir1) in iproduct!(0..n, cds.iter()) {
        for (j, dir2) in iproduct!(i + 1..n, cds.iter()) {
            if let Some(m) =
                matching_borders((&grid_with_ids[i].0, dir1), (&grid_with_ids[j].0, dir2))
            {
                matches.add_to_vector_hashmap(
                    &grid_with_ids[i].1,
                    (*dir1, grid_with_ids[j].1, *dir2, m.clone()),
                );
                matches.add_to_vector_hashmap(
                    &grid_with_ids[j].1,
                    (*dir2, grid_with_ids[i].1, *dir1, m),
                );
            }
        }
    }

    let ans1 = matches
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(k, _)| *k)
        .product::<usize>();
    for (key, val) in matches.iter() {
        if val.len() == 2 {
            println!("{:?} => {:?}", key, val);
        }
    }

    ans1
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
}
