use aoc::{common, grid::CellDir, io};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum CardinalDirection {
    Right,
    Down,
    Left,
    Up,
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
        }
    }

    pub fn to_dir(self) -> CellDir {
        match self {
            CardinalDirection::Up => (-1, 0),
            CardinalDirection::Down => (1, 0),
            CardinalDirection::Left => (0, -1),
            CardinalDirection::Right => (0, 1),
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

fn part2(input: &str) -> i64 {
    let mut v: Vec<(CardinalDirection, i64)> = Vec::new();
    for line in input.lines() {
        let tok = io::tokenize(line, " ");
        let hx = i64::from_str_radix(&tok[2][2..7], 16).unwrap();
        let dir = DIRS[io::parse_num::<usize>(&tok[2][7..8]).unwrap()];
        v.push((dir, hx));
    }

    let mut ans = 0;
    while !v.is_empty() {
        ans += reduce(&mut v);
    }
    ans + 1
}

fn part1(input: &str) -> i64 {
    let mut v: Vec<(CardinalDirection, i64)> = Vec::new();
    for line in input.lines() {
        let tok = io::tokenize(line, " ");
        let x = tok[1].parse::<i64>().unwrap();
        match tok[0] {
            "R" => {
                v.push((CardinalDirection::Right, x));
            }
            "L" => {
                v.push((CardinalDirection::Left, x));
            }
            "U" => {
                v.push((CardinalDirection::Up, x));
            }
            "D" => {
                v.push((CardinalDirection::Down, x));
            }
            _ => (),
        }
    }

    let mut ans = 0;
    while !v.is_empty() {
        ans += reduce(&mut v);
    }
    ans + 1
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    common::timed(&input, part2, false);
}
