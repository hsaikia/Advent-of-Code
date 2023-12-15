use aoc::{common, grid::Grid};

fn solve(grid: &Grid<char>, part1: bool) -> usize {
    let mut ans: usize = 0;

    for i in 0..grid.rows - 1 {
        let mut r1 = i as i32;
        let mut r2 = i as i32 + 1;
        let mut common_elements = 0;
        let mut elements = 0;
        while r1 >= 0 && r2 < grid.rows as i32 {
            common_elements += grid.common_elements_in_rows(r1 as usize, r2 as usize);
            elements += grid.cols;
            r1 -= 1;
            r2 += 1;
        }

        if (part1 && common_elements == elements) || (!part1 && common_elements + 1 == elements) {
            //println!("Mirror row : {}:{}", i, i + 1);
            ans += 100 * (i + 1);
        }
    }

    for i in 0..grid.cols - 1 {
        let mut c1 = i as i32;
        let mut c2 = i as i32 + 1;
        let mut common_elements = 0;
        let mut elements = 0;
        while c1 >= 0 && c2 < grid.cols as i32 {
            common_elements += grid.common_elements_in_cols(c1 as usize, c2 as usize);
            elements += grid.rows;
            c1 -= 1;
            c2 += 1;
        }

        if (part1 && common_elements == elements) || (!part1 && common_elements + 1 == elements) {
            //println!("Mirror col : {}:{}", i, i + 1);
            ans += i + 1;
        }
    }

    ans
}

fn process_and_solve<const PART1: bool>(input: &str) -> usize {
    let mut ans = 0;
    let batches = input.split("\n\n").collect::<Vec<_>>();
    for batch in batches {
        let grid = Grid::from_str(batch, |c| c);
        ans += solve(&grid, PART1);
    }
    ans
}

fn main() {
    let input = common::get_input();
    common::timed(&input, process_and_solve::<true>, true);
    common::timed(&input, process_and_solve::<false>, false);
}
