use std::collections::HashMap;

use aoc::{
    common, io,
    range::{Range, RangeUnion},
};

fn cat_to_idx(cat: &str) -> usize {
    match cat {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!("bad"),
    }
}

#[derive(Clone, Debug)]
enum Action<'a> {
    Accepted,
    Rejected,
    SendTo(&'a str),
}

#[derive(Clone, Debug)]
enum Rule<'a> {
    IfLess([i64; 4], Action<'a>),
    IfMore([i64; 4], Action<'a>),
    Process(Action<'a>),
}

fn get_action(rule: &str) -> Action<'_> {
    if rule == "A" {
        Action::Accepted
    } else if rule == "R" {
        Action::Rejected
    } else {
        Action::SendTo(rule)
    }
}

fn parse_rule<'a>(line: &'a str, map: &mut HashMap<&'a str, Vec<Rule<'a>>>) {
    let (from, rules) = line[0..line.len() - 1].split_once('{').unwrap();
    let rules = io::tokenize(rules, ",");
    for rule in &rules {
        if let Some((condition, dst)) = rule.split_once(':') {
            let dst_action = get_action(dst);
            let mut cat_arr = [0; 4];
            let cond_rule = if let Some((cat, val)) = condition.split_once('<') {
                cat_arr[cat_to_idx(cat)] = io::parse_num(val).unwrap();
                Rule::IfLess(cat_arr, dst_action)
            } else if let Some((cat, val)) = condition.split_once('>') {
                cat_arr[cat_to_idx(cat)] = io::parse_num(val).unwrap();
                Rule::IfMore(cat_arr, dst_action)
            } else {
                panic!("No < or > found!")
            };
            map.entry(from)
                .and_modify(|v| v.push(cond_rule.clone()))
                .or_insert(vec![cond_rule]);
        } else {
            let dst_process = Rule::Process(get_action(rule));
            map.entry(from)
                .and_modify(|v| v.push(dst_process.clone()))
                .or_insert(vec![dst_process]);
        }
    }
}

pub fn split_less(ru: &RangeUnion<i64>, val: i64) -> (RangeUnion<i64>, RangeUnion<i64>) {
    let mut acc: RangeUnion<i64> = RangeUnion::new();
    let mut rej: RangeUnion<i64> = RangeUnion::new();
    //ru.ranges.sort_by(|r1, r2| r1.a.cmp(&r2.a));

    let mut found = false;
    for range in ru.ranges.iter() {
        if range.contains(val) {
            found = true;
            acc.add_range(Range { a: range.a, b: val });
            rej.add_range(Range { a: val, b: range.b });
            continue;
        }

        if found {
            rej.add_range(*range);
        } else {
            acc.add_range(*range);
        }
    }
    (acc, rej)
}

pub fn split_more(ru: &RangeUnion<i64>, val: i64) -> (RangeUnion<i64>, RangeUnion<i64>) {
    let mut acc: RangeUnion<i64> = RangeUnion::new();
    let mut rej: RangeUnion<i64> = RangeUnion::new();
    //ru.ranges.sort_by(|r1, r2| r1.a.cmp(&r2.a));

    let mut found = false;
    for range in ru.ranges.iter() {
        if range.contains(val) {
            found = true;
            rej.add_range(Range {
                a: range.a,
                b: val + 1,
            });
            acc.add_range(Range {
                a: val + 1,
                b: range.b,
            });
            continue;
        }

        if found {
            acc.add_range(*range);
        } else {
            rej.add_range(*range);
        }
    }
    (acc, rej)
}

fn process_range_part<'a>(
    prt: &[RangeUnion<i64>],
    map: &HashMap<&'a str, Vec<Rule<'a>>>,
    start: &'a str,
) -> i64 {
    let rules = map.get(start).unwrap();
    let mut ret = 0;
    let mut part = prt.to_owned();
    for rule in rules {
        match rule {
            Rule::IfLess(cat, action) => {
                for i in 0..4 {
                    if cat[i] == 0 {
                        continue;
                    }
                    let (acc, rej) = split_less(&part[i], cat[i]);
                    match action {
                        Action::Accepted => {
                            let mut part_tmp = part.clone();
                            part_tmp[i] = acc;
                            ret += part_tmp.iter().map(|ru| ru.spread()).product::<i64>();
                        }
                        Action::Rejected => {
                            ret += 0;
                        }
                        Action::SendTo(dst) => {
                            let mut part_tmp = part.clone();
                            part_tmp[i] = acc;
                            ret += process_range_part(&part_tmp, map, dst);
                        }
                    }
                    part[i] = rej;
                }
            }
            Rule::IfMore(cat, action) => {
                for i in 0..4 {
                    if cat[i] == 0 {
                        continue;
                    }
                    let (acc, rej) = split_more(&part[i], cat[i]);
                    match action {
                        Action::Accepted => {
                            let mut part_tmp = part.clone();
                            part_tmp[i] = acc;
                            ret += part_tmp.iter().map(|ru| ru.spread()).product::<i64>();
                        }
                        Action::Rejected => {
                            ret += 0;
                        }
                        Action::SendTo(dst) => {
                            let mut part_tmp = part.clone();
                            part_tmp[i] = acc;
                            ret += process_range_part(&part_tmp, map, dst);
                        }
                    }
                    part[i] = rej;
                }
            }
            Rule::Process(action) => match action {
                Action::Accepted => {
                    ret += part.iter().map(|ru| ru.spread()).product::<i64>();
                }
                Action::Rejected => {
                    ret += 0;
                }
                Action::SendTo(dst) => {
                    ret += process_range_part(&part, map, dst);
                }
            },
        }
    }
    ret
}

fn process_part<'a>(part: [i64; 4], map: &HashMap<&'a str, Vec<Rule<'a>>>, start: &'a str) -> i64 {
    let rules = map.get(start).unwrap();
    for rule in rules {
        match rule {
            Rule::IfLess(cat, action) => {
                for i in 0..4 {
                    if cat[i] == 0 {
                        continue;
                    }
                    if part[i] < cat[i] {
                        match action {
                            Action::Accepted => {
                                return part.iter().sum();
                            }
                            Action::Rejected => {
                                return 0;
                            }
                            Action::SendTo(dst) => {
                                return process_part(part, map, dst);
                            }
                        }
                    }
                }
            }
            Rule::IfMore(cat, action) => {
                for i in 0..4 {
                    if cat[i] == 0 {
                        continue;
                    }
                    if part[i] > cat[i] {
                        match action {
                            Action::Accepted => {
                                return part.iter().sum();
                            }
                            Action::Rejected => {
                                return 0;
                            }
                            Action::SendTo(dst) => {
                                return process_part(part, map, dst);
                            }
                        }
                    }
                }
            }
            Rule::Process(action) => match action {
                Action::Accepted => {
                    return part.iter().sum();
                }
                Action::Rejected => {
                    return 0;
                }
                Action::SendTo(dst) => {
                    return process_part(part, map, dst);
                }
            },
        }
    }
    0
}

fn part1<'a>(input: &'a str) -> i64 {
    let mut map: HashMap<&'a str, Vec<Rule<'a>>> = HashMap::new();
    let batches = io::line_batches(input);

    for line in &batches[0] {
        parse_rule(line, &mut map);
    }

    let mut ans = 0;
    for line in &batches[1] {
        let cats = io::tokenize(&line[1..line.len() - 1], ",");
        assert!(cats.len() == 4);
        let mut part: [i64; 4] = [0; 4];
        for (i, cat) in cats.iter().enumerate() {
            let (_, v) = cat.split_once('=').unwrap();
            part[i] = io::parse_num(v).unwrap();
        }
        ans += process_part(part, &map, "in");
    }
    ans
}

fn part2<'a>(input: &'a str) -> i64 {
    let mut map: HashMap<&'a str, Vec<Rule<'a>>> = HashMap::new();
    let batches = io::line_batches(input);

    for line in &batches[0] {
        parse_rule(line, &mut map);
    }

    let mut rup: Vec<RangeUnion<i64>> = Vec::new();
    for _ in 0..4 {
        let mut ru: RangeUnion<i64> = RangeUnion::new();
        ru.add_range(Range { a: 1, b: 4001 });
        rup.push(ru);
    }

    process_range_part(&rup, &map, "in")
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    common::timed(&input, part2, false);
}