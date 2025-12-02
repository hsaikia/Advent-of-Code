use aoc::common;

fn solve<const PART: usize>(input: &str) -> i32 {
    let mut dial = 50;
    let mut num_zeros = 0;
    for line in input.split("\n") {
        //let old_zeros = num_zeros;
        let mut steps = line[1..].parse::<i32>().unwrap();
        if line.starts_with("L") {
            if PART == 2 {
                num_zeros += steps / 100;
                steps = steps % 100;
                if dial > 0 && dial < steps {
                    num_zeros += 1;
                }
            }
            dial = (dial - steps + 100) % 100;
        } else if line.starts_with("R") {
            if PART == 2 {
                num_zeros += steps / 100;
                steps = steps % 100;
                if dial + steps > 100 {
                    num_zeros += 1;
                }
            }
            dial = (dial + steps) % 100;
        }
        if dial == 0 {
            num_zeros += 1;
        }
        //dbg!(" {} {} {}", line, dial, num_zeros - old_zeros);
    }
    num_zeros
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
        let sample_input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(solve::<1>(sample_input), 3);
        assert_eq!(solve::<2>(sample_input), 6);
    }
}
