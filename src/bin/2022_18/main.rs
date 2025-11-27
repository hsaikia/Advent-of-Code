use aoc::common;

const RANDOM_PRIME1: i32 = 22283;
const RANDOM_PRIME2: i32 = 1783;

fn index(coord: &[i32]) -> i32 {
    RANDOM_PRIME1 * coord[0] + RANDOM_PRIME2 * coord[1] + coord[2]
}

fn part1(input: &str) -> usize {
    const SIDES: [(i32, i32, i32); 6] = [
        (0, 0, 1),
        (0, 0, -1),
        (0, 1, 0),
        (0, -1, 0),
        (1, 0, 0),
        (-1, 0, 0),
    ];
    let mut ans = 0;
    let mut cube_indices: Vec<i32> = Vec::new();
    for line in input.split('\n') {
        let coords = line
            .split(',')
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        for side in &SIDES {
            let mut ncoord = coords.clone();
            ncoord[0] += side.0;
            ncoord[1] += side.1;
            ncoord[2] += side.2;
            let idx = index(&ncoord);
            if cube_indices.contains(&idx) {
                ans -= 1;
            } else {
                ans += 1;
            }
        }

        let idx = index(&coords);
        cube_indices.push(idx);
    }

    ans
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, part1, true);
    }
}
