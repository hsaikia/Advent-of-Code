use aoc::{analytic, common, grid::CardinalDirection, io};

const DIRS: [CardinalDirection; 4] = [
    CardinalDirection::East,
    CardinalDirection::South,
    CardinalDirection::West,
    CardinalDirection::North,
];

fn reduce(v: &mut Vec<(CardinalDirection, i64)>) -> i64 {
    let mut ans = 0;

    if v.len() == 2 {
        assert!(v[0].0.opposite() == v[1].0);
        assert!(v[0].1 == v[1].1);
        ans = v[0].1;
        v.clear();
        return ans;
    }

    for i in 0..v.len() - 1 {
        if v[i].0 == v[i + 1].0.opposite() {
            ans = v[i].1.min(v[i + 1].1);
            v[i].1 -= ans;
            v[i + 1].1 -= ans;
            v.retain(|(_, hops)| *hops != 0);
            return ans;
        } else if v[i].0 == v[i + 1].0 {
            v[i + 1].1 += v[i].1;
            v.remove(i);
            return ans;
        }
    }

    for i in 1..v.len() - 1 {
        if (v[i - 1].0 == CardinalDirection::East
            && v[i].0 == CardinalDirection::South
            && v[i + 1].0 == CardinalDirection::West)
            || (v[i - 1].0 == CardinalDirection::West
                && v[i].0 == CardinalDirection::North
                && v[i + 1].0 == CardinalDirection::East)
        {
            let mi = v[i - 1].1.min(v[i + 1].1);
            ans = mi * (v[i].1 + 1);
            v[i - 1].1 -= mi;
            v[i + 1].1 -= mi;
            v.retain(|(_, hops)| *hops != 0);
            return ans;
        }
        if (v[i - 1].0 == CardinalDirection::East
            && v[i].0 == CardinalDirection::North
            && v[i + 1].0 == CardinalDirection::West)
            || (v[i - 1].0 == CardinalDirection::West
                && v[i].0 == CardinalDirection::South
                && v[i + 1].0 == CardinalDirection::East)
        {
            let mi = v[i - 1].1.min(v[i + 1].1);
            ans = -mi * (v[i].1 - 1);
            v[i - 1].1 -= mi;
            v[i + 1].1 -= mi;
            v.retain(|(_, hops)| *hops != 0);
            return ans;
        }
    }

    ans
}

fn incremental_reduction<const PART1: bool>(input: &str) -> i64 {
    let mut v: Vec<(CardinalDirection, i64)> = Vec::new();

    for line in input.lines() {
        let tok = io::tokenize(line, " ");
        let hops = if PART1 {
            tok[1].parse::<i64>().unwrap()
        } else {
            i64::from_str_radix(&tok[2][2..7], 16).unwrap()
        };

        let dir = if PART1 {
            match *tok.first().unwrap() {
                "R" => CardinalDirection::East,
                "L" => CardinalDirection::West,
                "U" => CardinalDirection::North,
                "D" => CardinalDirection::South,
                _ => panic!("Bad direction!"),
            }
        } else {
            DIRS[io::parse_num::<usize>(&tok[2][7..8])]
        };
        v.push((dir, hops));
    }

    let mut ans = 0;
    while !v.is_empty() {
        ans += reduce(&mut v);
    }
    ans + 1
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_sign_loss)]
fn analytic<const PART1: bool>(input: &str) -> usize {
    let mut positions: Vec<(i64, i64)> = Vec::new();
    let mut curr_position = (0, 0);
    let mut num_boundary_points = 0;
    for line in input.lines() {
        positions.push(curr_position);
        let tok = io::tokenize(line, " ");
        let hops = if PART1 {
            tok[1].parse::<usize>().unwrap()
        } else {
            usize::from_str_radix(&tok[2][2..7], 16).unwrap()
        };
        num_boundary_points += hops;
        let dir = if PART1 {
            match *tok.first().unwrap() {
                "R" => CardinalDirection::East.to_dir(),
                "L" => CardinalDirection::West.to_dir(),
                "U" => CardinalDirection::North.to_dir(),
                "D" => CardinalDirection::South.to_dir(),
                _ => (0, 0),
            }
        } else {
            DIRS[io::parse_num::<usize>(&tok[2][7..8])].to_dir()
        };
        curr_position.0 += i64::from(dir.0) * hops as i64;
        curr_position.1 += i64::from(dir.1) * hops as i64;
    }

    let area = analytic::polygon_area(&positions).abs() / 2;
    let inner_points = analytic::polygon_inner_vertices(area as usize, num_boundary_points);
    num_boundary_points + inner_points
}

fn main() {
    let input = common::get_input();
    println!("Incremental Reduction");
    common::timed(&input, incremental_reduction::<true>, true);
    common::timed(&input, incremental_reduction::<false>, false);
    println!("\nAnalytic");
    common::timed(&input, analytic::<true>, true);
    common::timed(&input, analytic::<false>, false);
}
