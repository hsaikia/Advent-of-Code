use std::collections::{HashMap, HashSet, VecDeque};

use aoc::{
    common,
    grid::{CellIndex, Grid},
};

const MAX_STEPS: usize = 1000;
const MAX_STEPS_PART2: usize = 26501365;
const ODD_EVEN: [usize; 2] = [7509, 7566];

fn part1(g: &Grid<bool>, start: CellIndex) -> usize {
    solve(g, start, 64).0
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct InfPos {
    idx: CellIndex,
    gx: i32,
    gy: i32,
}

#[allow(dead_code)]
fn odd_even_max_spaces(g: &Grid<bool>, start: CellIndex) -> (usize, usize) {
    let mut odd = 0;
    let mut even = 0;
    for max_steps in (1..500).step_by(2) {
        let (_, grid_filled) = solve(g, start, max_steps);
        odd = odd.max(*grid_filled.get(&(0, 0)).unwrap());
    }
    for max_steps in (2..500).step_by(2) {
        let (_, grid_filled) = solve(g, start, max_steps);
        even = even.max(*grid_filled.get(&(0, 0)).unwrap());
    }
    (odd, even)
}

#[allow(dead_code)]
fn expand_series() -> (usize, usize, usize) {
    let step_series = [129, 260, 391, 522, 653, 784];
    let odd_even_filled_grid_series = [1, 4, 8, 12, 16, 20];
    // Differences converge to 131
    // Speed of the number of filled grids expands by 4 with every 131 steps
    let mut plots = 0;
    let mut steps = 0;

    for spread in 0..step_series.len() {
        // Since the MAX_STEPS for Part2 is odd, the center (0,0) grid has odd parity
        plots += ODD_EVEN[spread % 2] * odd_even_filled_grid_series[spread];
        steps += step_series[spread];
    }

    let mut last_grid_num = *odd_even_filled_grid_series.last().unwrap();
    for spread in step_series.len().. {
        last_grid_num += 4;

        if steps + 131 > MAX_STEPS_PART2 {
            return (spread, steps, plots);
        }

        plots += ODD_EVEN[spread % 2] * last_grid_num;
        steps += 131;
    }

    (0, 0, 0)
}

fn part2(g: &Grid<bool>, start: CellIndex) -> usize {
    // let (sp, st, pl) = expand_series();
    // println!("Spread {} Steps {} Plots {}", sp, st, pl);
    // println!("Steps remaining {}", MAX_STEPS_PART2 - st);
    // pl
    solve_quadratic(g, start)
}

fn solve_quadratic(g: &Grid<bool>, start: CellIndex) -> usize {
    // Find the number of steps from ROW_SIZE/2 to MAX_STEPS in steps of ROW_SIZE
    for max_steps in (g.rows / 2..MAX_STEPS).step_by(g.rows) {
        let (val, _filled_grids) = solve(g, start, max_steps);
        println!("In {} steps, plots visited {}", max_steps, val);
    }

    // This computation is in addition to what was seen in find_sequence and expand_series
    // The number of filled grids are grow quadratically with step size 131 = which is the length/width of the grid
    // However there are some unfilled grids, and following that sequence of steps leads to a remainder of 77 steps.
    // We compute the 26501365 mod 131 which is 65. 26501365 = 131 * 202300.
    // It is very ugly that the numbers line up with the input dimensions so much, but such is the nature of AOC puzzles :-$

    // We get the following table
    // In 65 steps, plots visited 3787
    // In 196 steps, plots visited 33976
    // In 327 steps, plots visited 94315
    // In 458 steps, plots visited 184804
    // In 589 steps, plots visited 305443
    // In 720 steps, plots visited 456232
    // In 851 steps, plots visited 637171
    // In 982 steps, plots visited 848260

    // The differences are linear!

    // The first sequence is the border of (0,0), the second the border of grids of cardinality 1, the third of 2 and so on.
    // We see that the plots grow according to the square of the steps (which is understandable, since the plots represent area, and steps represent a radius)

    // solving for y = ax^2 + bx + c, with x as the cardinality and y as the number of plots visited
    // y = 15075x^2 + 15114x + 3787. We verify that the equation is correct.

    let max_cardinality = (MAX_STEPS_PART2 - 65) / 131;
    15075 * max_cardinality * max_cardinality + 15114 * max_cardinality + 3787
}

#[allow(dead_code)]
fn find_sequence(g: &Grid<bool>, start: CellIndex) {
    // let (odd, even) = odd_even_max_spaces(g, start);

    // println!(
    //     "Total garden cells ({}X{}) Odd Max {} Even Max {}",
    //     g.rows,
    //     g.cols,
    //     odd,even
    // );

    let max_steps_curr = 1;
    let mut steps_to_fill: Vec<usize> = Vec::new();
    let filled_grid_sizes = [1, 4, 8, 12, 16, 20];
    for (grid_spread, grid_size) in filled_grid_sizes.iter().enumerate() {
        for max_steps in max_steps_curr..MAX_STEPS {
            let global_parity = max_steps % 2;
            // Solve until the (0,0) grid is filled, and then the entire 9 cell grid and the 25 cell grid
            let (val, filled_grids) = solve(g, start, max_steps);

            let mut grids = 0;
            for (k, v) in filled_grids.iter() {
                let manhattan_dist = (k.0.unsigned_abs() + k.1.unsigned_abs()) as usize;
                let curr_parity = manhattan_dist % 2;
                let max_filled_cells_for_grid = if (curr_parity + global_parity) % 2 == 0 {
                    ODD_EVEN[1]
                } else {
                    ODD_EVEN[0]
                };

                if manhattan_dist == grid_spread && *v == max_filled_cells_for_grid {
                    grids += 1;
                }
            }

            if grids == *grid_size {
                steps_to_fill.push(max_steps);
                break;
            }

            println!(
                "Solving for spread {}. In exactly {} steps, he can reach any of {} garden plots.",
                grid_spread, max_steps, val
            );
        }
    }

    println!("{:?}", steps_to_fill);
}

fn solve_parts<const PART1: bool>(input: &str) -> usize {
    let g = Grid::from_str(input, |c| c != '#');
    let start: (usize, usize) = input
        .lines()
        .enumerate()
        .filter_map(|(row_idx, cols)| cols.find('S').map(|col_idx| (row_idx, col_idx)))
        .collect::<Vec<_>>()[0];
    if PART1 {
        return part1(&g, start);
    }
    part2(&g, start)
}

fn main() {
    let input = common::get_input();
    //common::timed(&input, solve_parts::<true>, true);
    common::timed(&input, solve_parts::<false>, false);
}

fn solve(
    g: &Grid<bool>,
    start: CellIndex,
    max_steps: usize,
) -> (usize, HashMap<(i32, i32), usize>) {
    let mut q: VecDeque<(InfPos, usize)> = VecDeque::new();
    q.push_back((
        InfPos {
            idx: start,
            gx: 0,
            gy: 0,
        },
        0,
    ));

    let mut seen: HashSet<InfPos> = HashSet::new();
    let mut ans = 0;

    let mut grid_indices_filled: HashMap<(i32, i32), usize> = HashMap::new();

    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    while !q.is_empty() {
        let (pos, steps) = q.pop_front().unwrap();

        if seen.contains(&pos) || steps > max_steps {
            continue;
        }

        seen.insert(pos);

        if steps % 2 == max_steps % 2 {
            grid_indices_filled
                .entry((pos.gx, pos.gy))
                .and_modify(|v| *v += 1)
                .or_insert(1);
            ans += 1;
        }

        let adj_4_inf: [CellIndex; 4] = [
            (((pos.idx.0 + g.rows - 1) % g.rows), pos.idx.1),
            (pos.idx.0, ((pos.idx.1 + 1) % g.cols)),
            (pos.idx.0, ((pos.idx.1 + g.cols - 1) % g.cols)),
            (((pos.idx.0 + 1) % g.rows), pos.idx.1),
        ];
        // println!(
        //     "For {:?} [{}, {}] neighbors {:?} steps {:?}",
        //     pos.idx, pos.gx, pos.gy, adj_4_inf, steps
        // );

        for nn in &adj_4_inf {
            if g.get(nn) {
                let mut gx = pos.gx;
                let mut gy = pos.gy;

                if pos.idx.0 == 0 && nn.0 == g.rows - 1 {
                    gx -= 1;
                    min_x = min_x.min(gx);
                } else if pos.idx.0 == g.rows - 1 && nn.0 == 0 {
                    gx += 1;
                    max_x = max_x.max(gx);
                }

                if pos.idx.1 == 0 && nn.1 == g.cols - 1 {
                    gy -= 1;
                    min_y = min_y.min(gy);
                } else if pos.idx.1 == g.cols - 1 && nn.1 == 0 {
                    gy += 1;
                    max_y = max_y.max(gy);
                }

                let pos_next = InfPos { idx: *nn, gx, gy };
                q.push_back((pos_next, steps + 1))
            }
        }
    }

    (ans, grid_indices_filled)
}
