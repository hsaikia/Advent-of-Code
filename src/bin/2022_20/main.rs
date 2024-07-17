use aoc::common;
use std::cmp::Ordering;

const DECRYPTION_KEY: i64 = 811589153;

fn sum_grove(numbers: &[i64], order: &[usize], key: i64) -> i64 {
    let mut idx_0 = numbers.iter().position(|&n| n == 0).unwrap();
    let mut ans = 0;
    for i in 1..=3000 {
        idx_0 = order[idx_0];

        if i % 1000 == 0 {
            //println!("Number at idx {} is {}", i, key * numbers[idx_0]);
            ans += numbers[idx_0] * key;
        }
    }
    ans
}

fn mix(numbers: &[i64], key: i64, times: usize) -> i64 {
    let l = numbers.len();
    let mut ptrs = get_forward_backward_ptrs(numbers.len());

    for _ in 0..times {
        for (i, num) in numbers.iter().enumerate() {
            let l1: i64 = l as i64 - 1;
            let q = (key as f64 * num.abs() as f64 / l1 as f64).floor();

            let mut steps = key * num;

            match steps.cmp(&0) {
                Ordering::Greater => steps -= q as i64 * l1,
                Ordering::Less => steps += (q as i64 + 1) * l1,
                Ordering::Equal => (),
            }

            for _ in 0..steps {
                // Z <-> (A) <-> B <-> C => Z <-> B <-> (A) <-> C
                let back_idx = ptrs.1[i];
                let front_1_idx = ptrs.0[i];
                let front_2_idx = ptrs.0[front_1_idx];

                ptrs.0[back_idx] = front_1_idx;

                ptrs.1[front_1_idx] = back_idx;
                ptrs.0[front_1_idx] = i;

                ptrs.0[i] = front_2_idx;
                ptrs.1[i] = front_1_idx;

                ptrs.1[front_2_idx] = i;
            }
        }
    }

    sum_grove(numbers, &ptrs.0, key)
}

fn get_forward_backward_ptrs(l: usize) -> (Vec<usize>, Vec<usize>) {
    let mut forward_ptrs = Vec::new();
    let mut backward_ptrs = Vec::new();

    for i in 0..l {
        forward_ptrs.push((i + 1) % l);
        backward_ptrs.push((i + l - 1) % l);
    }

    (forward_ptrs, backward_ptrs)
}

fn process_and_mix<const PART1: bool>(input: &str) -> i64 {
    let mut numbers = Vec::new();
    for line in input.split('\n') {
        numbers.push(line.parse::<i64>().unwrap());
    }

    if PART1 {
        return mix(&numbers, 1, 1);
    }

    mix(&numbers, DECRYPTION_KEY, 10)
}

fn main() {
    let input = common::get_input();
    common::timed(&input, process_and_mix::<true>, true);
    common::timed(&input, process_and_mix::<false>, false);
}
