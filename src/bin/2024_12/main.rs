use aoc::{common, grid::Grid};

fn score(
    farm: &Grid<char>,
    pos: &(usize, usize),
    visited: &mut Vec<(usize, usize)>,
) -> (usize, usize, usize) {
    let mut area = 0;
    let mut peri = 0;
    let mut sides = 0;

    if visited.contains(pos) {
        return (area, peri, sides);
    }

    let mut q = Vec::new();
    q.push(*pos);
    let mut cluster = Vec::new();

    while let Some(tp) = q.pop() {
        if cluster.contains(&tp) {
            continue;
        }
        cluster.push(tp);
        let adj: Vec<(usize, usize)> = farm
            .adjacent_4(&tp)
            .into_iter()
            .filter(|i| farm.get(i) == farm.get(&tp))
            .collect();
        for nx in adj {
            q.push(nx);
        }
    }

    //println!("Plant {} got {} area.", farm.get(pos), cluster.len());
    for c in cluster.iter() {
        visited.push(*c);
    }

    area = cluster.len();
    peri = area * 4;

    let mut corners_map: Grid<usize> = Grid::new(farm.rows + 1, farm.cols + 1, 0);

    for c in cluster.iter() {
        let val = corners_map.get(c);
        corners_map.set(c, val + 1);

        let val = corners_map.get(&(c.0, c.1 + 1));
        corners_map.set(&(c.0, c.1 + 1), val + 1);

        let val = corners_map.get(&(c.0 + 1, c.1 + 1));
        corners_map.set(&(c.0 + 1, c.1 + 1), val + 1);

        let val = corners_map.get(&(c.0 + 1, c.1));
        corners_map.set(&(c.0 + 1, c.1), val + 1);

        peri -= farm
            .adjacent_4(c)
            .iter()
            .filter(|x| cluster.contains(x))
            .count();
    }

    //corners_map.print();

    for i in 0..corners_map.rows {
        for j in 0..corners_map.cols {
            if corners_map.get(&(i, j)) % 2 == 1 {
                sides += 1;
            }
            if corners_map.get(&(i, j)) == 2 {
                if i == 0 || i == corners_map.rows - 1 {
                    continue;
                }
                if j == 0 || j == corners_map.cols - 1 {
                    continue;
                }

                let mut found = false;
                if farm.get(&(i - 1, j)) == farm.get(&(i - 1, j - 1)) {
                    found = true;
                }
                if farm.get(&(i, j - 1)) == farm.get(&(i - 1, j - 1)) {
                    found = true;
                }
                if farm.get(&(i - 1, j)) == farm.get(&(i, j)) {
                    found = true;
                }
                if farm.get(&(i, j - 1)) == farm.get(&(i, j)) {
                    found = true;
                }

                if !found {
                    //println!("Inc sides at pos {i},{j}");
                    sides += 2;
                }
            }
        }
    }

    (area, peri, sides)
}

fn solve<const PART: usize>(input: &str) -> usize {
    let mut ans = 0;
    let farm = Grid::from_str(input, |c| c);
    let mut visited: Vec<(usize, usize)> = Vec::new();
    for p in 'A'..='Z' {
        let pos = farm.positions(p);

        if pos.is_empty() {
            continue;
        }

        for pp in pos.iter() {
            let (area, peri, sides) = score(&farm, pp, &mut visited);
            ans += if PART == 1 { area * peri } else { area * sides };
        }
    }
    ans
}

fn main() {
    let input = common::get_input();
    println!("{input:?}");
    common::timed(&input, solve::<1>, true);
    common::timed(&input, solve::<2>, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "RRRRIICCFF\nRRRRIICCCF\nVVRRRCCFFF\nVVRCCCJFFF\nVVVVCJJCFE\nVVIVCCJJEE\nVVIIICJJEE\nMIIIIIJJEE\nMIIISIJEEE\nMMMISSJEEE";
        assert_eq!(solve::<1>(sample_input), 1930);
        assert_eq!(solve::<2>(sample_input), 1206);
        let sample_input = "AAAA\nBBCD\nBBCC\nEEEC";
        assert_eq!(solve::<1>(sample_input), 140);
        assert_eq!(solve::<2>(sample_input), 80);
        let sample_input = "AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA";
        assert_eq!(solve::<1>(sample_input), 1184);
        assert_eq!(solve::<2>(sample_input), 368);
        let sample_input = "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO";
        assert_eq!(solve::<1>(sample_input), 772);
        assert_eq!(solve::<2>(sample_input), 436);
        let sample_input = "EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE";
        assert_eq!(solve::<1>(sample_input), 692);
        assert_eq!(solve::<2>(sample_input), 236);
    }
}
