use aoc::{common, grid::Grid, io};

fn solve2(input: &str) -> usize {
    let mut grid = Grid::from_str_no_trim(input, |c| c);
    //grid.print();
    let max_l = grid.values.iter().map(|v| v.len()).max().unwrap_or(0);
    //dbg!(max_l);
    for val in grid.values.iter_mut() {
        val.resize(max_l, ' ');
    }
    grid.cols = max_l;
    //grid.print();
    let grid2 = grid.rotate_clockwise();
    //grid2.print();

    let mut ans = 0;
    let mut op_mul = false;
    let mut ret = 0;
    for val in grid2.values.iter() {
        let str = val.iter().collect::<String>();
        //println!("{}\n", str);
        if str.trim().is_empty() {
            //dbg!(ret);
            ans += ret;
            ret = 0;
            op_mul = false;
            continue;
        }
        let x = if str.contains("*") {
            op_mul = true;
            ret = 1;
            str[1..]
                .trim()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        } else if str.contains("+") {
            op_mul = false;
            ret = 0;
            str[1..]
                .trim()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        } else {
            str.trim()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        };

        //dbg!(x);
        if op_mul {
            ret *= x;
        } else {
            ret += x;
        }
    }
    ans += ret;
    ans
}

fn solve<const PART: usize>(input: &str) -> usize {
    let mut ans = 0;
    let mut nums_tot: Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        //dbg!(line);
        if line.contains("*") || line.contains("+") {
            let ops = io::tokenize(line, " ");
            for (j, op) in ops.iter().enumerate() {
                match *op {
                    "+" => {
                        let mut ret = 0;
                        for i in 0..nums_tot.len() {
                            ret += nums_tot[i][j];
                        }
                        ans += ret;
                    }
                    "*" => {
                        let mut ret = 1;
                        for i in 0..nums_tot.len() {
                            ret *= nums_tot[i][j];
                        }
                        ans += ret;
                    }
                    _ => {}
                }
            }
        } else {
            let nums = io::tokenize_nums(line, " ");
            nums_tot.push(nums);
        }
    }

    ans
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, solve::<1>, true);
        common::timed(&input, solve2, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "123 328  51 64\n 45 64  387 23\n  6 98  215 314\n*   +   *   +  ";
        assert_eq!(solve::<1>(sample_input), 4277556);
        assert_eq!(solve2(sample_input), 3263827);
    }
}
