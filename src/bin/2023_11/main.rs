use aoc::grid::Grid;

const INPUT: [(&str, &str); 2] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

fn solve(input: &str, galaxy_expansion : usize) {
    let mut ans: usize = 0;

    let grid = Grid::<char>::from_str(input, |c| c);
    let empty_rows = (0..grid.rows).filter(|r| grid.find_in_row(*r, &'#').is_empty()).collect::<Vec<_>>();
    let empty_cols = (0..grid.cols).filter(|r| grid.find_in_col(*r, &'#').is_empty()).collect::<Vec<_>>();
    let stars = grid.positions('#');

    for i in 0..stars.len() {
        for j in i + 1..stars.len() {
            let mut d = stars[i].0.abs_diff(stars[j].0) + stars[i].1.abs_diff(stars[j].1);

            let r1 = stars[i].0.min(stars[j].0);
            let r2 = stars[i].0.max(stars[j].0);
            let c1 = stars[i].1.min(stars[j].1);
            let c2 = stars[i].1.max(stars[j].1);
            for row in r1 + 1..r2 {
                if empty_rows.contains(&row) {
                    d += galaxy_expansion - 1;
                } 
            }
            for col in c1 + 1..c2 {
                if empty_cols.contains(&col) {
                    d += galaxy_expansion - 1;
                } 
            }
            ans += d ;
        }
    }

    println!("Answer : {}", ans);
}

fn main() {
    for (file, input) in INPUT {
        println!("{}", file);
        solve(input, 2);
        solve(input, 1000000);
    }
}
