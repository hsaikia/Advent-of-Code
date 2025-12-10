use std::collections::{HashMap, VecDeque};

use aoc::{common, io};
use rand::Rng;

fn compute_cost(sol: &[usize], cost: &mut usize, wir: &[Vec<usize>], jolt: &[usize]) {
    let mut val = vec![0; jolt.len()];
    for (w, s) in wir.iter().zip(sol.iter()) {
        for idx in w {
            val[*idx] += s;
        }
    }

    let mut bad = false;
    let mut c = 0;
    for (v, j) in val.iter().zip(jolt.iter()) {
        if v > j {
            bad = true;
            break;
        }
        c += 10000 * (j - v);
    }

    if bad {
        *cost = usize::MAX;
        return;
    }
    *cost = c + sol.iter().sum::<usize>();
}

fn solve2(input: &str) -> usize {
    let mut ans2 = 0;
    let mut patterns = Vec::new();
    let mut wirings = Vec::new();
    let mut joltages = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let tokens = io::tokenize(line, " ");
        let mut wiring_single = Vec::new();
        let mut joltage_single = Vec::new();

        for token in tokens {
            if token.contains("[") {
                patterns.push(token[1..token.len() - 1].to_string());
            } else if token.contains("(") {
                let wiring = io::tokenize_nums::<usize>(&token[1..token.len() - 1], ",");
                //dbg!(&wiring);
                wiring_single.push(wiring);
            } else {
                joltage_single = io::tokenize_nums::<usize>(&token[1..token.len() - 1], ",")
            }
        }
        joltages.push(joltage_single);
        wirings.push(wiring_single);
    }

    let mut rng = rand::thread_rng();
    const SCALE: usize = 50;
    const NUM_SOLUTIONS: usize = SCALE * 100;
    const NUM_SOLUTIONS_TO_KEEP: usize = SCALE * 50;
    const NUM_SOLUTIONS_TO_REPLACE: usize = SCALE * 40;

    for (jolt, wir) in joltages.iter().zip(wirings.iter()) {
        // Use differential evolution
        // Initialize N random solutions
        let min_vals: Vec<usize> = wir
            .iter()
            .map(|x| x.iter().map(|i| jolt[*i]).min().unwrap())
            .collect();
        //println!("{:?}", min_vals);
        let dimensions = wir.len();

        let num_solutions = NUM_SOLUTIONS * dimensions * dimensions;
        let num_solutions_to_keep = NUM_SOLUTIONS_TO_KEEP * dimensions * dimensions;
        let num_solitions_to_replace = NUM_SOLUTIONS_TO_REPLACE * dimensions * dimensions;

        let mut solutions = Vec::new();
        for _ in 0..num_solutions {
            let mut solution = Vec::new();
            for min_val in &min_vals {
                solution.push(rng.gen_range(0..=*min_val));
            }
            solutions.push((solution, usize::MAX));
        }

        //let mut iteration_count = 0;
        while solutions[num_solutions_to_keep].1 > 10000 {
            //iteration_count += 1;
            // if iteration_count % 100 == 0 {
            //     println!(
            //         "Iterations {} | Solution {} ",
            //         iteration_count, solutions[0].1
            //     );
            // }
            // Compute scores
            for (sol, cost) in solutions.iter_mut() {
                compute_cost(sol, cost, wir, jolt);
            }

            solutions.sort_by(|a, b| a.1.cmp(&b.1));

            // Keep the first 50%, replace the next 40% and randomize the last 10%
            for idx in num_solutions_to_keep..num_solutions_to_keep + num_solitions_to_replace {
                let a_idx = rng.gen_range(0..num_solutions_to_keep);
                let b_idx = rng.gen_range(0..num_solutions_to_keep);
                let c_idx = rng.gen_range(0..num_solutions_to_keep);

                for d in 0..dimensions {
                    let x = solutions[a_idx].0[d] + solutions[b_idx].0[d];
                    let y = solutions[c_idx].0[d];
                    if y > x {
                        solutions[idx].0[d] = 0;
                    } else {
                        solutions[idx].0[d] = (x - y).min(min_vals[d]);
                    }
                }
            }

            for idx in num_solutions_to_keep + num_solitions_to_replace..num_solutions {
                for d in 0..dimensions {
                    solutions[idx].0[d] = rng.gen_range(0..=min_vals[d])
                }
            }
        }

        // Compute scores
        for (sol, cost) in solutions.iter_mut() {
            compute_cost(sol, cost, wir, jolt);
        }

        solutions.sort_by(|a, b| a.1.cmp(&b.1));
        let x = solutions[0].1;
        // println!(
        //     "{:?} | Cost {} => Steps {} ",
        //     solutions[0].0, solutions[0].1, x
        // );
        ans2 += x;
    }
    ans2
}

fn solve1(input: &str) -> usize {
    let mut ans = 0;
    let mut patterns = Vec::new();
    let mut wirings = Vec::new();
    let mut joltages = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let tokens = io::tokenize(line, " ");
        let mut wiring_single = Vec::new();

        for token in tokens {
            if token.contains("[") {
                patterns.push(token[1..token.len() - 1].to_string());
            } else if token.contains("(") {
                let wiring = io::tokenize_nums::<usize>(&token[1..token.len() - 1], ",");
                //dbg!(&wiring);
                wiring_single.push(wiring);
            } else {
                joltages.push(token);
            }
        }

        wirings.push(wiring_single);
    }

    // println!("{:?}", patterns);
    // println!("{:?}", wirings);

    for (pat, wir) in patterns.iter().zip(wirings.iter()) {
        let mut map: HashMap<Vec<bool>, usize> = HashMap::new();
        let mut q: VecDeque<(Vec<bool>, usize)> = VecDeque::new();
        let s1: Vec<bool> = (0..pat.len()).map(|_| false).collect();
        let s2: Vec<bool> = pat
            .chars()
            .map(|c| if c == '#' { true } else { false })
            .collect();
        q.push_back((s1, 0));
        while !q.is_empty() {
            let (s, n) = q.pop_front().unwrap();

            if let Some(x) = map.get_mut(&s) {
                if *x > n {
                    *x = n;
                } else {
                    continue;
                }
            }

            map.insert(s.clone(), n);

            if s == s2 {
                //println!("Final state {:?} reached in {} steps", &s, n);
                continue;
            }

            for w in wir.iter() {
                let mut ss = s.clone();
                for switch in w.iter() {
                    ss[*switch] = !s[*switch];
                }
                q.push_back((ss, n + 1));
            }
        }

        ans += map.get(&s2).unwrap();
    }

    ans
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, solve1, true);
        common::timed(&input, solve2, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(solve1(sample_input), 7);
        assert_eq!(solve2(sample_input), 33);
    }
}
