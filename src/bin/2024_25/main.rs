use aoc::{common, grid::Grid};

fn solve(input: &str) -> usize {
    let mut ans = 0;
    let mut rows = 0;
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for pat in input.split("\n\n") {
        let grid = Grid::from_str(pat, |c| c);
        rows = grid.rows;
        let vec: Vec<usize> = (0..grid.cols)
            .map(|c| grid.find_in_col(c, '#').len() - 1)
            .collect();
        if grid.get(&(0, 0)) == '#' {
            // lock
            locks.push(vec);
        } else {
            // key
            keys.push(vec);
        }
    }

    for l in locks.iter() {
        for k in keys.iter() {
            ans += if l.iter().zip(k.iter()).all(|(a, b)| *a + *b < rows - 1) {
                1
            } else {
                0
            };
        }
    }
    ans
}

fn main() {
    let input = common::get_input();
    println!("{input:?}");
    common::timed(&input, solve, true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "#####\n.####\n.####\n.####\n.#.#.\n.#...\n.....\n\n#####\n##.##\n.#.##\n...##\n...#.\n...#.\n.....\n\n.....\n#....\n#....\n#...#\n#.#.#\n#.###\n#####\n\n.....\n.....\n#.#..\n###..\n###.#\n###.#\n#####\n\n.....\n.....\n.....\n#....\n#.#..\n#.#.#\n#####";
        assert_eq!(solve(sample_input), 3);
    }
}
