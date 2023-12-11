use aoc::{common, grid::Grid};

const INPUT: [(&str, &str); 2] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

fn solve(input: &str, galaxy_expansion: usize) {
    let mut ans: usize = 0;

    let grid = Grid::<char>::from_str(input, |c| c);
    let empty_rows = (0..grid.rows)
        .filter(|r| grid.find_in_row(*r, '#').is_empty())
        .collect::<Vec<_>>();
    let empty_cols = (0..grid.cols)
        .filter(|c| grid.find_in_col(*c, '#').is_empty())
        .collect::<Vec<_>>();

    let star_positions = grid.positions('#');

    for (i, (r1, c1)) in star_positions.iter().enumerate() {
        for (r2, c2) in star_positions.iter().skip(i + 1) {
            let (r1, r2) = common::minmax(r1, r2);
            let (c1, c2) = common::minmax(c1, c2);
            let mut d = r2 + c2 - r1 - c1;

            d += (r1 + 1..r2).filter(|r| empty_rows.contains(r)).count() * (galaxy_expansion - 1);
            d += (c1 + 1..c2).filter(|c| empty_cols.contains(c)).count() * (galaxy_expansion - 1);

            ans += d;
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