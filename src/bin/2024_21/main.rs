use std::collections::{HashMap, VecDeque};

use aoc::{common, io};

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+


     +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/

// Commands : up(0), right(1), down(2) left(3), enter(4)
const CHARS: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'];
const KEYPAD_MAP1: [[Option<usize>; 4]; 11] = [
    [Some(2), Some(10), None, None],      // 0
    [Some(4), Some(2), None, None],       // 1
    [Some(5), Some(3), Some(0), Some(1)], // 2
    [Some(6), None, Some(10), Some(2)],   // 3
    [Some(7), Some(5), Some(1), None],    // 4
    [Some(8), Some(6), Some(2), Some(4)], // 5
    [Some(9), None, Some(3), Some(5)],    // 6
    [None, Some(8), Some(4), None],       // 7
    [None, Some(9), Some(5), Some(7)],    // 8
    [None, None, Some(6), Some(8)],       // 9
    [Some(3), None, None, Some(0)],       // A
];

// positions are indexes in the KEYPAD_MAP1, so 0..11
// returns should be indexes in KEYPAD_MAP2
fn sequences<const MAP_SIZE: usize>(
    position: usize,
    input: &[usize],
    map: &[[Option<usize>; 4]; MAP_SIZE],
) -> Vec<Vec<usize>> {
    //println!("{} {:?}", position, input);
    let mut ans: Vec<Vec<usize>> = vec![];

    let mut q = VecDeque::new();
    // position in keypad and index in input array
    q.push_back((vec![], position, 0));

    let mut visited = HashMap::new();

    while let Some((key_presses, pos, idx)) = q.pop_front() {
        if let Some(v) = visited.get_mut(&(pos, idx)) {
            if *v < key_presses.len() {
                continue;
            }
            *v = key_presses.len();
        } else {
            visited.insert((pos, idx), key_presses.len());
        }

        if idx >= input.len() {
            ans.push(key_presses);
            continue;
        }

        if pos == input[idx] {
            let mut tmp = key_presses;
            tmp.push(4);
            q.push_back((tmp, pos, idx + 1));
            continue;
        }

        for (i, opt) in map[pos].iter().enumerate() {
            if let Some(x) = opt {
                let mut tmp = key_presses.clone();
                tmp.push(i);
                q.push_back((tmp, *x, idx));
            }
        }
    }
    ans
}

fn seq_len(
    start: usize,
    next: usize,
    hsh: &HashMap<(usize, usize), Vec<Vec<usize>>>,
    ans_hsh: &mut HashMap<(usize, usize, usize), usize>,
    iterations: usize,
) -> usize {
    let mut ans = 0;

    if let Some(res) = ans_hsh.get(&(start, next, iterations)) {
        return *res;
    }

    if let Some(next) = hsh.get(&(start, next)) {
        let tmp = next[0].clone();

        if iterations == 0 {
            return tmp.len();
        }

        let mut start = 4;

        for idx in &tmp {
            ans += seq_len(start, *idx, hsh, ans_hsh, iterations - 1);
            start = *idx;
        }
    }
    ans_hsh.insert((start, next, iterations), ans);
    ans
}

fn solve<const PART: usize, const ITERATIONS: usize>(input: &str) -> usize {
    let mut hsh = HashMap::new();

    // position, to_print map -> possibilities
    hsh.insert((0, 0), vec![vec![4]]);
    hsh.insert((0, 1), vec![vec![2, 1, 4], vec![1, 2, 4]]);
    hsh.insert((0, 2), vec![vec![2, 4]]);
    hsh.insert((0, 3), vec![vec![2, 3, 4]]);
    hsh.insert((0, 4), vec![vec![1, 4]]);

    hsh.insert((1, 0), vec![vec![3, 0, 4], vec![0, 3, 4]]);
    hsh.insert((1, 1), vec![vec![4]]);
    hsh.insert((1, 2), vec![vec![3, 4]]);
    hsh.insert((1, 3), vec![vec![3, 3, 4]]);
    hsh.insert((1, 4), vec![vec![0, 4]]);

    hsh.insert((2, 0), vec![vec![0, 4]]);
    hsh.insert((2, 1), vec![vec![1, 4]]);
    hsh.insert((2, 2), vec![vec![4]]);
    hsh.insert((2, 3), vec![vec![3, 4]]);
    hsh.insert((2, 4), vec![vec![0, 1, 4], vec![1, 0, 4]]);

    hsh.insert((3, 0), vec![vec![1, 0, 4]]);
    hsh.insert((3, 1), vec![vec![1, 1, 4]]);
    hsh.insert((3, 2), vec![vec![1, 4]]);
    hsh.insert((3, 3), vec![vec![4]]);
    hsh.insert((3, 4), vec![vec![1, 1, 0, 4], vec![1, 0, 1, 4]]);

    hsh.insert((4, 0), vec![vec![3, 4]]);
    hsh.insert((4, 1), vec![vec![2, 4]]);
    hsh.insert((4, 2), vec![vec![3, 2, 4], vec![2, 3, 4]]);
    //hsh.insert((4, 3), vec![vec![3, 2, 3, 4], vec![2, 3, 3, 4]]);
    // Using only the second option as the first one in general leads to more < commands which are harder to reach
    hsh.insert((4, 3), vec![vec![2, 3, 3, 4]]);
    hsh.insert((4, 4), vec![vec![4]]);

    let mut ans = 0;
    let mut char_map = HashMap::new();

    for (i, ch) in CHARS.iter().enumerate() {
        char_map.insert(ch, i);
    }

    let mut ans_hsh: HashMap<(usize, usize, usize), usize> = HashMap::new();

    for cmd in input.lines() {
        let number: usize = io::parse_num(cmd);
        let mut min_l = usize::MAX;
        let cmd_c: Vec<usize> = cmd.chars().map(|ch| *char_map.get(&ch).unwrap()).collect();
        let seq = sequences(10, &cmd_c, &KEYPAD_MAP1);
        for s in seq {
            if s.is_empty() {
                continue;
            }

            let mut res = 0;
            let mut start = 4;
            for idx in &s {
                res += seq_len(start, *idx, &hsh, &mut ans_hsh, ITERATIONS - 1);
                start = *idx;
            }
            min_l = res.min(min_l);
        }
        ans += number * min_l;
    }
    ans
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, solve::<1, 2>, true);
        common::timed(&input, solve::<2, 25>, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "029A\n980A\n179A\n456A\n379A";
        assert_eq!(solve::<1, 2>(sample_input), 126384);
        assert_eq!(solve::<2, 25>(sample_input), 154115708116294);
    }
}
