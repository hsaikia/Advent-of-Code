use std::cmp::Ordering;

use aoc::{common, io};

#[derive(Debug, Clone)]
enum Packet {
    List(Vec<Packet>),
    Num(u32),
}

fn compare(p1: &Packet, p2: &Packet) -> Ordering {
    match p1 {
        Packet::List(v1) => match p2 {
            Packet::List(v2) => {
                for (x, y) in v1.iter().zip(v2.iter()) {
                    if compare(x, y) == Ordering::Equal {
                        continue;
                    }
                    return compare(x, y);
                }
                v1.len().cmp(&v2.len())
            }
            Packet::Num(n2) => compare(p1, &Packet::List(vec![Packet::Num(*n2)])),
        },
        Packet::Num(n1) => match p2 {
            Packet::List(_) => compare(&Packet::List(vec![Packet::Num(*n1)]), p2),
            Packet::Num(n2) => n1.cmp(n2),
        },
    }
}

fn parse_packet(packet_str: &str) -> Packet {
    let mut open = Vec::new();
    let mut packets = Vec::new();

    if packet_str.is_empty() {
        return Packet::List(Vec::new());
    }

    let chs = packet_str.chars().collect::<Vec<_>>();

    let mut num: Option<u32> = None;
    for (i, ch) in chs.iter().enumerate() {
        if *ch == '[' {
            open.push(i);
        } else if *ch == ']' {
            let x = open.pop().unwrap();
            if open.is_empty() {
                packets.push(parse_packet(&packet_str[x + 1..i]));
            }
        } else if ch.is_ascii_digit() {
            let d = ch.to_digit(10).unwrap();
            if open.is_empty() {
                if let Some(n) = &mut num {
                    *n = 10 * *n + d;
                } else {
                    num = Some(d);
                }
            }
        } else if *ch == ',' {
            if let Some(n) = &mut num {
                if open.is_empty() {
                    packets.push(Packet::Num(*n));
                }
                num = None;
            }
        }
    }

    if let Some(n) = num {
        packets.push(Packet::Num(n));
    }

    Packet::List(packets)
}

fn part1(packet_pairs: &[(Packet, Packet)]) {
    let ans = packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (p1, p2))| {
            if compare(p1, p2) == Ordering::Less {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum::<usize>();

    println!("Part 1 Answer is {}", ans);
}

fn part2(packet_pairs: &[(Packet, Packet)]) {
    let mut packets = Vec::new();
    for (a, b) in packet_pairs {
        packets.push((a.clone(), 0));
        packets.push((b.clone(), 0));
    }

    packets.push((parse_packet("[[2]]"), 1));
    packets.push((parse_packet("[[6]]"), 1));

    packets.sort_by(|a, b| compare(&a.0, &b.0));

    let ans = packets
        .iter()
        .enumerate()
        .map(|(i, p)| (i + 1) * p.1)
        .filter(|x| *x > 0)
        .collect::<Vec<_>>();
    println!("Part 2 Answer is {}", ans[0] * ans[1]);
}

fn main() {
    let input = common::get_input();
    let packet_str_pairs = io::line_batches(&input);
    let mut packet_pairs: Vec<(Packet, Packet)> = Vec::new();
    for packet_str_pair in packet_str_pairs {
        let p1 = parse_packet(packet_str_pair[0]);
        let p2 = parse_packet(packet_str_pair[1]);
        packet_pairs.push((p1, p2));
    }

    part1(&packet_pairs);
    part2(&packet_pairs);
}
