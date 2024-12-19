use std::collections::HashMap;

use aoc::{common, io};

fn possible_ways(patterns: &[&str], towel: &str, map: &mut HashMap<String, usize>) -> usize {
    if towel.is_empty() {
        return 1;
    }
    if map.get(towel).is_some() {
        return *map.get(towel).unwrap();
    }
    let mut ways = 0;
    for p in patterns.iter() {
        if p.len() > towel.len() {
            continue;
        }
        if **p == towel[0..p.len()] {
            ways += possible_ways(patterns, &towel[p.len()..], map);
        }
    }
    map.insert(towel.to_string(), ways);
    ways
}

fn solve<const PART: usize>(input: &str) -> usize {
    let mut ans = 0;
    let batches = io::line_batches(input);
    let patterns = io::tokenize(batches[0][0], ", ");

    let mut cache: HashMap<String, usize> = HashMap::new();

    for towel in batches[1].iter() {
        //println!("Checking {}", towel);
        if PART == 1 {
            ans += if possible_ways(&patterns, towel, &mut cache) > 0 {
                1
            } else {
                0
            };
        }
        if PART == 2 {
            ans += possible_ways(&patterns, towel, &mut cache);
        }
    }
    ans
}

fn main() {
    let input = common::get_input();
    //println!("{input:?}");
    common::timed(&input, solve::<1>, true);
    common::timed(&input, solve::<2>, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input =
            "r, wr, b, g, bwu, rb, gb, br\n\nbrwrr\nbggr\ngbbr\nrrbgbr\nubwu\nbwurrg\nbrgr\nbbrgwb";
        assert_eq!(solve::<1>(sample_input), 6);
        assert_eq!(solve::<2>(sample_input), 16);
    }
}
