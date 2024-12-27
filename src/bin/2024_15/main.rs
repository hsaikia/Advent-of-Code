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
        panic!("Invalid char {ch}");
    }
}

fn push_possible(map: &Grid<char>, pos: &(usize, usize), dir: char) -> bool {
    let sym = map.get(pos);
    if sym == '.' || sym == '#' {
        return false;
    }
    let dir_xy = get_dir(dir);

    if let Some(idx) = map.cell_in_direction(pos, &dir_xy) {
        if map.get(&idx) == '#' {
            return false;
        }
        if map.get(&idx) == '.' {
            return true;
        }
        if map.get(&idx) == '[' {
            if dir == '^' || dir == 'v' {
                return push_possible(map, &idx, dir)
                    && push_possible(map, &(idx.0, idx.1 + 1), dir);
            }
            return push_possible(map, &idx, dir);
        }
        if map.get(&idx) == ']' {
            if dir == '^' || dir == 'v' {
                return push_possible(map, &idx, dir)
                    && push_possible(map, &(idx.0, idx.1 - 1), dir);
            }
            return push_possible(map, &idx, dir);
        }
        if map.get(&idx) == 'O' {
            return push_possible(map, &idx, dir);
        }
    }
    false
}

fn push(map: &mut Grid<char>, pos: (usize, usize), dir: char) -> bool {
    let sym = map.get(&pos);
    if sym == '.' || sym == '#' {
        return false;
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
            return true;
        }
        if map.get(&idx) == '[' {
            if dir == '^' || dir == 'v' {
                if push(map, idx, dir) && push(map, (idx.0, idx.1 + 1), dir) {
                    return push(map, pos, dir);
                }
            } else if push(map, idx, dir) {
                return push(map, pos, dir);
            }
        }
        if map.get(&idx) == ']' {
            if dir == '^' || dir == 'v' {
                if push(map, idx, dir) && push(map, (idx.0, idx.1 - 1), dir) {
                    push(map, pos, dir);
                }
            } else if push(map, idx, dir) {
                return push(map, pos, dir);
            }
        }
        if map.get(&idx) == 'O' && push(map, idx, dir) {
            return push(map, pos, dir);
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

    for ch in commands.chars() {
        if ch == '\n' {
            continue;
        }

        let pos = map.positions(&'@');
        let pp = push_possible(&map, &pos[0], ch);
        if pp {
            push(&mut map, pos[0], ch);
        }
    }

    let mut ans = 0;
    for pos in map.positions(if PART == 1 { &'O' } else { &'[' }) {
        ans += pos.0 * 100 + pos.1;
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
