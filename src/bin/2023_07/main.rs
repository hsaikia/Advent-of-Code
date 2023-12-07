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

fn hand_type_from_sizes(num_unique_cards: usize, largest_set_size: usize) -> HandType {
    if num_unique_cards == 5 {
        return HandType::HighCard;
    } else if num_unique_cards == 4 {
        return HandType::OnePair;
    } else if num_unique_cards == 3 {
        if largest_set_size == 3 {
            return HandType::ThreeOfAKind;
        } else {
            return HandType::TwoPair;
        }
    } else if num_unique_cards == 2 {
        if largest_set_size == 4 {
            return HandType::FourOfAKind;
        } else {
            return HandType::FullHouse;
        }
    }

    HandType::FiveOfAKind
}

fn get_hand_type(hand: &str, joker: bool) -> HandType {
    let (num_unique_cards, largest_set_size) = if joker {
        let unique_cards = hand
            .chars()
            .filter(|c| *c != 'J')
            .unique()
            .collect::<Vec<_>>();
        let mut counts = unique_cards
            .iter()
            .map(|x| hand.chars().filter(|y| y == x).count())
            .collect::<Vec<_>>();
        counts.sort_by(|a, b| b.cmp(a));
        let joker_counts = hand.chars().filter(|c| *c == 'J').count();
        if counts.is_empty() {
            (1, joker_counts)
        } else {
            (unique_cards.len(), counts[0] + joker_counts)
        }
    } else {
        let unique_cards = hand.chars().unique().collect::<Vec<_>>();
        let mut counts = unique_cards
            .iter()
            .map(|x| hand.chars().filter(|y| y == x).count())
            .collect::<Vec<_>>();
        counts.sort_by(|a, b| b.cmp(a));
        (unique_cards.len(), counts[0])
    };

    hand_type_from_sizes(num_unique_cards, largest_set_size)
}

fn compare_hands(hand1: &str, hand2: &str, joker: bool) -> Ordering {
    let t1 = get_hand_type(hand1, joker) as usize;
    let t2 = get_hand_type(hand2, joker) as usize;

    let order: Ordering = t1.cmp(&t2);

    if order == Ordering::Equal {
        let cards1 = hand1.chars().collect::<Vec<_>>();
        let cards2 = hand2.chars().collect::<Vec<_>>();

        for (c1, c2) in cards1.iter().zip(&cards2) {
            if c1 == c2 {
                continue;
            }
            let card_order = if joker { ORDER_2 } else { ORDER_1 };
            let idx1 = card_order.iter().position(|c| c == c1).unwrap();
            let idx2 = card_order.iter().position(|c| c == c2).unwrap();

            if idx1 == idx2 {
                continue;
            }

            return idx1.cmp(&idx2);
        }
    }
    order
}

fn solve(input: &str, joker: bool) {
    let mut cards: Vec<(&str, usize)> = Vec::new();

    for line in input.split('\n') {
        let hand_bid = io::tokenize(line, " ");
        cards.push((hand_bid[0], io::parse_num::<usize>(hand_bid[1]).unwrap()));
    }

    cards.sort_by(|c1, c2| compare_hands(c2.0, c1.0, joker));

    let ans = cards
        .iter()
        .enumerate()
        .map(|(i, e)| (i + 1) * e.1)
        .sum::<usize>();

    if !joker {
        println!("Answer Part 1 {}", ans);
    } else {
        println!("Answer Part 2 {}", ans);
    }
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        solve(input.1, false);
        solve(input.1, true);
    }
}
