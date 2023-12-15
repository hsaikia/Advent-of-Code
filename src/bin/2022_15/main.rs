use aoc::{
    common,
    range::{Range, RangeUnion},
};
use itertools::Itertools;

fn part1(coords: &[(i64, i64, i64, i64)], y: i64) {
    let known_beacon_positions = coords
        .iter()
        .filter_map(|coord| if y == coord.3 { Some(coord.2) } else { None })
        .unique()
        .collect::<Vec<_>>();

    let mut range_union = RangeUnion::<i64>::new();

    for coord in coords {
        let d = coord.0.abs_diff(coord.2) + coord.1.abs_diff(coord.3);
        let yd = y.abs_diff(coord.1);
        if yd > d {
            continue;
        }

        let xd = d as i64 - yd as i64;
        range_union.add_range(Range::<i64>::new(coord.0 - xd, coord.0 + xd + 1));
    }

    let pos_cannot_exist = range_union.spread()
        - known_beacon_positions
            .iter()
            .filter(|&x| range_union.contains(*x))
            .count() as i64;
    println!("Answer Part 1 {}", pos_cannot_exist);
}

fn part2(coords: &[(i64, i64, i64, i64)], xy_max: i64) {
    let range_limit = Range::<i64>::new(0, xy_max);

    for y in 0..=xy_max {
        let mut range_union = RangeUnion::<i64>::new();

        for coord in coords {
            let d = coord.0.abs_diff(coord.2) + coord.1.abs_diff(coord.3);
            let yd = y.abs_diff(coord.1);
            if yd > d {
                continue;
            }

            let xd = d as i64 - yd as i64;
            let range = Range::<i64>::new(coord.0 - xd, coord.0 + xd + 1);

            if let Some(r) = range.intersect(&range_limit) {
                range_union.add_range(r);
            }
            //println!("{:?}", range_union);
        }

        if range_union.spread() < xy_max {
            for x in 0..=xy_max {
                if range_union.contains(x) {
                    continue;
                }
                println!("X = {} Y = {}. Ans Part 2 {}", x, y, 4000000 * x + y);
                break;
            }
        }
    }
}

fn main() {
    let input = common::get_input();

    let mut coords: Vec<(i64, i64, i64, i64)> = Vec::new();
    for line in input.split('\n') {
        let tokens = line
            .split(' ')
            .filter(|&s| s.contains('='))
            .collect::<Vec<_>>();
        //println!("{:?}", tokens);
        coords.push((
            tokens[0][2..tokens[0].len() - 1].parse::<i64>().unwrap(),
            tokens[1][2..tokens[1].len() - 1].parse::<i64>().unwrap(),
            tokens[2][2..tokens[2].len() - 1].parse::<i64>().unwrap(),
            tokens[3][2..tokens[3].len()].parse::<i64>().unwrap(),
        ));
    }

    part1(&coords, 2000000);
    part2(&coords, 4000000);
}
