use aoc::common;

fn part1(input: &str) -> i32 {
    let mut total_wrapping_paper_area = 0;
    for dims_str in input.lines() {
        let dims = dims_str
            .split('x')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut areas = [dims[0] * dims[1], dims[1] * dims[2], dims[2] * dims[0]];
        areas.sort_unstable();
        let wrapping_paper_area = areas[0] + 2 * (areas[0] + areas[1] + areas[2]);
        total_wrapping_paper_area += wrapping_paper_area;
    }
    total_wrapping_paper_area
}

fn part2(input: &str) -> i32 {
    let mut total_ribbon_length = 0;
    for dims_str in input.lines() {
        let mut dims = dims_str
            .split('x')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        dims.sort_unstable();
        let ribbon_length = 2 * (dims[0] + dims[1]) + dims[0] * dims[1] * dims[2];
        total_ribbon_length += ribbon_length;
    }
    total_ribbon_length
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    common::timed(&input, part2, false);
}
