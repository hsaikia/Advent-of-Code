use std::collections::HashMap;

use aoc::{
    common::{self, HashMapVector},
    io,
};
use itertools::Itertools;

fn solve<const PART: usize>(input: &str) -> String {
    let mut conn = HashMap::new();
    let mut sccs: Vec<Vec<&str>> = Vec::new();
    for line in input.lines() {
        let p = io::tokenize(line, "-");
        conn.add_to_vector_hashmap(&p[0], p[1]);
        conn.add_to_vector_hashmap(&p[1], p[0]);
        sccs.push(p);
    }

    if PART == 1 {
        let mut three_sets = Vec::new();
        for k1 in conn.keys() {
            if !k1.starts_with('t') {
                continue;
            }

            for k2 in conn.keys() {
                if k1 == k2 {
                    continue;
                }
                if !conn.contains(k1, k2) {
                    continue;
                }

                for k3 in conn.keys() {
                    if k2 == k3 || k1 == k3 {
                        continue;
                    }

                    if !conn.contains(k1, k3) || !conn.contains(k2, k3) {
                        continue;
                    }

                    let mut tmp = [k1, k2, k3];
                    tmp.sort();
                    three_sets.push(tmp);
                }
            }
        }
        three_sets.sort();
        three_sets.dedup();
        three_sets.len().to_string()
    } else {
        loop {
            sccs.sort();
            sccs.dedup();

            let max_scc_size = sccs.iter().map(std::vec::Vec::len).max().unwrap();
            sccs.retain(|x| x.len() == max_scc_size);
            //println!("Scc Max Size {} Number SCCs {}", max_scc_size, sccs.len());
            if sccs.len() == 1 {
                break;
            }

            for scc in &mut sccs {
                for k in conn.keys() {
                    if !scc.contains(k) {
                        let all_conn = scc.iter().all(|x| conn.get(x).unwrap().contains(k));
                        if all_conn {
                            scc.push(k);
                            scc.sort_unstable();
                        }
                    }
                }
            }
        }

        sccs[0].iter().join(",")
    }
}

fn main() {
    let input = common::get_input();
    println!("{input:?}");
    common::timed(&input, solve::<1>, true);
    common::timed(&input, solve::<2>, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "kh-tc\nqp-kh\nde-cg\nka-co\nyn-aq\nqp-ub\ncg-tb\nvc-aq\ntb-ka\nwh-tc\nyn-cg\nkh-ub\nta-co\nde-co\ntc-td\ntb-wq\nwh-td\nta-ka\ntd-qp\naq-cg\nwq-ub\nub-vc\nde-ta\nwq-aq\nwq-vc\nwh-yn\nka-de\nkh-ta\nco-tc\nwh-qp\ntb-vc\ntd-yn";
        assert_eq!(solve::<1>(sample_input), "7");
        assert_eq!(solve::<2>(sample_input), "co,de,ka,ta");
    }
}
