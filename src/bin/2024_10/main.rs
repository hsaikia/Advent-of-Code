use aoc::{common, grid::Grid};

fn solve<const PART: i32>(input: &str) -> usize {
    let mut ans = 0;
    let map = Grid::from_str(input, |c| c.to_digit(10).unwrap());
    let trailheads = map.positions(0);
    for pos in trailheads {
        let mut q = Vec::new();
        q.push((pos, 0));
        let mut trails = Vec::new();

        while let Some(tp) = q.pop() {
            if tp.1 == 9 {
                trails.push(tp.0);
                continue;
            }
            let nns = map.adjacent_4(&tp.0);
            for nn in nns.iter() {
                if map.get(nn) == tp.1 + 1 {
                    q.push((*nn, tp.1 + 1));
                }
            }
        }
        if PART == 1 {
            trails.sort();
            trails.dedup();
        }
        ans += trails.len();
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
        let sample_input =
            "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";
        assert_eq!(solve::<1>(sample_input), 36);
        assert_eq!(solve::<2>(sample_input), 81);
    }
}
