use aoc::common;
use aoc::io;

fn good_difference(seq: &[u32]) -> bool {
    let diff_seq: Vec<i64> = seq
        .windows(2)
        .map(|w| i64::from(w[1]) - i64::from(w[0]))
        .collect();
    diff_seq.iter().all(|x| *x >= 1 && *x <= 3)
        || diff_seq.iter().all(|x| *x >= -3 && *x <= -1)
}

fn good_sequence<const PART: usize>(seq: &[u32]) -> bool {
    if good_difference(seq) {
        return true;
    } else if PART == 2 {
        return (0..seq.len()).any(|drop_idx| {
            let mut seq_new = seq.to_vec();
            seq_new.remove(drop_idx);
            good_difference(&seq_new)
        });
    }
    false
}

fn solve<const PART: usize>(input: &str) -> usize {
    input
        .lines()
        .map(|l| io::tokenize_nums(l, " "))
        .filter(|v| good_sequence::<PART>(v))
        .count()
}

fn main() {
    let input = common::get_input();
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
