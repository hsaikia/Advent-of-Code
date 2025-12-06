use aoc::{common, grid::Grid, io};

enum Operation {
    Add,
    Multiply,
}

fn solve2(input: &str) -> usize {
    let grid = Grid::from_str_no_trim(input, |c| c, &' ');
    //grid.print();
    let grid2 = grid.rotate_clockwise();
    //grid2.print();

    let mut ans = 0;
    let mut op = Operation::Add;
    let mut ret = 0;
    for val in grid2.values.iter() {
        if val[0] == '+' {
            op = Operation::Add;
            ret = 0;
        } else if val[0] == '*' {
            op = Operation::Multiply;
            ret = 1;
        };
        if let Ok(number) = val
            .iter()
            .skip(1)
            .rev()
            .collect::<String>()
            .trim()
            .parse::<usize>()
        {
            //dbg!(number);
            match op {
                Operation::Add => ret += number,
                Operation::Multiply => ret *= number,
            }
        } else {
            ans += ret;
            ret = 0;
        }
    }
    ans += ret;
    ans
}

fn solve1(input: &str) -> usize {
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
                        for row in &nums_tot {
                            ret += row[j];
                        }
                        ans += ret;
                    }
                    "*" => {
                        let mut ret = 1;
                        for row in &nums_tot {
                            ret *= row[j];
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
        common::timed(&input, solve1, true);
        common::timed(&input, solve2, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "123 328  51 64\n 45 64  387 23\n  6 98  215 314\n*   +   *   +  ";
        assert_eq!(solve1(sample_input), 4277556);
        assert_eq!(solve2(sample_input), 3263827);
    }
}
