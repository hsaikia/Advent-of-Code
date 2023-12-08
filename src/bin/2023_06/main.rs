use std::time::Instant;

const INPUT: [(&str, &str, (usize, usize)); 2] = [
    (
        "Sample Input",
        include_str!("sample_input.txt"),
        (71530, 940200),
    ),
    (
        "Input",
        include_str!("input.txt"),
        (53916768, 250133010811025),
    ),
];

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
fn solve_analytic(t: usize, d: usize) -> usize {
    let det = ((t * t - 4 * d) as f64).sqrt();
    let x1f = (t as f64 - det) / 2.0;
    let x2f = (t as f64 + det) / 2.0;
    let x1 = x1f.ceil() as usize + ((x1f.ceil() - x1f).abs() < f64::EPSILON) as usize;
    let x2 = x2f.floor() as usize + ((x2f - x2f.floor()).abs() < f64::EPSILON) as usize;
    x2 - x1 + 1
}

// Binary search
// As before, we do not solve for the roots analytically but instead perform a binary search
// in the range [0, x/2] where x is in [0, t] since the function x(t - x) = d is quadratic, it is
// monotonic in this range, and hence we can perform binary search
fn solve_binary_search(t: usize, d: usize) -> usize {
    let mut lo = 0 as f64;
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
        return t - 2 * lo.ceil() as usize - 1;
    }
    t - 2 * lo.ceil() as usize + 1
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

fn solve(t: usize, d: usize, sol_type: Solution) -> usize {
    match sol_type {
        Solution::BruteForce1 => solve_brute_force1(t, d),
        Solution::BruteForce2 => solve_brute_force2(t, d),
        Solution::BinarySearch => solve_binary_search(t, d),
        Solution::Analytic => solve_analytic(t, d),
    }
}

fn part1(input: &str) {
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
    let start = Instant::now();
    for (t, d) in ts.iter().zip(ds.iter()) {
        let num_ways = solve(*t, *d, Solution::BruteForce1);
        ans *= num_ways;
    }
    let duration = start.elapsed();
    println!("Time elapsed in Part 1 is: {:?}", duration);

    println!("Part 1 Ans {}", ans);
}

fn part2(t: usize, d: usize) {
    let start = Instant::now();
    let num_ways = solve(t, d, Solution::Analytic);
    let duration = start.elapsed();
    println!("Time elapsed in Part 2 is: {:?}", duration);
    println!("Part 2 Ans {}", num_ways);
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        part1(input.1);
        part2(input.2 .0, input.2 .1);
    }
}
