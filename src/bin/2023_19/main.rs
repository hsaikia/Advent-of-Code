use std::collections::HashMap;

use aoc::{
    common::{self, HashMapVector},
    io,
    range::Range,
};

const MIN_INC_RANGE: i64 = 1;
const MAX_EXC_RANGE: i64 = 4001;

fn category_to_idx(category: &str) -> usize {
    match category {
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

#[derive(Clone, Debug, PartialEq)]
enum Cmp {
    Less,
    More,
}

#[derive(Clone, Debug)]
enum Rule<'a> {
    Conditional([i64; 4], Action<'a>, Cmp),
    Process(Action<'a>),
}

fn get_action(res: &str) -> Action<'_> {
    if res == "A" {
        Action::Accepted
    } else if res == "R" {
        Action::Rejected
    } else {
        Action::SendTo(res)
    }
}

fn parse_rule<'a>(line: &'a str, map: &mut HashMap<&'a str, Vec<Rule<'a>>>) {
    let (from, rules) = line[0..line.len() - 1].split_once('{').unwrap();
    let rules = io::tokenize(rules, ",");
    for rule in &rules {
        if let Some((condition, dst)) = rule.split_once(':') {
            let dst_cond_action = get_action(dst);
            let mut category_arr = [0; 4];
            let cond_rule = if let Some((category, val)) = condition.split_once('<') {
                category_arr[category_to_idx(category)] = io::parse_num(val);
                Rule::Conditional(category_arr, dst_cond_action, Cmp::Less)
            } else if let Some((category, val)) = condition.split_once('>') {
                category_arr[category_to_idx(category)] = io::parse_num(val);
                Rule::Conditional(category_arr, dst_cond_action, Cmp::More)
            } else {
                panic!("No < or > found!")
            };
            map.add_to_vector_hashmap(&from, cond_rule.clone());
        } else {
            let dst_process = Rule::Process(get_action(rule));
            map.add_to_vector_hashmap(&from, dst_process.clone());
        }
    }
}

fn process_part_range<'a>(
    part_ranges: &[Range<i64>],
    map: &HashMap<&'a str, Vec<Rule<'a>>>,
    start: &'a str,
) -> i64 {
    let rules = map.get(start).unwrap();
    let mut ret = 0;
    let mut part = part_ranges.to_owned();

    for rule in rules {
        match rule {
            Rule::Conditional(category, action, cmp) => {
                let idx = (0..4).filter(|&i| category[i] != 0).collect::<Vec<_>>()[0];
                let range = match cmp {
                    Cmp::Less => Range {
                        a: MIN_INC_RANGE,
                        b: category[idx],
                    },
                    Cmp::More => Range {
                        a: category[idx] + 1,
                        b: MAX_EXC_RANGE,
                    },
                };

                let mut part_tmp = part.clone();
                part_tmp[idx] = part[idx].intersect(&range).unwrap();
                part[idx] = part[idx].difference(&range)[0];

                ret += match action {
                    Action::Accepted => part_tmp
                        .iter()
                        .map(aoc::range::Range::spread)
                        .product::<i64>(),
                    Action::Rejected => 0,
                    Action::SendTo(dst) => process_part_range(&part_tmp, map, dst),
                };
            }
            Rule::Process(action) => {
                ret += match action {
                    Action::Accepted => part.iter().map(aoc::range::Range::spread).product::<i64>(),
                    Action::Rejected => 0,
                    Action::SendTo(dst) => process_part_range(&part, map, dst),
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
            Rule::Conditional(category, action, cmp) => {
                let idx = (0..4).filter(|&i| category[i] != 0).collect::<Vec<_>>()[0];
                if (part[idx] < category[idx] && cmp == &Cmp::Less)
                    || (part[idx] > category[idx] && cmp == &Cmp::More)
                {
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
        for (i, category) in cats.iter().enumerate() {
            let (_, v) = category.split_once('=').unwrap();
            part[i] = io::parse_num(v);
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
        ranges.push(Range {
            a: MIN_INC_RANGE,
            b: MAX_EXC_RANGE,
        });
    }
    process_part_range(&ranges, &map, "in")
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, part1, true);
        common::timed(&input, part2, false);
    }
}
