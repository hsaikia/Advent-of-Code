use aoc::{common, grid::Grid, io};
use glam::I64Vec2;

fn largest_cluster(grid: &Grid<char>) -> usize {
    let pos = grid.positions(&'#');
    let mut ans = 0;

    for p in pos {
        let mut cluster: Vec<(usize, usize)> = Vec::new();
        let mut q = Vec::new();
        q.push(p);
        while let Some(tp) = q.pop() {
            if cluster.contains(&tp) {
                break;
            }
            cluster.push(tp);

            for nx in grid.adjacent_4(&tp).iter().filter(|x| grid.get(x) == '#') {
                q.push(*nx);
            }
        }

        ans = ans.max(cluster.len());
    }
    ans
}

fn solve<const PART: usize, const SECONDS: usize, const MAX_X: i64, const MAX_Y: i64>(
    input: &str,
) -> usize {
    let mut ans = 0;
    let mut positions: Vec<I64Vec2> = Vec::new();
    let mut velocities: Vec<I64Vec2> = Vec::new();

    for line in input.lines() {
        let vals = io::tokenize(line, " ");
        let pos = io::tokenize(vals[0], "=")[1];
        let vel = io::tokenize(vals[1], "=")[1];
        let pxy: Vec<i64> = io::tokenize_nums(pos, ",");
        let vxy: Vec<i64> = io::tokenize_nums(vel, ",");

        positions.push(I64Vec2 {
            x: pxy[0],
            y: pxy[1],
        });
        velocities.push(I64Vec2 {
            x: vxy[0],
            y: vxy[1],
        });
    }

    for s in 0..SECONDS {
        let mut quadrant = [0; 4];
        for (curr, vel) in positions.iter_mut().zip(velocities.iter()) {
            *curr += *vel;

            if curr.x >= MAX_X {
                curr.x -= MAX_X;
            }
            if curr.x < 0 {
                curr.x += MAX_X;
            }

            if curr.y >= MAX_Y {
                curr.y -= MAX_Y;
            }
            if curr.y < 0 {
                curr.y += MAX_Y;
            }

            if PART == 1 {
                if curr.x > MAX_X / 2 && curr.y > MAX_Y / 2 {
                    quadrant[0] += 1;
                } else if curr.x < MAX_X / 2 && curr.y > MAX_Y / 2 {
                    quadrant[1] += 1;
                } else if curr.x < MAX_X / 2 && curr.y < MAX_Y / 2 {
                    quadrant[2] += 1;
                } else if curr.x > MAX_X / 2 && curr.y < MAX_Y / 2 {
                    quadrant[3] += 1;
                }
            }
        }

        if PART == 1 {
            ans = quadrant[0] * quadrant[1] * quadrant[2] * quadrant[3];
        } else if PART == 2 {
            let mut map: Grid<char> = Grid::new(
                usize::try_from(MAX_X).unwrap(),
                usize::try_from(MAX_Y).unwrap(),
                '.',
            );
            for p in &positions {
                map.set(
                    &(usize::try_from(p.x).unwrap(), usize::try_from(p.y).unwrap()),
                    '#',
                );
            }

            if largest_cluster(&map) > 10 {
                //println!("After second {}", s + 1);
                //println!("Largest cluster {}", largest_cluster(&map));
                //map.print();
                ans = s + 1;
                break;
            }
        }
    }

    ans
}

fn main() {
    let input = common::get_input();
    //println!("{input:?}");
    common::timed(&input, solve::<1, 100, 101, 103>, true);
    common::timed(&input, solve::<2, 10000, 101, 103>, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "p=0,4 v=3,-3\np=6,3 v=-1,-3\np=10,3 v=-1,2\np=2,0 v=2,-1\np=0,0 v=1,3\np=3,0 v=-2,-2\np=7,6 v=-1,-3\np=3,0 v=-1,-2\np=9,3 v=2,3\np=7,3 v=-1,2\np=2,4 v=2,-3\np=9,5 v=-3,-3";
        assert_eq!(solve::<1, 100, 11, 7>(sample_input), 12);
    }
}
