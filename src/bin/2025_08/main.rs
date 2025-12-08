use std::collections::HashMap;

use aoc::{
    common::{self, HashMapCount},
    io,
};
use itertools::Itertools;

struct Box {
    pub x: usize,
    y: usize,
    z: usize,
}

impl Box {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    fn dist(b1: &Box, b2: &Box) -> usize {
        let dx = b1.x.abs_diff(b2.x);
        let dy = b1.y.abs_diff(b2.y);
        let dz = b1.z.abs_diff(b2.z);
        dx * dx + dy * dy + dz * dz
    }
}

fn find_group(gm: &HashMap<usize, usize>, g: usize) -> usize {
    let mut ret = g;
    while let Some(i) = gm.get(&ret) {
        ret = *i;
    }
    ret
}

fn all_connected(gm: &HashMap<usize, usize>, groups: usize) -> bool {
    (0..groups).all(|g| find_group(gm, g) == 0)
}

fn solve<const PART: usize, const MIN_CONNECTIONS: usize>(input: &str) -> usize {
    let mut boxes = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let coords: Vec<usize> = io::tokenize_nums(line, ",");
        boxes.push(Box::new(coords[0], coords[1], coords[2]));
    }

    let mut distances = Vec::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            distances.push((i, j, Box::dist(&boxes[i], &boxes[j])));
        }
    }

    distances.sort_by(|a, b| a.2.cmp(&b.2));

    let mut group_mapping: HashMap<usize, usize> = HashMap::new();

    for (i, j, _) in distances.iter().take(if PART == 1 {
        MIN_CONNECTIONS
    } else {
        distances.len()
    }) {
        let g1 = find_group(&group_mapping, *i);
        let g2 = find_group(&group_mapping, *j);

        //println!("Joining {} and {} | groups {} and {}", i, j, g1, g2);
        if g1 == g2 {
            continue;
        }

        let g = g1.min(g2);

        if g1 != g {
            group_mapping.insert(g1, g);
        }
        if g2 != g {
            group_mapping.insert(g2, g);
        }

        if all_connected(&group_mapping, boxes.len()) {
            return boxes[*i].x * boxes[*j].x;
        }
    }

    let mut group_to_member_count: HashMap<usize, usize> = HashMap::new();

    for i in 0..boxes.len() {
        let g = find_group(&group_mapping, i);
        //println!("Box {} belongs to group {}", i, g);
        group_to_member_count.insert_with_count(&g, 1);
    }

    //dbg!(&group_to_member_count);

    group_to_member_count
        .values()
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .product()
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, solve::<1, 1000>, true);
        common::timed(&input, solve::<2, 1000>, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";
        assert_eq!(solve::<1, 10>(sample_input), 40);
        assert_eq!(solve::<2, 10>(sample_input), 25272);
    }
}
