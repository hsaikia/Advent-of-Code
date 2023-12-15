use std::{env, time::Instant};

use aoc::io;

fn hash(s : &str) -> u32 {
    s.chars().fold(0, |acc, c| ((acc + (c as u8) as u32) * 17) % 256)
}

fn solve(input: &str) {
    let mut ans: u32 = 0;

    let seqs = io::tokenize(input, ",");
    for seq in &seqs {
        ans += hash(seq);
    }

    println!("Answer : {}", ans);
}

fn solve2(input: &str) {
    let mut ans: usize = 0;

    const VAL : Vec<(&str, u32)> = Vec::new();
    let mut map : [Vec<(&str, u32)>; 256] = [VAL; 256];

    let seqs = io::tokenize(input, ",");
    for seq in &seqs {
        if seq.find('=').is_some() {
            if let Some((id, val)) = seq.split_once('=') {
                let val : u32 = io::parse_num(val).unwrap();
                let idx = hash(id) as usize;
                let mut found = false;
            for (id1, val1) in &mut map[idx] {
                if id == *id1 {
                    *val1 = val;
                    found = true;
                    break;
                }
            }
            if !found {
                map[idx].push((id, val));
            }
            }
            
        } else {
            if let Some((id, _)) = seq.split_once('-') {
                let idx = hash(id) as usize;
                let mut index = None;
            for (i, (id1, _)) in map[idx].iter().enumerate() {
                if id == *id1 {
                    index = Some(i);
                    break;
                }
            }

            if let Some(i) = index {
                map[idx].remove(i);
                
            }
            }
            
        }
    }

    //dbg!(&map);

    for idx in 0..256 {
        for (slot, bx) in map[idx].iter().enumerate() {
            ans += (idx + 1) * (slot + 1) * bx.1 as usize;
        }
        
    }

    println!("Answer : {}", ans);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let input = std::fs::read_to_string(filepath).unwrap();
    
    let start = Instant::now();
    solve(&input);
    let duration = start.elapsed();
    println!("Time elapsed in Part 1 is: {:?}", duration);   

    let start = Instant::now();
    solve2(&input);
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
