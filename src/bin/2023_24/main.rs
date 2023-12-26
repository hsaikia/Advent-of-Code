use indicatif::ProgressBar;
use std::f64::EPSILON;

use aoc::{common, io};
use glam::{DVec3, I64Vec3};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct Hail {
    p: I64Vec3,
    v: DVec3,
}

impl Hail {

    fn time_to_collision_2d(&self, other: &Hail, d1 : usize, d2 : usize) -> Option<(f64, f64)> {
        let pd = self.p - other.p;
        let den = self.v[d1] * other.v[d2] - other.v[d1] * self.v[d2];
        if den.abs() < EPSILON {
            return None;
        }

        let num1 = other.v[d1] * pd[d2] as f64 - pd[d1] as f64 * other.v[d2];
        let num2 = self.v[d1] * pd[d2] as f64 - pd[d1] as f64 * self.v[d2];

        Some((num1 / den, num2 / den))
    }

    fn time_to_collision_3d(&self, other: &Hail) -> Option<f64> {
        let pd = other.p - self.p;
        let pdf = DVec3{x : pd.x as f64, y : pd.y as f64, z : pd.z as f64};
        let v2_cross_pd = other.v.cross(pdf).length();
        let v2_cross_v1 = other.v.cross(self.v).length();

        if v2_cross_v1 == 0.0 {
            return None;
        }
        Some(v2_cross_pd / v2_cross_v1)
    }

    fn position_in_time(&self, time: f64) -> I64Vec3 {
        self.p + I64Vec3{ x : (self.v.x * time) as i64, y : (self.v.y * time) as i64, z : (self.v.z * time) as i64 }
    }
}

fn intersect(hail1: &Hail, hail2: &Hail, limits: Option<(i64, i64)>) -> bool {
    if let Some((t1, t2)) = hail1.time_to_collision_2d(&hail2, 0, 1) {
        if t1 > 0.0&& t2 > 0.0 {
            if let Some((min_lim, max_lim)) = limits {
                let pos = hail1.position_in_time(t1);
                if pos.x >= min_lim && pos.x <= max_lim && pos.y >= min_lim && pos.y <= max_lim {
                    return true;
                }
            } else {
                return true;
            }
        }
    }
    false
}

fn intersecting_pairs(hails: &Vec<Hail>, limits: Option<(i64, i64)>) -> usize {
    let mut ans = 0;

    for i in 0..hails.len() {
        for j in i + 1..hails.len() {
            ans += if intersect(&hails[i], &hails[j], limits) {
                1
            } else {
                0
            };
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
            p: I64Vec3 {
                x: ps[0],
                y: ps[1],
                z: ps[2],
            },
            v: DVec3 {
                x: vs[0] as f64,
                y: vs[1] as f64,
                z: vs[2] as f64,
            },
        });
    }
    intersecting_pairs(&hail, Some((200000000000000, 400000000000000)))
}

fn test_rock_vel(rock_vel: &DVec3, hails: &Vec<Hail>) -> Option<I64Vec3> {
    // Relative speed of all of the hail stones would be V - V_r
    let mut hails_tmp = hails.clone();
    for hail in &mut hails_tmp {
        hail.v -= *rock_vel;
    }

    // Test for intersections
    let n = hails_tmp.len();
    let mut pos = None;

    for i in 0..n {
        for j in i + 1..n {
            let opt_tcs = hails_tmp[i].time_to_collision_3d(&hails_tmp[j]);
            if let Some(mut t1) = opt_tcs {
                t1 = t1.round();
                let p1 = hails_tmp[i].position_in_time(t1);
                if let Some(ps) = pos {
                    if p1 != ps {
                        return None;
                    }
                } else {
                    pos = Some(p1);
                }
                                
            } 
        }
    }
    pos
}

fn part2(input: &str) -> i64 {
    let mut hails: Vec<Hail> = Vec::new();
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
        hails.push(Hail {
            p: I64Vec3 {
                x: ps[0],
                y: ps[1],
                z: ps[2],
            },
            v: DVec3 {
                x: vs[0] as f64,
                y: vs[1] as f64,
                z: vs[2] as f64,
            },
        });
    }

    const MAX_SPEED: i64 = 500;
    let bar = ProgressBar::new(8 * MAX_SPEED as u64 * MAX_SPEED as u64 * MAX_SPEED as u64);
    let mut opt_rock_pos : Option<I64Vec3> = None;
    for vx in -MAX_SPEED..=MAX_SPEED {
        for vy in -MAX_SPEED..=MAX_SPEED {
            for vz in -MAX_SPEED..=MAX_SPEED {
                bar.inc(1);
                // This is the velocity of the rock V_r
                let rock_vel = DVec3 {
                    x: vx as f64,
                    y: vy as f64,
                    z: vz as f64,
                };

                opt_rock_pos = test_rock_vel(&rock_vel, &hails);
                if let Some(rock_pos) = opt_rock_pos {
                    // All intersections found!
                    println!("Stone Pos {} Possible Ans {}", rock_pos, rock_pos[0] + rock_pos[1] + rock_pos[2]);
                    println!("Stone Vel {}", rock_vel);
                } else {
                    continue;
                }
            }
        }
    }
    opt_rock_pos.unwrap().to_array().iter().sum()
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    common::timed(&input, part2, false);
}
