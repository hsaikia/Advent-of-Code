use std::collections::HashMap;

use aoc::common;

fn solve2(input: &str) -> usize {
    let mut ans = 0;
    let l = input.len();
    let mut id_spaces_map : HashMap<usize, usize> = HashMap::new();
    for c in input.chars().into_iter() {
        let x = c.to_digit(10).unwrap() as usize;
        let id = id_spaces_map.len();
        id_spaces_map.insert(id, x);
    }

    loop {
        let mut found = false;
        // even is file and odd is free space
        for odd in (0..l).filter(|x| x % 2 == 1) {
            for even in  (0..l).filter(|x| x % 2 == 0) {
                let free = *id_spaces_map.get(&odd).unwrap();
                let file = *id_spaces_map.get(&even).unwrap();
                if free >= file {
                    id_spaces_map.entry(odd).and_modify(|v| *v -= file);
                    id_spaces_map.entry(even).and_modify(|v| *v -= file);
                    found = true;
                    break;
                }
            }

            if found {
                break;
            }
        }

        if !found {
            break;
        }
    }

    0
}

fn solve<const PART: i32>(input: &str) -> u64 {
    let mut ans = 0;
    //let mut dk : [u64; 100000] = []
    let mut id_spaces_map : HashMap<u64, u64> = HashMap::new();
    let mut free_spaces : Vec<u64> = Vec::new();
    let mut idx = 0;
    for (i, c) in input.chars().into_iter().enumerate() {
        let x = c.to_digit(10).unwrap() as u64;
        if i % 2 == 0 {
            let id = id_spaces_map.len() as u64;
            id_spaces_map.insert(id, x);
        } else {
            for i in idx..idx + x {
                free_spaces.push(i);
            }
        }
        idx += x;
    }
    // println!("{id_spaces_map:?}");

    // let mut max_id = *id_spaces_map.iter().filter(|(_, v)| *v > &0).map(|(k, _)| k).max().unwrap();

    // println!("Max key {max_id} total space {idx}");

    let mut arr : [u64; 95000] = [0; 95000];
    let mut id = 0;
    let mut idx : u64 = 0;

    for (i, c) in input.chars().into_iter().enumerate() {

        // if id_spaces_map.iter().filter(|(k, v)| *k >= &id && *v > &0).count() == 0 {
        //     break;
        // }

        let x = c.to_digit(10).unwrap() as u64;
        //let mut done = false;
        if i % 2 == 0 {
            for j in idx..idx + x {
                if id_spaces_map.iter().filter(|(k, v)| *k >= &id && *v > &0).count() == 0 {
                    break;
                }
                //println!("Id {id} Idx {j}");
                ans += id * j;
                id_spaces_map.entry(id).and_modify(|v| *v -= 1);
            }
            id += 1;
        } else {
            for j in idx..idx + x {
                if let Some(max_id) = id_spaces_map.iter().filter(|(_, v)| *v > &0).map(|(k, _)| k).max() {
                    ans += *max_id * j;
                    //println!("Id {max_id} Idx {j}");
                    id_spaces_map.entry(*max_id).and_modify(|v| *v -= 1);
                } else {
                    //done = true;
                    break;
                }
                //println!("{id_spaces_map:?}");
            }
        }
        idx += x;

        // if done {
        //     break;
        // }
    }

    //max_key(&mut id_spaces_map);

    //println!("{id_spaces_map:?}");
    //println!("{free_spaces:?}");

    // let mut ptr1 = 0;
    // let mut ptr2 = 
    // for (i, c) in input.chars().into_iter().enumerate()  {
    //     let x = c.to_digit(10).unwrap();
    //     if i % 2 == 1 {
    //         free_space += x;
    //     }
    // }    

    ans
}

fn main() {
    let input = common::get_input();
    //println!("{input:?}");
    common::timed(&input, solve::<1>, true);
    //common::timed(&input, solve::<2>, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............";
        assert_eq!(solve::<1>(sample_input), 14);
        assert_eq!(solve::<2>(sample_input), 34);
    }
}
