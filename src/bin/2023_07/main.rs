use std::cmp::Ordering;

use aoc::io;
use itertools::Itertools;

const INPUT: [(&str, &str); 2] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

const ORDER_1: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];
const ORDER_2: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

#[derive(Debug)]
enum HandType {
    FiveOfAKind = 0,
    FourOfAKind = 1,
    FullHouse = 2,
    ThreeOfAKind = 3,
    TwoPair = 4,
    OnePair = 5,
    HighCard = 6,
}

fn compare(hand1: HandType, hand2: HandType) -> Ordering {
    (hand1 as usize).cmp(&(hand2 as usize))
}

fn hand_type_from_sizes(unique_cards: usize, largest_set_size: usize) -> HandType {
    if unique_cards == 5 {
        return HandType::HighCard;
    } else if unique_cards == 4 {
        return HandType::OnePair;
    } else if unique_cards == 3 {
        if largest_set_size == 3 {
            return HandType::ThreeOfAKind;
        } else {
            return HandType::TwoPair;
        }
    } else if unique_cards == 2 {
        if largest_set_size == 4 {
            return HandType::FourOfAKind;
        } else {
            return HandType::FullHouse;
        }
    }

    HandType::FiveOfAKind
}

fn get_hand_type(hand: &str, joker: bool) -> HandType {
    let (unique_cards, largest_set_size) = if joker {
        let u = hand
            .chars()
            .filter(|c| *c != 'J')
            .unique()
            .collect::<Vec<_>>();
        let mut counts = u
            .iter()
            .map(|x| hand.chars().filter(|y| y == x).count())
            .collect::<Vec<_>>();
        counts.sort_by(|a, b| b.cmp(a));
        let joker_counts = hand.chars().filter(|c| *c == 'J').count();
        if counts.is_empty() {
            (1, joker_counts)
        } else {
            (u.len(), counts[0] + joker_counts)
        }
    } else {
        let u = hand.chars().unique().collect::<Vec<_>>();
        let mut counts = u
            .iter()
            .map(|x| hand.chars().filter(|y| y == x).count())
            .collect::<Vec<_>>();
        counts.sort_by(|a, b| b.cmp(a));
        (u.len(), counts[0])
    };

    hand_type_from_sizes(unique_cards, largest_set_size)
}

fn compare_hands(hand1: &str, hand2: &str, part1: bool) -> Ordering {
    let t1 = get_hand_type(hand1, !part1);
    let t2 = get_hand_type(hand2, !part1);

    let c = compare(t1, t2);
    if c == Ordering::Equal {
        let c1 = hand1.chars().collect::<Vec<_>>();
        let c2 = hand2.chars().collect::<Vec<_>>();
        for i in 0..5 {
            if c1[i] == c2[i] {
                continue;
            }
            let order = if part1 { ORDER_1 } else { ORDER_2 };
            let idx1 = order.iter().position(|c| *c == c1[i]).unwrap();
            let idx2 = order.iter().position(|c| *c == c2[i]).unwrap();

            if idx1 == idx2 {
                continue;
            }

            return idx1.cmp(&idx2);
        }
    }
    c
}

fn solve(input: &str, part1: bool) {
    let mut cards: Vec<(&str, usize)> = Vec::new();

    for line in input.split('\n') {
        let hand_bid = io::tokenize(line, " ");
        cards.push((hand_bid[0], io::parse_num::<usize>(hand_bid[1]).unwrap()));
    }

    cards.sort_by(|c1, c2| compare_hands(c2.0, c1.0, part1));
    if part1 {
        println!(
            "Answer Part 1 {}",
            cards
                .iter()
                .enumerate()
                .map(|(i, e)| (i + 1) * e.1)
                .sum::<usize>()
        );
    } else {
        println!(
            "Answer Part 2 {}",
            cards
                .iter()
                .enumerate()
                .map(|(i, e)| (i + 1) * e.1)
                .sum::<usize>()
        );
    }
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        solve(input.1, true);
        solve(input.1, false);
    }
}
