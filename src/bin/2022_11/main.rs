use std::collections::VecDeque;

const MOD_SAMPLE: usize = 23 * 19 * 13 * 17;
const MOD_INPUT: usize = 2 * 7 * 13 * 3 * 19 * 5 * 11 * 17;

enum WorryReducer {
    Part1(usize),
    Part2(usize),
}

#[derive(Debug)]
struct Monkey {
    // Starting items
    items: VecDeque<usize>,
    // Operation
    op: fn(usize) -> usize,
    // Test to throw to another monkey
    test: fn(usize) -> usize,
    times_inspected: usize,
}

impl Monkey {
    fn throw(&mut self, worry_reducer: &WorryReducer) -> Option<(usize, usize)> {
        let x = self.items.pop_front();
        if let Some(mut worry) = x {
            self.times_inspected += 1;
            worry = (self.op)(worry);

            match worry_reducer {
                WorryReducer::Part1(divider) => {
                    worry = (worry - worry % divider) / divider;
                }
                WorryReducer::Part2(modulus) => {
                    worry %= modulus;
                }
            }

            return Some(((self.test)(worry), worry));
        }
        None
    }

    fn catch(&mut self, item: usize) {
        self.items.push_back(item);
    }
}

fn input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![50, 70, 89, 75, 66, 66].into(),
            op: |x| x * 5,
            test: |x| {
                if x % 2 == 0 {
                    2
                } else {
                    1
                }
            },
            times_inspected: 0,
        },
        Monkey {
            items: vec![85].into(),
            op: |x| x.pow(2),
            test: |x| {
                if x % 7 == 0 {
                    3
                } else {
                    6
                }
            },
            times_inspected: 0,
        },
        Monkey {
            items: vec![66, 51, 71, 76, 58, 55, 58, 60].into(),
            op: |x| x + 1,
            test: |x| {
                if x % 13 == 0 {
                    1
                } else {
                    3
                }
            },
            times_inspected: 0,
        },
        Monkey {
            items: vec![79, 52, 55, 51].into(),
            op: |x| x + 6,
            test: |x| {
                if x % 3 == 0 {
                    6
                } else {
                    4
                }
            },
            times_inspected: 0,
        },
        Monkey {
            items: vec![69, 92].into(),
            op: |x| x * 17,
            test: |x| {
                if x % 19 == 0 {
                    7
                } else {
                    5
                }
            },
            times_inspected: 0,
        },
        Monkey {
            items: vec![71, 76, 73, 98, 67, 79, 99].into(),
            op: |x| x + 8,
            test: |x| {
                if x % 5 == 0 {
                    0
                } else {
                    2
                }
            },
            times_inspected: 0,
        },
        Monkey {
            items: vec![82, 76, 69, 69, 57].into(),
            op: |x| x + 7,
            test: |x| {
                if x % 11 == 0 {
                    7
                } else {
                    4
                }
            },
            times_inspected: 0,
        },
        Monkey {
            items: vec![65, 79, 86].into(),
            op: |x| x + 5,
            test: |x| {
                if x % 17 == 0 {
                    5
                } else {
                    0
                }
            },
            times_inspected: 0,
        },
    ]
}

fn sample_input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![79, 98].into(),
            op: |x| x * 19,
            test: |x| {
                if x % 23 == 0 {
                    2
                } else {
                    3
                }
            },
            times_inspected: 0,
        },
        Monkey {
            items: vec![54, 65, 75, 74].into(),
            op: |x| x + 6,
            test: |x| {
                if x % 19 == 0 {
                    2
                } else {
                    0
                }
            },
            times_inspected: 0,
        },
        Monkey {
            items: vec![79, 60, 97].into(),
            op: |x| x.pow(2),
            test: |x| {
                if x % 13 == 0 {
                    1
                } else {
                    3
                }
            },
            times_inspected: 0,
        },
        Monkey {
            items: vec![74].into(),
            op: |x| x + 3,
            test: |x| {
                if x % 17 == 0 {
                    0
                } else {
                    1
                }
            },
            times_inspected: 0,
        },
    ]
}

fn simulate(monkeys: &mut Vec<Monkey>, worry_reducer: &WorryReducer, rounds: usize) {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some((id, item)) = monkeys[i].throw(worry_reducer) {
                monkeys[id].catch(item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.times_inspected.cmp(&a.times_inspected));
    println!(
        "Answer with {} monkeys = {}",
        monkeys.len(),
        monkeys[0].times_inspected * monkeys[1].times_inspected
    );
}

fn part1() {
    let mut monkeys = sample_input();
    simulate(&mut monkeys, &WorryReducer::Part1(3), 20);

    let mut monkeys = input();
    simulate(&mut monkeys, &WorryReducer::Part1(3), 20);
}

fn part2() {
    let mut monkeys = sample_input();
    simulate(&mut monkeys, &WorryReducer::Part2(MOD_SAMPLE), 10000);

    let mut monkeys = input();
    simulate(&mut monkeys, &WorryReducer::Part2(MOD_INPUT), 10000);
}

fn main() {
    part1();
    part2();
}
