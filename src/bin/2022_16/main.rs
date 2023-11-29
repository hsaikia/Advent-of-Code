use aoc::graph::Graph;
use aoc::io;

use std::collections::VecDeque;

const FILES: [&str; 1] = [
    "./src/bin/2022_16/sample_input.txt",
    //"./src/bin/2022_16/input.txt",
];

fn part1(graph: &Graph<&str, u64>) {
    let mut max = 0;
    let mut queue: VecDeque<(&str, u8, u64, Vec<&str>)> = VecDeque::new();
    queue.push_back(("AA", 0, 0, vec![]));
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();

        println!("{:?}", curr);

        if curr.1 >= 30 {
            continue;
        }

        max = max.max(curr.2);

        if let Some(x) = graph.node_weights.get(curr.0) {
            if *x > 0 && !curr.3.contains(&curr.0) {
                // Open Valve
                let mut lst = curr.3.clone();
                lst.push(curr.0);
                queue.push_back((curr.0, curr.1 + 1, curr.2 + *x, lst));
            }
            for nx in graph.connections.get(curr.0).unwrap() {
                queue.push_back((nx.0, curr.1 + 1, curr.2, curr.3.clone()));
            }
        }
    }

    println!("Maximum pressure released in 30 minutes : {}", max);
}

fn main() {
    for filename in FILES {
        println!("Input file {filename}");
        if let Ok(lines) = io::read_lines(filename) {
            let input_lines = lines.flatten().collect::<Vec<String>>();
            let mut graph = Graph::<&str, u64>::new();
            for line in &input_lines {
                let tokens = io::tokenize(line, " ");
                //println!("{:?}", &tokens);
                let flow_rate = io::parse_num::<u64>(tokens[4]).unwrap();
                //println!("{}", flow_rate);
                graph.add_node_weight(tokens[1], flow_rate);
                for i in 9..tokens.len() {
                    graph.add_bidirectional_edge(tokens[1], tokens[i], 1);
                }
            }

            println!("{:?}", graph);

            part1(&graph);
        }
    }
}
