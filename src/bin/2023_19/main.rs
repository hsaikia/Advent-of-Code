use std::collections::HashMap;

use aoc::{common, io, range::Range};

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

fn process_range_part<'a>(
    prt: &[Range<i64>],
    map: &HashMap<&'a str, Vec<Rule<'a>>>,
    start: &'a str,
) -> i64 {
    //println!("Visiting {:?}", prt);
    let rules = map.get(start).unwrap();
    let mut ret = 0;
    let mut part = prt.to_owned();

    for rule in rules {
        match rule {
            Rule::IfLess(cat, action) => {
                let idx = (0..4).filter(|&i| cat[i] != 0).collect::<Vec<_>>()[0];
                let range = Range { a: 1, b: cat[idx] };
                let mut part_tmp = part.clone();
                part_tmp[idx] = part[idx].intersect(&range).unwrap();
                part[idx] = part[idx].difference(&range)[0];

                ret += match action {
                    Action::Accepted => part_tmp.iter().map(|r| r.spread()).product::<i64>(),
                    Action::Rejected => 0,
                    Action::SendTo(dst) => process_range_part(&part_tmp, map, dst),
                };
            }
            Rule::IfMore(cat, action) => {
                let idx = (0..4).filter(|&i| cat[i] != 0).collect::<Vec<_>>()[0];
                let range = Range {
                    a: cat[idx] + 1,
                    b: 4001,
                };
                let mut part_tmp = part.clone();
                part_tmp[idx] = part[idx].intersect(&range).unwrap();
                part[idx] = part[idx].difference(&range)[0];

                ret += match action {
                    Action::Accepted => part_tmp.iter().map(|r| r.spread()).product::<i64>(),
                    Action::Rejected => 0,
                    Action::SendTo(dst) => process_range_part(&part_tmp, map, dst),
                };
            }
            Rule::Process(action) => {
                ret += match action {
                    Action::Accepted => part.iter().map(|r| r.spread()).product::<i64>(),
                    Action::Rejected => 0,
                    Action::SendTo(dst) => process_range_part(&part, map, dst),
                };
            }
        }
    }
    ret
}

fn process_part<'a>(part: [i64; 4], map: &HashMap<&'a str, Vec<Rule<'a>>>, start: &'a str) -> i64 {
    let rules = map.get(start).unwrap();
    for rule in rules {
        match rule {
            Rule::IfLess(cat, action) => {
                let idx = (0..4).filter(|&i| cat[i] != 0).collect::<Vec<_>>()[0];
                if part[idx] < cat[idx] {
                    return match action {
                        Action::Accepted => part.iter().sum(),
                        Action::Rejected => 0,
                        Action::SendTo(dst) => process_part(part, map, dst),
                    };
                }
            }
            Rule::IfMore(cat, action) => {
                let idx = (0..4).filter(|&i| cat[i] != 0).collect::<Vec<_>>()[0];
                if part[idx] > cat[idx] {
                    return match action {
                        Action::Accepted => part.iter().sum(),
                        Action::Rejected => 0,
                        Action::SendTo(dst) => process_part(part, map, dst),
                    };
                }
            }
            Rule::Process(action) => {
                return match action {
                    Action::Accepted => part.iter().sum(),
                    Action::Rejected => 0,
                    Action::SendTo(dst) => process_part(part, map, dst),
                }
            }
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

    let mut ranges: Vec<Range<i64>> = Vec::new();
    for _ in 0..4 {
        ranges.push(Range { a: 1, b: 4001 });
    }

    process_range_part(&ranges, &map, "in")
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    common::timed(&input, part2, false);
}
