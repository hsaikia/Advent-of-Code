use std::{env, time::Instant};

use aoc::io;

fn hash(s: &str) -> u32 {
    s.chars()
        .fold(0, |acc, c| ((acc + (c as u8) as u32) * 17) % 256)
}

fn part1(input: &str) {
    let ans = input.split(',').into_iter().map(|s| hash(s)).sum::<u32>();
    println!("Answer Part1: {}", ans);
}

fn part2(input: &str) {
    const VAL: Vec<(&str, u32)> = Vec::new();
    let mut map: [Vec<(&str, u32)>; 256] = [VAL; 256];

    let seqs = io::tokenize(input, ",");
    for seq in &seqs {
        if seq.find('=').is_some() {
            if let Some((id, val)) = seq.split_once('=') {
                let val: u32 = io::parse_num(val).unwrap();
                let box_idx = hash(id) as usize;
                let mut found = false;
                for (id1, val1) in &mut map[box_idx] {
                    if id == *id1 {
                        *val1 = val;
                        found = true;
                        break;
                    }
                }
                if !found {
                    map[box_idx].push((id, val));
                }
            }
        } else {
            if let Some((id, _)) = seq.split_once('-') {
                let box_idx = hash(id) as usize;
                map[box_idx].retain(|(id1, _)| *id1 != id);
            }
        }
    }

    let ans = (0..256)
        .map(|idx| {
            map[idx]
                .iter()
                .enumerate()
                .map(|(slot, (_, val))| (idx as u32 + 1) * (slot as u32 + 1) * val)
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("Answer Part2 : {}", ans);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let input = std::fs::read_to_string(filepath).unwrap();

    let start = Instant::now();
    part1(&input);
    let duration = start.elapsed();
    println!("Time elapsed in Part 1 is: {:?}", duration);

    let start = Instant::now();
    part2(&input);
    let duration = start.elapsed();
    println!("Time elapsed in Part 2 is: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let sample_input = "HASH";

        assert_eq!(hash(sample_input), 52);
        assert_eq!(hash("rn"), 0);
        assert_eq!(hash("qp"), 1);
        assert_eq!(hash("cm"), 0);
    }
}
