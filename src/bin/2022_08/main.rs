use aoc::grid::Grid;

const INPUT: [(&str, &str); 2] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

fn part1(grid: &Grid<u32>) {
    // Check visibility for each internal tree
    let mut visible = Grid::<bool>::new(grid.rows, grid.cols, false);

    for i in 0..grid.rows {
        let mut best: [i32; 2] = [-1, -1];

        // left to right sweep
        for j in 0..grid.cols {
            let val = grid.get(i, j).unwrap() as i32;
            if val > best[0] {
                best[0] = val;
                visible.set(i, j, true);
            }
        }

        // right to left sweep
        for j in (0..grid.cols).rev() {
            let val = grid.get(i, j).unwrap() as i32;
            if val > best[1] {
                best[1] = val;
                visible.set(i, j, true);
            }
        }
    }

    for j in 0..grid.cols {
        let mut best: [i32; 2] = [-1, -1];

        // top to bottom sweep
        for i in 0..grid.rows {
            let val = grid.get(i, j).unwrap() as i32;
            if val > best[0] {
                best[0] = val;
                visible.set(i, j, true);
            }
        }

        // bottom to top sweep
        for i in (0..grid.rows).rev() {
            let val = grid.get(i, j).unwrap() as i32;
            if val > best[1] {
                best[1] = val;
                visible.set(i, j, true);
            }
        }
    }

    let ans = visible
        .values
        .iter()
        .map(|row| row.iter().filter(|&x| *x).count())
        .sum::<usize>();
    println!("Part 1 Answer {}", ans);
}

fn part2(grid: &Grid<u32>) {
    let mut ans = 0;
    for i in 0..grid.rows {
        for j in 0..grid.cols {
            let mut scores = [0; 4];
            let sweeps = grid.sweep_4(i, j);
            let h = grid.get(i, j).unwrap();

            for (i, sweep) in sweeps.iter().enumerate() {
                for (ni, nj) in sweep {
                    let h1 = grid.get(*ni, *nj).unwrap();
                    scores[i] += 1;

                    if h1 >= h {
                        break;
                    }
                }
            }

            ans = ans.max(scores.iter().product::<i32>());
        }
    }
    println!("Part 2 Answer {}", ans);
}

fn main() {
    for (file, input) in INPUT {
        println!("{}", file);
        let grid = Grid::from_str(input, |c| c.to_digit(10).unwrap());
        part1(&grid);
        part2(&grid);
    }
}
