use std::collections::{HashMap, VecDeque};

use aoc::{common, io};
use num::Integer;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Pulse {
    High,
    #[default]
    Low,
}

#[derive(Debug, Clone)]
struct Gate<'a> {
    mt: ModType<'a>,
}

impl<'a> Gate<'a> {
    pub fn reset(&mut self) {
        match &mut self.mt {
            ModType::Conjunction(pulse_map) => {
                for val in pulse_map.values_mut() {
                    *val = Pulse::Low;
                }
            }
            ModType::FlipFlop(state) => *state = State::Off,
            _ => {}
        }
    }
    pub fn process(&mut self, from: &'a str, pulse: &Pulse) -> Option<Pulse> {
        //println!("Receiving {:?} from {}", pulse, from);
        match &mut self.mt {
            ModType::Conjunction(pulse_map) => {
                pulse_map
                    .entry(from)
                    .and_modify(|p| *p = *pulse)
                    .or_insert(*pulse);
                //println!("{:?}", pulse_map);
                if pulse_map.values().all(|x| *x == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }

            ModType::FlipFlop(state) => match pulse {
                Pulse::Low => {
                    if *state == State::On {
                        self.mt = ModType::FlipFlop(State::Off);
                        Some(Pulse::Low)
                    } else {
                        self.mt = ModType::FlipFlop(State::On);
                        Some(Pulse::High)
                    }
                }
                Pulse::High => None,
            },

            ModType::Broadcaster => Some(*pulse),
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
enum State {
    On,
    #[default]
    Off,
}

#[derive(Debug, Clone)]
enum ModType<'a> {
    Broadcaster,
    FlipFlop(State),
    Conjunction(HashMap<&'a str, Pulse>),
}

fn part1(input: &str) -> usize {
    let mut order: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut states: HashMap<&str, Gate> = HashMap::new();

    for line in input.lines() {
        let tokens = io::tokenize(line, " -> ");
        if tokens[0] == "broadcaster" {
            states.insert(
                &tokens[0],
                Gate {
                    mt: ModType::Broadcaster,
                },
            );
            order.insert(tokens[0], io::tokenize(tokens[1], ", "));
        } else {
            if &tokens[0][0..1] == "&" {
                states.insert(
                    &tokens[0][1..],
                    Gate {
                        mt: ModType::Conjunction(HashMap::new()),
                    },
                );
            } else {
                states.insert(
                    &tokens[0][1..],
                    Gate {
                        mt: ModType::FlipFlop(State::Off),
                    },
                );
            }

            order.insert(&tokens[0][1..], io::tokenize(tokens[1], ", "));
        }
    }

    // for (key, vals) in order.iter() {
    //     println!("{} -> {:?}", key, vals);
    // }

    // Fix Conjunction maps
    for (key, vals) in order.iter() {
        for val in vals {
            if let Some(entry) = states.get_mut(val) {
                match &mut entry.mt {
                    ModType::Conjunction(map) => {
                        map.insert(key, Pulse::Low);
                    }
                    _ => (),
                }
            }
        }
    }

    // for (key, vals) in states.iter() {
    //     println!("{} -> {:?}", key, vals);
    // }

    // Hack!
    // Determine button presses for vm, lm, jd, fv highs as they all connect to zg which must output low
    let mut ans1: usize = 0;
    let mut ans2: usize = 1;
    const PART1_TIMES: usize = 1000;

    for elem in ["vm", "lm", "jd", "fv"] {
        // Reset all
        for vals in states.values_mut() {
            vals.reset();
        }

        let mut sent = [0; 2];
        let mut button_presses = 0;
        loop {
            if button_presses == PART1_TIMES {
                ans1 = sent[0] * sent[1];
            }

            //println!("\n-- Push --\n");
            button_presses += 1;
            let mut q: VecDeque<(&str, Option<Pulse>)> = VecDeque::new();
            q.push_back(("broadcaster", Some(Pulse::Low)));
            sent[0] += 1;
            let mut found_elem = false;

            while !q.is_empty() {
                let (md, opt_pulse) = q.pop_front().unwrap();
                if let Some(pulse) = opt_pulse {
                    if md == elem && pulse == Pulse::High {
                        println!("Found {} in {} presses", elem, button_presses);
                        ans2 = ans2.lcm(&button_presses);
                        found_elem = true;
                        break;
                    }

                    for dg in order.get(md).unwrap() {
                        if button_presses <= PART1_TIMES && pulse == Pulse::High {
                            sent[1] += 1
                        } else {
                            sent[0] += 1
                        }
                        //println!("{} -> ({:?}) -> {}", md, pulse, dg);

                        if let Some(dst) = states.get_mut(dg) {
                            let new_pulse = dst.process(md, &pulse);
                            q.push_back((dg, new_pulse));
                        }
                    }
                }
            }

            if found_elem {
                break;
            }
        }
    }

    println!("Part 1 {:?} Part 2 {}", ans1, ans2);
    0
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    //common::timed(&input, part2, false);
}
