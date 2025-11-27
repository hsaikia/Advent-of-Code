use std::collections::HashMap;

use aoc::{common, io};

fn reduce(input: &str, mul: bool) -> String {
    let mut num: usize = io::parse_num(input);
    if mul {
        num *= 2024;
    }
    num.to_string()
}

fn evolve(input: &str) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    if input == "0" {
        ret.push("1".to_string());
    } else if input.len() % 2 == 0 {
        ret.push(reduce(&input[0..input.len() / 2], false));
        ret.push(reduce(&input[input.len() / 2..], false));
    } else {
        ret.push(reduce(input, true));
    }
    ret
}

fn num_stones(stone: &str, times: usize, cache: &mut HashMap<(String, usize), usize>) -> usize {
    if times == 0 {
        return 1;
    }
    if let Some(v) = cache.get(&(stone.to_string(), times)) {
        return *v;
    }
    let mut ans = 0;
    let next = evolve(stone);
    for ns in &next {
        ans += num_stones(ns, times - 1, cache);
    }
    cache.insert((stone.to_string(), times), ans);
    ans
}

fn solve<const ITERATIONS: usize>(input: &str) -> usize {
    let mut ans = 0;
    let stones: Vec<String> = io::tokenize(input, " ")
        .iter()
        .map(std::string::ToString::to_string)
        .collect();
    let mut cache: HashMap<(String, usize), usize> = HashMap::new();
    for stone in &stones {
        ans += num_stones(stone, ITERATIONS, &mut cache);
    }
    ans
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, solve::<25>, true);
        common::timed(&input, solve::<75>, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "125 17";
        assert_eq!(solve::<25>(sample_input), 55312);
        assert_eq!(solve::<75>(sample_input), 65601038650482);
    }
}
