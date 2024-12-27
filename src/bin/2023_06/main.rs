use itertools::Itertools;

use aoc::common;

#[allow(dead_code)]
enum Solution {
    BruteForce1,
    BruteForce2,
    BinarySearch,
    Analytic,
}

// Brute force
fn solve_brute_force1(t: usize, d: usize) -> usize {
    (1..t).map(|x| x * (t - x)).filter(|x| *x > d).count()
}

// Solve for x(t - x) = d and find the distance between the two roots
// x^2 - tx + d = 0 has two roots, (t - sqrt(t^2 - 4d)) / 2 and (t + sqrt(t^2 - 4d)) / 2
// Since the winning distances must be strictly greater than d, we must take the ceil of the first root
// and the floor of the second root
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
fn solve_analytic(t: usize, d: usize) -> usize {
    let det = ((t * t - 4 * d) as f64).sqrt();
    let x1f = (t as f64 - det) / 2.0;
    let x2f = (t as f64 + det) / 2.0;
    let x1 = x1f.ceil() as usize + usize::from((x1f.ceil() - x1f).abs() < f64::EPSILON);
    let x2 = x2f.floor() as usize + usize::from((x2f - x2f.floor()).abs() < f64::EPSILON);
    x2 - x1 + 1
}

// Binary search
// As before, we do not solve for the roots analytically but instead perform a binary search
// in the range [0, x/2] where x is in [0, t] since the function x(t - x) = d is quadratic, it is
// monotonic in this range, and hence we can perform binary search
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_sign_loss)]
fn solve_binary_search(t: usize, d: usize) -> usize {
    let mut lo = 0.0;
    let mut hi = t as f64 / 2.0;
    while lo + 0.001 < hi {
        let mid = (lo + hi) / 2.0;
        let y = mid * (t as f64 - mid);
        if y < d as f64 {
            lo = mid;
        } else if y > d as f64 {
            hi = mid;
        }
    }
    if (lo.ceil() - lo).abs() < f64::EPSILON {
        return t - 2 * lo.ceil().abs() as usize - 1;
    }
    t - 2 * lo.ceil().abs() as usize + 1
}

// Pascal's triangle - Brute Force improvement
fn solve_brute_force2(t: usize, d: usize) -> usize {
    let h = t / 2;
    let mut i = 1;
    while i <= h {
        if i * (t - i) > d {
            break;
        }
        i += 1;
    }
    2 * (h - i + 1) - (t + 1) % 2
}

fn solve(t: usize, d: usize, sol_type: &Solution) -> usize {
    match *sol_type {
        Solution::BruteForce1 => solve_brute_force1(t, d),
        Solution::BruteForce2 => solve_brute_force2(t, d),
        Solution::BinarySearch => solve_binary_search(t, d),
        Solution::Analytic => solve_analytic(t, d),
    }
}

fn part1(input: &str) -> usize {
    let lines = input.split('\n').collect::<Vec<_>>();
    let ts = lines[0]
        .split(' ')
        .skip(1)
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let ds = lines[1]
        .split(' ')
        .skip(1)
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut ans: usize = 1;
    for (t, d) in ts.iter().zip(ds.iter()) {
        let num_ways = solve(*t, *d, &Solution::BruteForce1);
        ans *= num_ways;
    }
    ans
}

fn part2(input: &str) -> usize {
    let lines = input.split('\n').collect::<Vec<_>>();
    let t = lines[0]
        .split(' ')
        .skip(1)
        .join("")
        .parse::<usize>()
        .unwrap();
    let d = lines[1]
        .split(' ')
        .skip(1)
        .join("")
        .parse::<usize>()
        .unwrap();

    solve(t, d, &Solution::Analytic)
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    common::timed(&input, part2, false);
}
