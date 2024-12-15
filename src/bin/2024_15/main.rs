use aoc::{common, grid::Grid};
use itertools::Itertools;

fn get_dir(ch: char) -> (i32, i32) {
    if ch == '>' {
        (0, 1)
    } else if ch == 'v' {
        (1, 0)
    } else if ch == '<' {
        (0, -1)
    } else if ch == '^' {
        (-1, 0)
    } else {
        panic!("Invalid char {}", ch);
    }
}

fn possible_steps(map: &Grid<char>, pos: &(usize, usize), dir: char) -> usize {
    let sym = map.get(pos);
    if sym == '.' || sym == '#' {
        return 0;
    }
    let dir_xy = get_dir(dir);

    if let Some(idx) = map.cell_in_direction(pos, &dir_xy) {
        if map.get(&idx) == '#' {
            return 0;
        }
        if map.get(&idx) == '.' {
            return 1 + possible_steps(map, &idx, dir);
        }
        if map.get(&idx) == '[' {
            if dir == '^' || dir == 'v' {
                return possible_steps(map, &idx, dir).min(possible_steps(
                    map,
                    &(idx.0, idx.1 + 1),
                    dir,
                ));
            }
            return possible_steps(map, &idx, dir);
        }
        if map.get(&idx) == ']' {
            if dir == '^' || dir == 'v' {
                return possible_steps(map, &idx, dir).min(possible_steps(
                    map,
                    &(idx.0, idx.1 - 1),
                    dir,
                ));
            }
            return possible_steps(map, &idx, dir);
        }
        if map.get(&idx) == 'O' {
            return possible_steps(map, &idx, dir);
        }
    }
    0
}

fn push(map: &mut Grid<char>, pos: (usize, usize), dir: char, steps: usize) -> bool {
    let sym = map.get(&pos);
    if sym == '.' || sym == '#' {
        return false;
    }
    if steps == 0 {
        return true;
    }
    let dir_xy = get_dir(dir);

    if let Some(idx) = map.cell_in_direction(&pos, &dir_xy) {
        if map.get(&idx) == '#' {
            return false;
        }
        if map.get(&idx) == '.' {
            let sym = map.get(&pos);
            map.set(&idx, sym);
            map.set(&pos, '.');
            return push(map, idx, dir, steps - 1);
        }
        if map.get(&idx) == '[' {
            if dir == '^' || dir == 'v' {
                if push(map, idx, dir, steps) && push(map, (idx.0, idx.1 + 1), dir, steps) {
                    return push(map, pos, dir, steps);
                }
            } else if push(map, idx, dir, steps) {
                return push(map, pos, dir, steps);
            }
        }
        if map.get(&idx) == ']' {
            if dir == '^' || dir == 'v' {
                if push(map, idx, dir, steps) && push(map, (idx.0, idx.1 - 1), dir, steps) {
                    push(map, pos, dir, steps);
                }
            } else if push(map, idx, dir, steps) {
                return push(map, pos, dir, steps);
            }
        }
        if map.get(&idx) == 'O' && push(map, idx, dir, steps) {
            return push(map, pos, dir, steps);
        }
    }

    true
}

fn solve<const PART: usize>(input: &str) -> usize {
    let batches: Vec<&str> = input.split("\n\n").collect();
    let transformed: String = if PART == 2 {
        batches[0]
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| {
                        if c == '.' {
                            ".."
                        } else if c == '@' {
                            "@."
                        } else if c == 'O' {
                            "[]"
                        } else {
                            "##"
                        }
                    })
                    .join("")
            })
            .join("\n")
    } else {
        batches[0].to_string()
    };
    let mut map = Grid::from_str(&transformed, |c| c);
    let commands = batches[1];

    //map.print();

    for ch in commands.chars() {
        if ch == '\n' {
            continue;
        }

        let pos = map.positions('@');
        let ps = possible_steps(&map, &pos[0], ch);
        push(&mut map, pos[0], ch, ps.min(1));
    }

    let mut ans = 0;
    for pos in map.positions(if PART == 1 { 'O' } else { '[' }) {
        ans += pos.0 * 100 + pos.1
    }
    ans
}
fn main() {
    let input = common::get_input();
    //println!("{input:?}");
    common::timed(&input, solve::<1>, true);
    common::timed(&input, solve::<2>, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "##########\n#..O..O.O#\n#......O.#\n#.OO..O.O#\n#..O@..O.#\n#O#..O...#\n#O..O..O.#\n#.OO.O.OO#\n#....O...#\n##########\n\n<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\nvvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\nv^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(solve::<1>(sample_input), 10092);
        assert_eq!(solve::<2>(sample_input), 9021);
    }
}
