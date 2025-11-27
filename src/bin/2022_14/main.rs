use aoc::{common, io};
use std::cmp::Ordering;

fn part1(input_lines: &str) -> usize {
    let mut blocked: Vec<(i32, i32)> = Vec::new();
    let mut max_depth = 0;
    for line in input_lines.split('\n') {
        let tokens = io::tokenize(line, " -> ");
        let mut opt_prev_coord = None;
        for token in tokens {
            let coords = token
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            max_depth = max_depth.max(coords[1]);
            if opt_prev_coord.is_none() {
                blocked.push((coords[0], coords[1]));
                opt_prev_coord = Some(coords);
            } else if let Some(prev_coord) = &mut opt_prev_coord {
                let (mut dx, mut dy) = (coords[0] - prev_coord[0], coords[1] - prev_coord[1]);
                if dx != 0 {
                    dx /= dx.abs();
                }
                if dy != 0 {
                    dy /= dy.abs();
                }
                while coords.cmp(prev_coord) != Ordering::Equal {
                    prev_coord[0] += dx;
                    prev_coord[1] += dy;
                    blocked.push((prev_coord[0], prev_coord[1]));
                }
            }
        }
    }

    //println!("{:?} Max depth {}", blocked, max_depth);
    let rocks = blocked.len();

    let dir = [(0, 1), (-1, 1), (1, 1)];
    loop {
        let mut sand_pos = (500, 0);
        //println!("New Sand .. {}", blocked.len());
        loop {
            //println!("Sand moving.. {:?}", sand_pos);
            let mut settled = true;
            for d in dir {
                let new_sand_pos = (sand_pos.0 + d.0, sand_pos.1 + d.1);
                if !blocked.contains(&new_sand_pos) {
                    sand_pos = new_sand_pos;

                    settled = false;
                    break;
                }
            }

            if settled {
                blocked.push(sand_pos);
                break;
            }

            if sand_pos.1 > max_depth {
                return blocked.len() - rocks;
            }
        }
    }
}

fn part2(input_lines: &str) -> usize {
    let mut blocked: Vec<(i32, i32)> = Vec::new();
    let mut max_depth = 0;
    for line in input_lines.split('\n') {
        let tokens = io::tokenize(line, " -> ");
        let mut opt_prev_coord = None;
        for token in tokens {
            let coords = token
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            max_depth = max_depth.max(coords[1]);
            if opt_prev_coord.is_none() {
                blocked.push((coords[0], coords[1]));
                opt_prev_coord = Some(coords);
            } else if let Some(prev_coord) = &mut opt_prev_coord {
                let (mut dx, mut dy) = (coords[0] - prev_coord[0], coords[1] - prev_coord[1]);
                if dx != 0 {
                    dx /= dx.abs();
                }
                if dy != 0 {
                    dy /= dy.abs();
                }
                while coords.cmp(prev_coord) != Ordering::Equal {
                    prev_coord[0] += dx;
                    prev_coord[1] += dy;
                    blocked.push((prev_coord[0], prev_coord[1]));
                }
            }
        }
    }

    //println!("{:?} Max depth {}", blocked, max_depth);
    let rocks = blocked.len();

    let dir = [(0, 1), (-1, 1), (1, 1)];
    loop {
        let mut sand_pos = (500, 0);

        if blocked.contains(&sand_pos) {
            return blocked.len() - rocks;
        }

        //println!("New Sand .. {}", blocked.len());
        loop {
            //println!("Sand moving.. {:?}", sand_pos);
            let mut settled = true;
            for d in dir {
                let new_sand_pos = (sand_pos.0 + d.0, sand_pos.1 + d.1);
                if !blocked.contains(&new_sand_pos) {
                    sand_pos = new_sand_pos;

                    settled = false;
                    break;
                }
            }

            if settled || sand_pos.1 == max_depth + 1 {
                blocked.push(sand_pos);
                break;
            }
        }
    }
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, part1, true);
        common::timed(&input, part2, false);
    }
}
