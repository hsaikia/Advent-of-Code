use aoc::{common, io};
use std::{collections::HashMap, hash::Hash};

fn expand(arr: &[char], nums: &[usize]) -> (Vec<char>, Vec<usize>) {
    let mut arr5 = arr.to_owned();
    let mut nums5 = nums.to_owned();
    for _ in 0..4 {
        arr5.push('?');
        arr5.extend(arr.to_owned());
        nums5.extend(nums.to_owned());
    }
    (arr5, nums5)
}

fn cnt(s: &[char], ch: char) -> usize {
    s.iter().filter(|&c| *c == ch).count()
}

#[derive(Hash, PartialEq, Eq)]
struct Id {
    arr_len: usize,
    num_len: usize,
    hashes_seen_so_far: usize,
}

fn insert_in_cache_and_return(map: &mut HashMap<Id, usize>, id: Id, ret: usize) -> usize {
    map.insert(id, ret);
    ret
}

fn solve(s: &[char], mut n: Vec<usize>, hashes: usize, mp: &mut HashMap<Id, usize>) -> usize {
    let id = Id {
        arr_len: s.len(),
        num_len: n.len(),
        hashes_seen_so_far: hashes,
    };

    if let Some(x) = mp.get(&id) {
        return *x;
    }

    if n.is_empty() {
        if cnt(s, '#') == 0 {
            return insert_in_cache_and_return(mp, id, 1);
        }
        return insert_in_cache_and_return(mp, id, 0);
    }

    if s.is_empty() && n.len() == 1 && n[0] == hashes {
        return insert_in_cache_and_return(mp, id, 1);
    }

    if s.is_empty() {
        return insert_in_cache_and_return(mp, id, 0);
    }

    if s[0] == '#' {
        if n[0] == hashes {
            return insert_in_cache_and_return(mp, id, 0);
        }
        let ret = solve(&s[1..], n.clone(), hashes + 1, mp);
        return insert_in_cache_and_return(mp, id, ret);
    } else if s[0] == '.' {
        if n[0] == hashes {
            n.remove(0);
            let ret = solve(&s[1..], n.clone(), 0, mp);
            return insert_in_cache_and_return(mp, id, ret);
        } else if hashes > 0 && hashes < n[0] {
            return insert_in_cache_and_return(mp, id, 0);
        }
        let ret = solve(&s[1..], n.clone(), 0, mp);
        return insert_in_cache_and_return(mp, id, ret);
    } else if s[0] == '?' {
        if n[0] == hashes {
            n.remove(0);
            let ret = solve(&s[1..], n.clone(), 0, mp);
            return insert_in_cache_and_return(mp, id, ret);
        } else if hashes == 0 {
            let dp1 = solve(&s[1..], n.clone(), 1, mp);
            let dp0 = solve(&s[1..], n.clone(), 0, mp);
            return insert_in_cache_and_return(mp, id, dp1 + dp0);
        }
        let ret = solve(&s[1..], n.clone(), hashes + 1, mp);
        return insert_in_cache_and_return(mp, id, ret);
    }

    0
}

fn process(input: &str, part1: bool) -> usize {
    let mut ans: usize = 0;

    for line in input.split('\n') {
        let (arr, nums) = line.split_once(' ').unwrap();
        let arr = arr.chars().collect::<Vec<_>>();
        let nums = io::tokenize(nums, ",")
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        // Expand
        let (arr, nums) = if part1 {
            (arr, nums)
        } else {
            expand(&arr, &nums)
        };
        let mut mp: HashMap<Id, usize> = HashMap::new();
        let dp_sol = solve(&arr, nums.clone(), 0, &mut mp);
        ans += dp_sol;
    }

    ans
}

fn part1(input: &str) -> usize {
    process(input, true)
}

fn part2(input: &str) -> usize {
    process(input, false)
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    common::timed(&input, part2, false);
}
