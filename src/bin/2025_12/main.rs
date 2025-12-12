use aoc::{common, grid::Grid, io};

fn solve<const PART: usize>(input: &str) -> usize {
    let mut ans = 0;

    let mut shape_lines = Vec::new();
    let mut cases: Vec<(usize, usize, Vec<usize>)> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains("x") {
            if let Some((dim, vals)) = line.split_once(": ") {
                let dims = io::tokenize_nums(dim, "x");
                let values = io::tokenize_nums(vals, " ");
                cases.push((dims[0], dims[1], values));
            }
        } else {
            shape_lines.push(line);
        }
    }

    let total_shapes = shape_lines.iter().filter(|l| l.contains(":")).count();
    let mut shapes: Vec<Grid<char>> = vec![Grid::new(3, 3, '.'); total_shapes];

    for i in 0..total_shapes {
        for r in 0..3 {
            shapes[i].set_row(r, shape_lines[4 * i + r + 1].chars().collect());
        }
    }

    // LEt's figure out the clear YESes and clear NOs first..
    // Clear yes : Can fit all 3x3 shapes
    // Clear no : Total space too little to fit even all solid spaces

    let solid_spaces: Vec<usize> = shapes.iter().map(|g| g.count(&'#')).collect();

    for (d0, d1, number) in &cases {
        let x = d0 / 3;
        let y = d1 / 3;
        let n = number.iter().sum();
        if x * y >= n {
            println!("Trivial YES");
            ans += 1;
            continue;
        }

        let min_solid_spaces_required: usize = number
            .iter()
            .zip(solid_spaces.iter())
            .map(|(x, y)| x * y)
            .sum();

        if min_solid_spaces_required > d0 * d1 {
            println!("Trivial NO");
            continue;
        }

        // Thanks for the input Eric!
        println!("Tough : {}x{} : {:?}", d0, d1, number);
    }

    ans
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, solve::<1>, true);
    }
}
