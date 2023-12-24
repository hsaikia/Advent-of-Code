use std::{
    collections::{HashMap, HashSet, VecDeque},
    f64::EPSILON,
};

use aoc::{
    common,
    grid::{CellDir, CellIndex, Grid},
    io,
};
use glam::DVec3;
use num::complex::ComplexFloat;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct Hail {
    p: DVec3,
    v: DVec3,
}

impl Hail {
    fn time_to_collision(&self, other: &Hail) -> Option<(f64, f64)> {
        let pd = self.p - other.p;
        //println!("{}", pd);

        let den = self.v.x * other.v.y - other.v.x * self.v.y;
        if den.abs() < EPSILON {
            return None;
        }

        let num1 = other.v.x * pd.y - pd.x * other.v.y;
        let num2 = self.v.x * pd.y - pd.x * self.v.y;

        Some((num1 / den, num2 / den))
    }

    fn position_in_time(&self, time: f64) -> DVec3 {
        self.p + self.v * time
    }
}

fn intersecting_pairs(hails: &Vec<Hail>, min_lim: f64, max_lim: f64) -> usize {
    let mut ans = 0;

    for i in 0..hails.len() {
        for j in i + 1..hails.len() {
            if let Some((t1, t2)) = hails[i].time_to_collision(&hails[j]) {
                //let t = t.floor();
                if t1 > 0.0 && t2 > 0.0 {
                    let pos = hails[i].position_in_time(t1);
                    //let pos2 = hails[j].position_in_time(t2);

                    //assert_eq!(pos, pos2);

                    if pos.x >= min_lim && pos.x <= max_lim && pos.y >= min_lim && pos.y <= max_lim
                    {
                        println!(
                            "Hail {} {:?} and Hail {} {:?} are colliding  at x = {} y = {}",
                            i, hails[i], j, hails[j], pos.x, pos.y
                        );
                        ans += 1
                    }
                }
            }
        }
    }

    ans
}

fn part1(input: &str) -> usize {
    let mut hail: Vec<Hail> = Vec::new();
    for line in input.lines() {
        let (pos, vel) = line.split_once(" @ ").unwrap();
        let ps = io::tokenize(pos, ", ")
            .iter()
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let vs = io::tokenize(vel, ", ")
            .iter()
            .map(|s| s.trim().parse::<f64>().unwrap())
            .collect::<Vec<_>>();
        hail.push(Hail {
            p: DVec3 {
                x: ps[0] as f64,
                y: ps[1] as f64,
                z: ps[2] as f64,
            },
            v: DVec3 {
                x: vs[0] as f64,
                y: vs[1] as f64,
                z: vs[2] as f64,
            },
        });
    }

    //println!("{:?}", hail);

    //intersecting_pairs(&hail, 7.0, 27.0)
    intersecting_pairs(&hail, 200000000000000.0, 400000000000000.0)
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    //common::timed(&input, part2, false);
}
