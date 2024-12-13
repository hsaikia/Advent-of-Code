use aoc::{analytic::linear_equation, common, io};

fn get_x_y(batch: &str, delim1: &str, delim2: &str) -> (i64, i64) {
    let tokens = io::tokenize(batch, delim1);
    let tokens1 = io::tokenize(tokens[1], ", ");
    let x: i64 = io::parse_num(tokens1[0]);
    let tokens2 = io::tokenize(tokens1[1], delim2);
    let y: i64 = io::parse_num(tokens2[1]);
    (x, y)
}

fn solve<const PART: usize>(input: &str) -> i64 {
    let mut ans = 0;
    for batch in io::line_batches(input) {
        let (ax, ay) = get_x_y(batch[0], "X+", "+");
        let (bx, by) = get_x_y(batch[1], "X+", "+");
        let (px, py) = get_x_y(batch[2], "X=", "=");

        let px = px + if PART == 2 { 10000000000000 } else { 0 };
        let py = py + if PART == 2 { 10000000000000 } else { 0 };

        if let Some((sa, sb)) = linear_equation(ax, bx, px, ay, by, py) {
            ans += sa * 3 + sb;
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
        let sample_input = "Button A: X+94, Y+34\nButton B: X+22, Y+67\nPrize: X=8400, Y=5400\n\nButton A: X+26, Y+66\nButton B: X+67, Y+21\nPrize: X=12748, Y=12176\n\nButton A: X+17, Y+86\nButton B: X+84, Y+37\nPrize: X=7870, Y=6450\n\nButton A: X+69, Y+23\nButton B: X+27, Y+71\nPrize: X=18641, Y=10279";
        assert_eq!(solve::<1>(sample_input), 480);
        assert_eq!(solve::<2>(sample_input), 875318608908);
    }
}
