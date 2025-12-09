use std::collections::HashSet;

use aoc::{common, io};

fn intersect_inner(boundary: &[&(usize, usize)], p1: &(usize, usize), p2: &(usize, usize)) -> bool {
    boundary
        .iter()
        .any(|b| b.0 > p1.0 && b.0 < p2.0 && b.1 > p1.1 && b.1 < p2.1)
}

fn solve<const PART: usize>(input: &str) -> usize {
    let mut corner_pts = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let corners = io::tokenize_nums::<usize>(line, ",");
        corner_pts.push(corners);
    }

    let mut boundary: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..corner_pts.len() {
        for j in i + 1..corner_pts.len() {
            if corner_pts[i][0] == corner_pts[j][0] {
                let y1 = corner_pts[i][1].min(corner_pts[j][1]);
                let y2 = corner_pts[i][1].max(corner_pts[j][1]);
                for y in y1..=y2 {
                    boundary.insert((corner_pts[i][0], y));
                }
            }

            if corner_pts[i][1] == corner_pts[j][1] {
                let x1 = corner_pts[i][0].min(corner_pts[j][0]);
                let x2 = corner_pts[i][0].max(corner_pts[j][0]);
                for x in x1..=x2 {
                    boundary.insert((x, corner_pts[i][1]));
                }
            }
        }
    }

    let boundary_vec: Vec<&(usize, usize)> = boundary.iter().collect();

    let mut max_area = 0;
    for i in 0..corner_pts.len() {
        for j in i + 1..corner_pts.len() {
            let xmin = corner_pts[i][0].min(corner_pts[j][0]);
            let xmax = corner_pts[i][0].max(corner_pts[j][0]);
            let ymin = corner_pts[i][1].min(corner_pts[j][1]);
            let ymax = corner_pts[i][1].max(corner_pts[j][1]);
            let x = xmax - xmin + 1;
            let y = ymax - ymin + 1;

            if x * y <= max_area {
                continue;
            }

            if PART == 1 {
                max_area = max_area.max(x * y);
            } else if !intersect_inner(&boundary_vec, &(xmin, ymin), &(xmax, ymax)) {
                //println!("{:?} {:?} => {}", corner_pts[i], corner_pts[j], x * y);
                max_area = max_area.max(x * y);
            }
        }
    }

    max_area
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, solve::<1>, true);
        common::timed(&input, solve::<2>, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";
        assert_eq!(solve::<1>(sample_input), 50);
        assert_eq!(solve::<2>(sample_input), 24);
    }
}
