use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

pub trait ShortestPath<T: Ord + Hash + Clone + Debug> {
    fn connections_and_cost(&self, node: &T) -> Vec<(T, i64)>;

    fn termination_condition(&self, node: &T) -> bool;

    // Dijkstra's algorithm for finding the shortest path from a start node to an end node
    fn shortest_path(&self, start: T) -> i64 {
        let mut distances: HashMap<T, i64> = HashMap::new();
        let mut pq: BinaryHeap<(i64, T)> = BinaryHeap::new();

        pq.push((0, start));

        while !pq.is_empty() {
            let (dist, node) = pq.pop().unwrap();

            if self.termination_condition(&node) {
                return -dist;
            }

            let neighbors = self.connections_and_cost(&node);
            for (neighbor, cost) in neighbors {
                let new_dist = dist - cost;

                let opt_ndist = distances.get_mut(&neighbor);
                if let Some(ndist) = opt_ndist {
                    if new_dist > *ndist {
                        *ndist = new_dist;
                        pq.push((new_dist, neighbor));
                    }
                } else {
                    distances.insert(neighbor.clone(), new_dist);
                    pq.push((new_dist, neighbor));
                }
            }
        }

        i64::MAX
    }
}
