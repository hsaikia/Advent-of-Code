use std::{collections::HashMap, ops::BitXor};

use aoc::{common, io};

fn mix(secret: usize, b: usize) -> usize {
    secret.bitxor(b)
}

fn prune(secret: usize) -> usize {
    secret % 16777216
}

fn generate(n: usize, times: usize) -> usize {
    if times == 0 {
        return n;
    }

    let mut secret = n;
    let a = secret * 64;
    secret = mix(secret, a);
    secret = prune(secret);
    let b = secret / 32;
    secret = mix(secret, b);
    secret = prune(secret);
    let c = secret * 2048;
    secret = mix(secret, c);
    secret = prune(secret);
    generate(secret, times - 1)
}

fn generate_lst(lst: &mut Vec<usize>, times: usize) {
    if times == 0 {
        return;
    }

    let mut secret = *lst.last().unwrap();
    let n2 = secret * 64;
    secret = mix(secret, n2);
    secret = prune(secret);
    let n4 = secret / 32;
    secret = mix(secret, n4);
    secret = prune(secret);
    let n6 = secret * 2048;
    secret = mix(secret, n6);
    secret = prune(secret);
    lst.push(secret);
    generate_lst(lst, times - 1)
}

fn solve<const PART: usize>(input: &str) -> usize {
    let mut ans = 0;
    let mut map: HashMap<(i64, i64, i64, i64, usize), usize> = HashMap::new();
    for (i, ns) in input.lines().enumerate() {
        let number: usize = io::parse_num(ns);

        if PART == 1 {
            ans += generate(number, 2000);
        } else {
            let mut lst = vec![number];
            generate_lst(&mut lst, 2000);
            let lst_dig: Vec<usize> = lst
                .iter()
                .map(|n| n.to_string().chars().last().unwrap().to_digit(10).unwrap() as usize)
                .collect();
            let diff: Vec<(i64, usize)> = lst_dig
                .iter()
                .zip(lst_dig.iter().skip(1))
                .map(|(a, b)| (*b as i64 - *a as i64, *b))
                .collect();

            for w in diff.windows(4) {
                let key = (w[0].0, w[1].0, w[2].0, w[3].0, i);
                map.entry(key).or_insert(w[3].1);
            }
        }
    }

    if PART == 1 {
        return ans;
    }
    let mut total_map: HashMap<(i64, i64, i64, i64), usize> = HashMap::new();
    for ((k1, k2, k3, k4, _), v) in map.iter() {
        if let Some(b) = total_map.get_mut(&(*k1, *k2, *k3, *k4)) {
            *b += *v;
        } else {
            total_map.insert((*k1, *k2, *k3, *k4), *v);
        }
    }

    *total_map.values().max().unwrap()
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
        let sample_input1 = "1\n10\n100\n2024";
        assert_eq!(solve::<1>(sample_input1), 37327623);
        let sample_input2 = "1\n2\n3\n2024";
        assert_eq!(solve::<2>(sample_input2), 23);
    }
}
