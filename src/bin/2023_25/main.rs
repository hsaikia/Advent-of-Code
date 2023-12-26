use std::collections::{HashMap, HashSet, VecDeque};

use aoc::{
    common::{self, HashMapVector},
    io,
};
use itertools::Itertools;
use rand::Rng;

fn bfs_cluster(node: usize, map: &HashMap<usize, Vec<usize>>, clusters: &mut Vec<Option<usize>>) {
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

fn bfs_proper(start: usize, end: usize, map: &HashMap<usize, Vec<usize>>) -> Option<Vec<usize>> {
    let mut q: VecDeque<(usize, Vec<usize>)> = VecDeque::new();
    q.push_back((start, vec![start]));

    let mut seen: HashSet<usize> = HashSet::new();

    while !q.is_empty() {
        let (node, path) = q.pop_front().unwrap();

        if node == end {
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

fn part1_proper(input: &str) -> usize {
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

    for (k, v) in map.iter() {
        //println!("{} => {:?} | {}", k, v, v.len());
        idxs.insert(k, idxs.len());
        nodes.push(k);
        // let k_map = bfs(k, &map);
        // println!("{:?}", k_map);
    }

    let mut idx_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (k, v) in map.iter() {
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
        println!("Attempt {}", att);

        let s = rng.gen_range(0..n);
        let e = rng.gen_range(0..n);

        if s == e {
            continue;
        }

        println!("S {} E {}", nodes[s], nodes[e]);

        let mut idx_map_tmp = idx_map.clone();

        for p in 0..3 {
            if let Some(path) = bfs_proper(s, e, &idx_map_tmp) {
                println!("Path {:?}", path.iter().map(|v| nodes[*v]).join(","));
                for i in 0..path.len() - 1 {
                    println!(
                        "Path {}. Removing Edge {}-{}",
                        p,
                        nodes[path[i]],
                        nodes[path[i + 1]]
                    );
                    idx_map_tmp
                        .entry(path[i])
                        .and_modify(|v| v.retain(|x| *x != path[i + 1]));
                    idx_map_tmp
                        .entry(path[i + 1])
                        .and_modify(|v| v.retain(|x| *x != path[i]));
                }
            } else {
                continue;
            }
        }

        let mut clusters = (0..n).into_iter().map(|_| None).collect_vec();
        bfs_cluster(s, &idx_map_tmp, &mut clusters);
        bfs_cluster(e, &idx_map_tmp, &mut clusters);

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
    let input = common::get_input();
    common::timed(&input, part1_proper, true);
    //common::timed(&input, part2, false);
}

fn does_split(
    map: &HashMap<usize, Vec<usize>>,
    wires: &Vec<(usize, usize)>,
    nodes: &Vec<&str>,
    //) -> Option<(Vec<usize>, Vec<usize>)> {
) -> Option<usize> {
    let mut conn = Vec::new();
    for _ in 0..map.len() {
        conn.push(None);
    }

    // Make new map with wires removed
    let mut map_new = map.clone();
    for wire in wires {
        map_new
            .entry(wire.0)
            .and_modify(|v| v.retain_mut(|x| *x != wire.1));
        map_new
            .entry(wire.1)
            .and_modify(|v| v.retain_mut(|x| *x != wire.0));
    }

    let mut clusters = Vec::new();
    for i in 0..map.len() {
        if conn[i].is_none() {
            clusters.push(i);
            bfs_cluster(i, &map_new, &mut conn);
        }
    }
    //println!("Split into {} groups. ", conn.iter().unique().count());

    if conn.iter().unique().count() == 2 {
        let mut ans = 1;
        for cluster in clusters {
            ans *= conn
                .iter()
                .enumerate()
                .filter_map(|(i, x)| if cluster == x.unwrap() { Some(i) } else { None })
                .count();
        }
        return Some(ans);
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

    for (k, v) in map.iter() {
        //println!("{} => {:?} | {}", k, v, v.len());
        idxs.insert(k, idxs.len());
        nodes.push(k);
        // let k_map = bfs(k, &map);
        // println!("{:?}", k_map);
    }

    let mut idx_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (k, v) in map.iter() {
        idx_map.insert(
            *idxs.get(k).unwrap(),
            v.iter()
                .map(|vv| *idxs.get(vv).unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let edges = idx_map
        .iter()
        .flat_map(|(k, v)| {
            v.iter()
                .filter_map(|x| if k < x { Some((*k, *x)) } else { None })
                .collect::<Vec<_>>()
        })
        .collect_vec();

    let mut c = 0;
    for i in 0..edges.len() {
        for j in i + 1..edges.len() {
            for k in j + 1..edges.len() {
                c += 1;
                print!("Checking #{} ", c);
                let wires = vec![edges[i], edges[j], edges[k]];
                for wire in &wires {
                    print!("{}-{} | ", nodes[wire.0], nodes[wire.1]);
                }
                println!();

                if let Some(ans) = does_split(&idx_map, &wires, &nodes) {
                    return ans;
                }
            }
        }
    }
    0
}

