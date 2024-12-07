use aoc::common;
use aoc::io;

fn diff_vec(seq: &[usize]) -> Vec<i32> {
    let mut num_diffs: Vec<i32> = Vec::new();
    for i in 1..seq.len() {
        num_diffs.push(seq[i] as i32 - seq[i - 1] as i32);
    }
    num_diffs
}

fn good_difference(seq: &[usize]) -> bool {
    let diff_seq = diff_vec(seq);
    const ALLOWED_DIFF_POS: [i32; 3] = [1, 2, 3];
    const ALLOWED_DIFF_NEG: [i32; 3] = [-1, -2, -3];
    diff_seq.iter().all(|x| ALLOWED_DIFF_POS.contains(x))
        || diff_seq.iter().all(|x| ALLOWED_DIFF_NEG.contains(x))
}

fn good_sequence<const PART: usize>(seq: &[usize]) -> bool {
    if good_difference(seq) {
        return true;
    } else if PART == 2 {
        for drop_idx in 0..seq.len() {
            let mut seq_new = seq.to_vec();
            seq_new.remove(drop_idx);
            if good_difference(&seq_new) {
                return true;
            }
        }
    }
    false
}

fn solve<const PART: usize>(input: &str) -> usize {
    let mut ans = 0;
    for report in input.lines() {
        let sequence_str = io::tokenize(report, " ");
        let sequence = sequence_str
            .iter()
            .map(|x| io::parse_num(x))
            .collect::<Vec<usize>>();
        ans += if good_sequence::<PART>(&sequence) {
            1
        } else {
            0
        };
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
        let sample_input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        assert_eq!(solve::<1>(sample_input), 2);
        assert_eq!(solve::<2>(sample_input), 4);
    }
}
