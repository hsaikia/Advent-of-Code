use std::collections::HashMap;

use aoc::{
    common::{self, HashMapVector},
    graph::ShortestPath,
    io,
};

type Node<'a> = (&'a str, Vec<&'a str>, usize, i64);

struct Graph<'a> {
    pub connections: HashMap<&'a str, Vec<&'a str>>,
    pub flow_rate: HashMap<&'a str, i64>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Graph {
            connections: HashMap::new(),
            flow_rate: HashMap::new(),
        }
    }

    fn total_pressure(&self, opened_valves: &[&str]) -> i64 {
        opened_valves
            .iter()
            .map(|valve| self.flow_rate.get(valve).unwrap())
            .sum::<i64>()
    }

    fn all_open(&self, opened_valves: &[&str]) -> bool {
        for (valve, rate) in self.flow_rate.iter() {
            if *rate > 0 && !opened_valves.contains(valve) {
                return false;
            }
        }
        true
    }
}

impl<'a> ShortestPath<Node<'a>> for Graph<'a> {
    fn connections_and_cost(&self, node: &Node<'a>) -> Vec<(Node<'a>, i64)> {
        let mut ret: Vec<(Node, i64)> = Vec::new();
        let (curr_valve, opened_valves, elapsed_time, tot_pressure) = node;

        if elapsed_time >= &30 {
            return ret;
        }

        let curr_valve_pressure = *self.flow_rate.get(curr_valve).unwrap();
        let tot_opened_pressure = self.total_pressure(opened_valves);

        // If current valve is not opened, add it
        if curr_valve_pressure > 0 && !opened_valves.contains(curr_valve) {
            let mut opened_valves_tmp = opened_valves.clone();
            opened_valves_tmp.push(curr_valve);
            ret.push((
                (
                    *curr_valve,
                    opened_valves_tmp,
                    elapsed_time + 1,
                    tot_pressure - tot_opened_pressure - curr_valve_pressure,
                ),
                -tot_opened_pressure - curr_valve_pressure,
            ));
        }

        for valve in self.connections.get(curr_valve).unwrap() {
            // We can do to any valve, open or closed, non-zero or zero flow
            ret.push((
                (
                    *valve,
                    opened_valves.clone(),
                    elapsed_time + 1,
                    *tot_pressure - tot_opened_pressure,
                ),
                -tot_opened_pressure,
            ));
        }
        ret
    }

    fn termination_condition(&self, node: &Node) -> bool {
        let (_, opened_valves, elapsed_time, _) = node;
        self.all_open(opened_valves) && *elapsed_time >= 30
    }
}

fn part1(input: &str) -> i64 {
    let mut graph: Graph = Graph::new();
    for line in input.lines() {
        let tokens = io::tokenize(line, " ");
        for valve in tokens.iter().skip(9) {
            graph
                .connections
                .add_to_vector_hashmap(&tokens[1], &valve[0..2]);
        }
        graph
            .flow_rate
            .entry(tokens[1])
            .or_insert(io::parse_num(tokens[4]));
    }

    -graph.shortest_path(("AA", vec![], 0, 0))
}
fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
}
