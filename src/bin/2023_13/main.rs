use aoc::grid::Grid;

const INPUT: [(&str, &str); 2] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

fn solve(input: &str) {
    let mut ans: usize = 0;

    let batches = input.split("\n\n").collect::<Vec<_>>();
    for batch in batches {
        let grid = Grid::from_str(batch, |c| c);

        for i in 0..grid.rows - 1 {
            let mut r1 = i as i32;
            let mut r2 = i as i32 + 1;
            let mut found = true;
            while r1 >= 0 && r2 < grid.rows as i32 {
                if !grid.are_rows_equal(r1 as usize, r2 as usize) {
                    found = false;
                    break;
                }
                r1 -= 1;
                r2 += 1;
            }

            if found {
                //println!("Mirror row : {}:{}", i, i + 1);
                ans += 100 * (i + 1);
            }
        }

        for i in 0..grid.cols - 1 {
            let mut c1 = i as i32;
            let mut c2 = i as i32 + 1;
            let mut found = true;
            while c1 >= 0 && c2 < grid.cols as i32 {
                if !grid.are_cols_equal(c1 as usize, c2 as usize) {
                    found = false;
                    break;
                }
                c1 -= 1;
                c2 += 1;
            }

            if found {
                //println!("Mirror col : {}:{}", i, i + 1);
                ans += i + 1;
            }
        }
    }

    println!("Answer Part1 : {}", ans);
}

fn solve2(input: &str) {
    let mut ans: usize = 0;

    let batches = input.split("\n\n").collect::<Vec<_>>();
    for batch in batches {
        let grid = Grid::from_str(batch, |c| c);

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

            if common_elements + 1 == elements {
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

            if common_elements + 1 == elements {
                //println!("Mirror col : {}:{}", i, i + 1);
                ans += i + 1;
            }
        }
    }

    println!("Answer Part2 : {}", ans);
}

fn main() {
    for (file, input) in INPUT {
        println!("{}", file);
        solve(input);
        solve2(input);
    }
}
