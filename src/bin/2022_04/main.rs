use aoc::common;

fn part1(input_lines: &str) -> usize {
    let mut ans: usize = 0;

    for line in input_lines.split('\n') {
        let ranges: Vec<_> = line.split(',').collect();
        let idx1: Vec<_> = ranges[0].split('-').flat_map(str::parse::<usize>).collect();
        let idx2: Vec<_> = ranges[1].split('-').flat_map(str::parse::<usize>).collect();

        if (idx2[0] >= idx1[0] && idx2[1] <= idx1[1]) || (idx1[0] >= idx2[0] && idx1[1] <= idx2[1])
        {
            ans += 1;
        }
    }

    ans
}

fn part2(input_lines: &str) -> usize {
    let mut ans: usize = 0;

    for line in input_lines.split('\n') {
        let ranges: Vec<_> = line.split(',').collect();
        let idx1: Vec<_> = ranges[0].split('-').flat_map(str::parse::<usize>).collect();
        let idx2: Vec<_> = ranges[1].split('-').flat_map(str::parse::<usize>).collect();

        let idx_l = idx1[0].max(idx2[0]);
        let idx_r = idx1[1].min(idx2[1]);

        if idx_l <= idx_r {
            ans += 1;
        }
    }

    ans
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, part1, true);
        common::timed(&input, part2, false);
    }
}
