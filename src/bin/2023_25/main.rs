use std::collections::{HashMap, HashSet, VecDeque};

use aoc::{
    common::{self, HashMapVector},
    io,
};
use itertools::Itertools;
use rand::Rng;

fn bfs_cluster(node: usize, map: &HashMap<usize, Vec<usize>>, clusters: &mut [Option<usize>]) {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((node, node));

    while !q.is_empty() {
        let (f, cl) = q.pop_front().unwrap();

        if clusters[f].is_some() {
            continue;
        }

        clusters[f] = Some(cl);
        if let Some(v) = map.get(&f) {
            for x in v {
                q.push_back((*x, cl));
            }
        }
    }
}

fn bfs_path(start: usize, dst: usize, map: &HashMap<usize, Vec<usize>>) -> Option<Vec<usize>> {
    let mut q: VecDeque<(usize, Vec<usize>)> = VecDeque::new();
    q.push_back((start, vec![start]));

    let mut seen: HashSet<usize> = HashSet::new();

    while !q.is_empty() {
        let (node, path) = q.pop_front().unwrap();

        if node == dst {
            return Some(path);
        }

        if seen.contains(&node) {
            continue;
        }
        seen.insert(node);

        if let Some(v) = map.get(&node) {
            for x in v {
                let mut path_tmp = path.clone();
                path_tmp.push(*x);
                q.push_back((*x, path_tmp));
            }
        }
    }

    None
}

fn part1(input: &str) -> usize {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (from, dsts) = line.split_once(": ").unwrap();
        let dsts = io::tokenize(dsts, " ");
        for dst in dsts {
            map.add_to_vector_hashmap(&from, dst);
            map.add_to_vector_hashmap(&dst, from);
        }
    }

    let mut idxs: HashMap<&str, usize> = HashMap::new();
    let mut nodes: Vec<&str> = Vec::new();

    for k in map.keys() {
        idxs.insert(k, idxs.len());
        nodes.push(k);
    }

    let mut idx_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (k, v) in &map {
        idx_map.insert(
            *idxs.get(k).unwrap(),
            v.iter()
                .map(|vv| *idxs.get(vv).unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let mut rng = rand::thread_rng();
    let n = map.len();
    let mut att = 0;
    loop {
        att += 1;
        println!("Attempt {att}");

        let start = rng.gen_range(0..n);
        let dst = rng.gen_range(0..n);

        if start == dst {
            continue;
        }

        let mut idx_map_tmp = idx_map.clone();

        for _p in 0..3 {
            if let Some(path) = bfs_path(start, dst, &idx_map_tmp) {
                for i in 0..path.len() - 1 {
                    idx_map_tmp
                        .entry(path[i])
                        .and_modify(|v| v.retain(|x| *x != path[i + 1]));
                    idx_map_tmp
                        .entry(path[i + 1])
                        .and_modify(|v| v.retain(|x| *x != path[i]));
                }
            }
        }

        let mut clusters = (0..n).map(|_| None).collect_vec();
        bfs_cluster(start, &idx_map_tmp, &mut clusters);
        bfs_cluster(dst, &idx_map_tmp, &mut clusters);

        let cluster_ids = clusters.iter().unique().collect_vec();

        if cluster_ids.len() != 2 {
            continue;
        }

        let mut ans = 1;
        for cl in cluster_ids {
            ans *= clusters.iter().filter(|&x| x == cl).count();
        }

        return ans;
    }
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, part1, true);
    }
}
