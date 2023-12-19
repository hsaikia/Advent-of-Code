use aoc::{
    common,
    graph::ShortestPath,
    grid::{CellDir, CellIndex, Grid},
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum CardinalDirection {
    Up,
    Down,
    Right,
    Left,
}

impl CardinalDirection {
    pub fn next(&self) -> Vec<CardinalDirection> {
        match self {
            CardinalDirection::Up | CardinalDirection::Down => {
                vec![CardinalDirection::Left, CardinalDirection::Right]
            }
            CardinalDirection::Left | CardinalDirection::Right => {
                vec![CardinalDirection::Up, CardinalDirection::Down]
            }
        }
    }

    pub fn to_dir(self) -> CellDir {
        match self {
            CardinalDirection::Up => (-1, 0),
            CardinalDirection::Down => (1, 0),
            CardinalDirection::Left => (0, -1),
            CardinalDirection::Right => (0, 1),
        }
    }
}

type Hops = usize;
type Node = (CellIndex, CardinalDirection, Hops);

const START: CellIndex = (0, 0);

fn add_neighbor_in_cardinal_dir(
    ret: &mut Vec<(Node, i64)>,
    cell_index: &CellIndex,
    cardinal_dir: &CardinalDirection,
    g: &Grid<i64>,
    hops: usize,
) {
    let dir = cardinal_dir.to_dir();
    let opt_neighbor = g.cell_in_direction(cell_index.0, cell_index.1, dir.0, dir.1);

    if let Some(neighbor) = opt_neighbor {
        ret.push((
            (neighbor, *cardinal_dir, hops),
            g.get(neighbor.0, neighbor.1).unwrap(),
        ));
    }
}

struct MyGrid1(Grid<i64>);
struct MyGrid2(Grid<i64>);

impl ShortestPath<Node> for MyGrid1 {
    fn connections_and_cost(&self, node: &Node) -> Vec<(Node, i64)> {
        let (cell_index, cardinal_dir, hops) = node;
        let mut ret = Vec::new();

        for other_cardinal_dir in cardinal_dir.next() {
            add_neighbor_in_cardinal_dir(&mut ret, cell_index, &other_cardinal_dir, &self.0, 1);
        }

        if *hops < 3 {
            add_neighbor_in_cardinal_dir(&mut ret, cell_index, cardinal_dir, &self.0, hops + 1);
        }

        ret
    }

    fn termination_condition(&self, node: &Node) -> bool {
        let (cell_index, _, _) = node;
        cell_index.0 == self.0.rows - 1 && cell_index.1 == self.0.cols - 1
    }
}

impl ShortestPath<Node> for MyGrid2 {
    fn connections_and_cost(&self, node: &Node) -> Vec<(Node, i64)> {
        let (cell_index, cardinal_dir, hops) = node;
        let mut ret = Vec::new();

        if *hops < 4 {
            add_neighbor_in_cardinal_dir(&mut ret, cell_index, cardinal_dir, &self.0, hops + 1);
        } else if *hops < 10 {
            for other_cardinal_dir in cardinal_dir.next() {
                add_neighbor_in_cardinal_dir(&mut ret, cell_index, &other_cardinal_dir, &self.0, 1);
            }
            add_neighbor_in_cardinal_dir(&mut ret, cell_index, cardinal_dir, &self.0, hops + 1);
        } else {
            for other_cardinal_dir in cardinal_dir.next() {
                add_neighbor_in_cardinal_dir(&mut ret, cell_index, &other_cardinal_dir, &self.0, 1);
            }
        }

        ret
    }

    fn termination_condition(&self, node: &Node) -> bool {
        let (cell_index, _, hops) = node;
        cell_index.0 == self.0.rows - 1
            && cell_index.1 == self.0.cols - 1
            && *hops >= 4
            && *hops <= 10
    }
}

fn part1(input: &str) -> i64 {
    let g = MyGrid1(Grid::from_str(input, |c| c.to_digit(10).unwrap() as i64));
    g.shortest_path((START, CardinalDirection::Right, 0))
        .min(g.shortest_path((START, CardinalDirection::Down, 0)))
}

fn part2(input: &str) -> i64 {
    let g = MyGrid2(Grid::from_str(input, |c| c.to_digit(10).unwrap() as i64));
    g.shortest_path((START, CardinalDirection::Right, 0))
        .min(g.shortest_path((START, CardinalDirection::Down, 0)))
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    common::timed(&input, part2, false);
}
