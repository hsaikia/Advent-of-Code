use std::collections::HashSet;

use aoc::{common, io};

fn hash_set_from_str(strs: &[&str]) -> HashSet<usize> {
    strs.iter()
        .map(|s| io::parse_num::<usize>(s))
        .collect::<HashSet<_>>()
}

fn val(sz: usize) -> usize {
    if sz < 2 {
        return sz;
    }
    2_usize.pow(u32::try_from(sz).unwrap() - 1)
}

fn matching_cards(input: &str) -> Vec<Vec<usize>> {
    let mut ret: Vec<Vec<usize>> = Vec::new();
    for line in input.lines() {
        let cards = line.split(" | ").collect::<Vec<_>>();
        let segment1 = cards[0].split(": ").collect::<Vec<_>>();
        let winning = io::tokenize(segment1[1], " ");
        let in_hand = io::tokenize(cards[1], " ");

        let l1 = hash_set_from_str(&winning);
        let l2 = hash_set_from_str(&in_hand);

        ret.push(l1.intersection(&l2).copied().collect::<Vec<usize>>());
    }
    ret
}

fn part1(matches: &Vec<Vec<usize>>) -> usize {
    let mut ans: usize = 0;
    for m in matches {
        ans += val(m.len());
    }
    ans
}

fn part2(matching_cards: &[Vec<usize>]) -> usize {
    let n = matching_cards.len();
    let mut number_of_cards = Vec::with_capacity(n);
    number_of_cards.resize(n, 1);

    for (i, m) in matching_cards.iter().enumerate() {
        for idx in i + 1..i + 1 + m.len() {
            if idx < n {
                number_of_cards[idx] += number_of_cards[i];
            }
        }
    }
    number_of_cards.iter().sum::<usize>()
}

fn solve<const PART1: bool>(input: &str) -> usize {
    let matching_cards = matching_cards(input);
    if PART1 {
        return part1(&matching_cards);
    }
    part2(&matching_cards)
}

fn main() {
    let input = common::get_input();
    common::timed(&input, solve::<true>, true);
    common::timed(&input, solve::<false>, false);
}
