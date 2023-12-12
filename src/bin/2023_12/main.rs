use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use aoc::io;
use itertools::Itertools;

const INPUT: [(&str, &str); 1] = [
    ("Sample Input", include_str!("sample_input.txt")),
    //("Input", include_str!("input.txt")),
];

fn get_all(arr: Vec<char>, n: usize) -> Vec<Vec<char>> {
    //println!("{:?}", arr);

    let cq = arr.iter().filter(|&x| *x == '?').count();
    let ch = arr.iter().filter(|&x| *x == '#').count();

    if cq + ch < n {
        return vec![];
    }

    if cq == 0 && ch == n {
        return vec![arr];
    }

    let mut ret = Vec::new();

    for idx in 0..arr.len() {
        if arr[idx] == '?' {
            let mut arr1 = arr.clone();
            arr1[idx] = '#';

            ret.extend(get_all(arr1.clone(), n));
            arr1[idx] = '.';
            ret.extend(get_all(arr1, n));
            break;
        }
    }

    ret
}

fn check(arr: &[char], nums: &[usize]) -> bool {
    if arr.iter().filter(|&c| *c == '#').count() != nums.iter().sum::<usize>() {
        return false;
    }

    let mut num_hashes = 0;
    let mut hash_vec = Vec::new();
    for i in 0..arr.len() {
        if arr[i] == '#' {
            num_hashes += 1;
        } else if arr[i] == '.' {
            if num_hashes > 0 {
                hash_vec.push(num_hashes);
                num_hashes = 0;
            }
        }
    }

    if num_hashes > 0 {
        hash_vec.push(num_hashes);
    }

    //println!("{:?} => {:?}. Actual {:?}", arr, hash_vec, nums);

    hash_vec == nums
}

fn expand(arr: &Vec<char>, nums: &Vec<usize>) -> (Vec<char>, Vec<usize>) {
    let mut arr1 = arr.clone();
    arr1.push('?');
    arr1.extend(arr.clone());
    arr1.push('?');
    arr1.extend(arr.clone());
    arr1.push('?');
    arr1.extend(arr.clone());
    arr1.push('?');
    arr1.extend(arr.clone());

    let mut nums1 = nums.clone();
    nums1.extend(nums.clone());
    nums1.extend(nums.clone());
    nums1.extend(nums.clone());
    nums1.extend(nums.clone());

    (arr1, nums1)
}

fn cnt(s: &[char], ch: char) -> usize {
    s.iter().filter(|&c| *c == ch).count()
}

fn append_front(s: &[char]) -> Vec<char> {
    let mut ret: Vec<char> = Vec::new();
    ret.push('?');
    ret.extend(s);
    ret
}

fn append_back(s: &[char]) -> Vec<char> {
    let mut ret: Vec<char> = Vec::new();
    ret.extend(s);
    ret.push('?');
    ret
}

fn dp_solve(s: Vec<char>, mut n: Vec<usize>, mut hashes: usize) -> usize {
    //println!("Calling DP solve on {:?} with {:?} and hashes {}", s, n, hashes);

    if n.is_empty() {
        //println!("FOUND MATCH!");
        return 1;
    }

    if s.is_empty() && n[0] == hashes {
        return 1;
    }

    if s.is_empty() {
        return 0;
    }

    if s[0] == '#' {

        if n[0] == hashes {
            return 0;
        }

        return dp_solve(s[1..].to_vec(), n, hashes + 1);
    } else if s[0] == '.' {

        if n[0] == hashes {
            n.remove(0);
        }

        return dp_solve(s[1..].to_vec(), n, 0);
    } else if s[0] == '?' {

        if n[0] == hashes {
            n.remove(0);
            return dp_solve(s[1..].to_vec(), n, 0);
        } else if hashes == 0 {
            let dph = dp_solve(s[1..].to_vec(), n.clone(), 1);
            let dpd = dp_solve(s[1..].to_vec(), n, 0);
            return dpd + dph;
        } else {
            return dp_solve(s[1..].to_vec(), n.clone(), hashes + 1);
        }   
    }

    0
}

fn solve(input: &str) {
    let mut ans: usize = 0;

    for line in input.split('\n') {
        let (arr, nums) = line.split_once(' ').unwrap();
        let arr = arr.chars().collect::<Vec<_>>();
        let nums = io::tokenize(nums, ",")
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        // let l = arr.len();
        // let n = nums.len();

        // // dot spaces
        // let m = l - ns;

        // // undetermined spaces
        // let f = m + 1 - n;

        // Expand

        //let (arr, nums) = expand(&arr, &nums);

        let ns = nums.iter().sum::<usize>();

        //println!("{:?} {:?} {}", arr, nums, ns);

        let all = get_all(arr.clone(), ns);

        let mut good: Vec<Vec<char>> = Vec::new();
        for a in &all {
            if check(a, &nums) {
                good.push(a.clone());
            }
        }

        let arr_new = (0..arr.len())
            .map(|idx| {
                if arr[idx] == '?' {
                    let cd = good.iter().filter(|g| g[idx] == '.').count();
                    let ch = good.iter().filter(|g| g[idx] == '#').count();

                    if cd == 0 && ch == good.len() {
                        '#'
                    } else if cd == good.len() && ch == 0 {
                        '.'
                    } else {
                        '?'
                    }
                } else {
                    arr[idx]
                }
            })
            .collect::<Vec<_>>();

        let dp_sol = dp_solve(arr.clone(), nums.clone(), 0);

        println!(
            "{:?} | {:?} | {} combos to check | Good {} | DP {}\n{:?}\n",
            arr,
            nums,
            all.len(),
            good.len(),
            dp_sol,
            arr_new
        );
        ans += good.len();
    }

    println!("Answer : {}", ans);
}

fn main() {
    for (file, input) in INPUT {
        println!("{}", file);
        solve(input);
    }
}
