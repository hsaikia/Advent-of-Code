use std::{collections::HashMap, hash::Hash};

use aoc::io;

const INPUT: [(&str, &str); 2] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

fn expand(arr: &[char], nums: &[usize]) -> (Vec<char>, Vec<usize>) {
    let mut arr1 = arr.to_owned();
    arr1.push('?');
    arr1.extend(arr.to_owned());
    arr1.push('?');
    arr1.extend(arr.to_owned());
    arr1.push('?');
    arr1.extend(arr.to_owned());
    arr1.push('?');
    arr1.extend(arr.to_owned());

    let mut nums1 = nums.to_owned();
    nums1.extend(nums.to_owned());
    nums1.extend(nums.to_owned());
    nums1.extend(nums.to_owned());
    nums1.extend(nums.to_owned());

    (arr1, nums1)
}

fn cnt(s: &[char], ch: char) -> usize {
    s.iter().filter(|&c| *c == ch).count()
}

#[derive(Hash, PartialEq, Eq)]
struct Id {
    sl: usize,
    nl: usize,
    hs: usize,
}

fn solve(s: Vec<char>, mut n: Vec<usize>, hashes: usize, mp: &mut HashMap<Id, usize>) -> usize {
    let id = Id {
        sl: s.len(),
        nl: n.len(),
        hs: hashes,
    };

    if let Some(x) = mp.get(&id) {
        return *x;
    }

    if n.is_empty() {
        if cnt(&s, '#') == 0 {
            mp.insert(id, 1);
            return 1;
        } else {
            mp.insert(id, 0);
            return 0;
        }
    }

    if s.is_empty() && n.len() == 1 && n[0] == hashes {
        mp.insert(id, 1);
        return 1;
    }

    if s.is_empty() {
        mp.insert(id, 0);
        return 0;
    }

    if s[0] == '#' {
        if n[0] == hashes {
            mp.insert(id, 0);
            return 0;
        }

        let ret = solve(s[1..].to_vec(), n.clone(), hashes + 1, mp);
        mp.insert(id, ret);
        return ret;
    } else if s[0] == '.' {
        if n[0] == hashes {
            n.remove(0);
            let ret = solve(s[1..].to_vec(), n.clone(), 0, mp);
            mp.insert(id, ret);
            return ret;
        } else if hashes > 0 && hashes < n[0] {
            mp.insert(id, 0);
            return 0;
        } else {
            let ret = solve(s[1..].to_vec(), n.clone(), 0, mp);
            mp.insert(id, ret);
            return ret;
        }
    } else if s[0] == '?' {
        if n[0] == hashes {
            n.remove(0);
            let ret = solve(s[1..].to_vec(), n.clone(), 0, mp);
            mp.insert(id, ret);
            return ret;
        } else if hashes == 0 {
            let dph = solve(s[1..].to_vec(), n.clone(), 1, mp);
            let dpd = solve(s[1..].to_vec(), n.clone(), 0, mp);
            mp.insert(id, dpd + dph);
            return dpd + dph;
        } else {
            let ret = solve(s[1..].to_vec(), n.clone(), hashes + 1, mp);
            mp.insert(id, ret);
            return ret;
        }
    }

    0
}

fn process(input: &str, part1: bool) {
    let mut ans: usize = 0;

    for line in input.split('\n') {
        //println!("{}", line);
        let (arr, nums) = line.split_once(' ').unwrap();
        let arr = arr.chars().collect::<Vec<_>>();
        let nums = io::tokenize(nums, ",")
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        // Expand
        if !part1 {
            let (arr, nums) = expand(&arr, &nums);
            let mut mp: HashMap<Id, usize> = HashMap::new();
            let dp_sol = solve(arr.clone(), nums.clone(), 0, &mut mp);
            ans += dp_sol;
        } else {
            let mut mp: HashMap<Id, usize> = HashMap::new();
            let dp_sol = solve(arr.clone(), nums.clone(), 0, &mut mp);
            ans += dp_sol;
        }
    }

    println!("Answer : {}", ans);
}

fn main() {
    for (file, input) in INPUT {
        println!("{}", file);
        process(input, true);
        process(input, false);
    }
}
