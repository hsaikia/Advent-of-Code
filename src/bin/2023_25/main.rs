use std::collections::{HashMap, HashSet, VecDeque};

use aoc::{
    common,
    grid::{CellDir, CellIndex, Grid},
    io,
};

fn part1(input: &str) -> usize {
    let mut ans = 0;
    ans
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    //common::timed(&input, part2, false);
}
