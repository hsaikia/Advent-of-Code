use std::collections::HashMap;

use aoc::{common, io};

fn path_count<'a>(
    from: &'a str,
    to: &'a str,
    map: &HashMap<&str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(x) = cache.get(from) {
        return *x;
    }

    let mut ans = 0;
    if let Some(nxts) = map.get(from) {
        for nxt in nxts {
            let x = if *nxt == to {
                1
            } else {
                path_count(nxt, to, map, cache)
            };

            ans += x;
        }
    }
    cache.insert(from, ans);
    ans
}

fn solve<const PART: usize>(input: &str) -> usize {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if let Some((input, outputs)) = line.split_once(": ") {
            let outputs = io::tokenize(outputs, " ");
            map.insert(input, outputs);
        }
    }

    let mut cache = HashMap::new();
    if PART == 1 {
        path_count("you", "out", &map, &mut cache)
    } else {
        // Assuming there are no cycles, there can only be a path from fft to dac or dac to fft
        let paths_fft_dac = path_count("fft", "dac", &map, &mut cache);
        cache.clear();
        let paths_dac_fft = path_count("dac", "fft", &map, &mut cache);

        if paths_dac_fft > 0 {
            let x1 = path_count("svr", "dac", &map, &mut cache);
            cache.clear();
            let x2 = path_count("fft", "out", &map, &mut cache);
            x1 * paths_dac_fft * x2
        } else {
            let x1 = path_count("svr", "fft", &map, &mut cache);
            cache.clear();
            let x2 = path_count("dac", "out", &map, &mut cache);
            x1 * paths_fft_dac * x2
        }
    }
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, solve::<1>, true);
        common::timed(&input, solve::<2>, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input1 = "aaa: you hhh\nyou: bbb ccc\nbbb: ddd eee\nccc: ddd eee fff\nddd: ggg\neee: out\nfff: out\nggg: out\nhhh: ccc fff iii\niii: out";
        let sample_input2 = "svr: aaa bbb\n\naaa: fft\nfft: ccc\nbbb: tty\ntty: ccc\nccc: ddd eee\nddd: hub\nhub: fff\neee: dac\ndac: fff\nfff: ggg hhh\nggg: out\nhhh: out";
        assert_eq!(solve::<1>(sample_input1), 5);
        assert_eq!(solve::<2>(sample_input2), 2);
    }
}
