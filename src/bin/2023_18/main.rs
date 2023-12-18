use aoc::{
    analytic::{self, picks_formula},
    common,
    io,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum CardinalDirection {
    Right,
    Down,
    Left,
    Up,
    None,
}

const DIRS: [CardinalDirection; 4] = [
    CardinalDirection::Right,
    CardinalDirection::Down,
    CardinalDirection::Left,
    CardinalDirection::Up,
];

impl CardinalDirection {
    pub fn right(&self) -> CardinalDirection {
        DIRS[(*self as usize + 1) % 4]
    }

    pub fn left(&self) -> CardinalDirection {
        DIRS[(*self as usize + 3) % 4]
    }

    pub fn opp(&self) -> CardinalDirection {
        match self {
            CardinalDirection::Up => CardinalDirection::Down,
            CardinalDirection::Down => CardinalDirection::Up,
            CardinalDirection::Left => CardinalDirection::Right,
            CardinalDirection::Right => CardinalDirection::Left,
            _ => CardinalDirection::None,
        }
    }

    pub fn to_dir(self) -> (i64, i64) {
        match self {
            CardinalDirection::Up => (-1, 0),
            CardinalDirection::Down => (1, 0),
            CardinalDirection::Left => (0, -1),
            CardinalDirection::Right => (0, 1),
            _ => (0, 0),
        }
    }
}

fn reduce(v: &mut Vec<(CardinalDirection, i64)>) -> i64 {
    let mut ans = 0;

    if v.len() == 2 {
        assert!(v[0].0.opp() == v[1].0);
        assert!(v[0].1 == v[1].1);
        ans = v[0].1;
        v.clear();
        return ans;
    }

    for i in 0..v.len() - 1 {
        if v[i].0 == v[i + 1].0.opp() {
            if v[i].1 > v[i + 1].1 {
                ans = v[i + 1].1;
                v[i].1 -= v[i + 1].1;
                v.remove(i + 1);
            } else if v[i].1 < v[i + 1].1 {
                ans = v[i].1;
                v[i + 1].1 -= v[i].1;
                v.remove(i);
            } else {
                ans = v[i].1;
                v.remove(i + 1);
                v.remove(i);
            }
            return ans;
        } else if v[i].0 == v[i + 1].0 {
            v[i + 1].1 += v[i].1;
            v.remove(i);
            return ans;
        }
    }

    for i in 1..v.len() - 1 {
        if (v[i - 1].0 == CardinalDirection::Right
            && v[i].0 == CardinalDirection::Down
            && v[i + 1].0 == CardinalDirection::Left)
            || (v[i - 1].0 == CardinalDirection::Left
                && v[i].0 == CardinalDirection::Up
                && v[i + 1].0 == CardinalDirection::Right)
        {
            if v[i - 1].1 < v[i + 1].1 {
                ans = (v[i - 1].1) * (v[i].1 + 1);
                v[i + 1].1 -= v[i - 1].1;
                v.remove(i - 1);
            } else if v[i - 1].1 > v[i + 1].1 {
                ans = (v[i + 1].1) * (v[i].1 + 1);
                v[i - 1].1 -= v[i + 1].1;
                v.remove(i + 1);
            } else {
                ans = (v[i - 1].1) * (v[i].1 + 1);
                v.remove(i + 1);
                v.remove(i - 1);
            }
            return ans;
        }
        if (v[i - 1].0 == CardinalDirection::Right
            && v[i].0 == CardinalDirection::Up
            && v[i + 1].0 == CardinalDirection::Left)
            || (v[i - 1].0 == CardinalDirection::Left
                && v[i].0 == CardinalDirection::Down
                && v[i + 1].0 == CardinalDirection::Right)
        {
            if v[i - 1].1 < v[i + 1].1 {
                ans = -(v[i - 1].1) * (v[i].1 - 1);
                v[i + 1].1 -= v[i - 1].1;
                v.remove(i - 1);
            } else if v[i - 1].1 > v[i + 1].1 {
                ans = -(v[i + 1].1) * (v[i].1 - 1);
                v[i - 1].1 -= v[i + 1].1;
                v.remove(i + 1);
            } else {
                ans = -(v[i - 1].1) * (v[i].1 - 1);
                v.remove(i + 1);
                v.remove(i - 1);
            }
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
            match tok[0] {
                "R" => CardinalDirection::Right,
                "L" => CardinalDirection::Left,
                "U" => CardinalDirection::Up,
                "D" => CardinalDirection::Down,
                _ => CardinalDirection::None,
            }
        } else {
            DIRS[io::parse_num::<usize>(&tok[2][7..8]).unwrap()]
        };
        v.push((dir, hops));
    }

    let mut ans = 0;
    while !v.is_empty() {
        ans += reduce(&mut v);
    }
    ans + 1
}

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
            match tok[0] {
                "R" => CardinalDirection::Right.to_dir(),
                "L" => CardinalDirection::Left.to_dir(),
                "U" => CardinalDirection::Up.to_dir(),
                "D" => CardinalDirection::Down.to_dir(),
                _ => (0, 0),
            }
        } else {
            DIRS[io::parse_num::<usize>(&tok[2][7..8]).unwrap()].to_dir()
        };
        curr_position.0 += dir.0 * hops as i64;
        curr_position.1 += dir.1 * hops as i64;
    }

    let area = analytic::shoelace_formula(&positions).abs() / 2;
    let inner_points = picks_formula(area as usize, num_boundary_points);
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
