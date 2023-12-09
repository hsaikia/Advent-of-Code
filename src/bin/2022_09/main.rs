use itertools::Itertools;

const INPUT: [(&str, &str); 3] = [
    ("Sample Input 1", include_str!("sample_input.txt")),
    ("Sample Input 2", include_str!("sample_input_2.txt")),
    ("Input", include_str!("input.txt")),
];

fn follow_knot(head_new: (i32, i32), tail_old: (i32, i32)) -> (i32, i32) {
    let mut dx = head_new.0 - tail_old.0;
    let mut dy = head_new.1 - tail_old.1;

    let dx_abs = dx.abs();
    let dy_abs = dy.abs();

    if dx_abs <= 1 && dy_abs <= 1 {
        return tail_old;
    }

    if dx_abs != 0 {
        dx /= dx_abs;
    }
    if dy_abs != 0 {
        dy /= dy_abs;
    }
    (tail_old.0 + dx, tail_old.1 + dy)
}

fn solve<const ROPE_SIZE: usize>(input_lines: &str) {
    let mut rope: [(i32, i32); ROPE_SIZE] = [(0, 0); ROPE_SIZE];
    let mut tail_history = Vec::new();
    for line in input_lines.split('\n') {
        let command = line.split(' ').collect::<Vec<&str>>();
        let steps = command[1].parse::<i32>().unwrap();
        let mut dx = 0;
        let mut dy = 0;
        match command[0] {
            "R" => dy = 1,
            "L" => dy = -1,
            "U" => dx = -1,
            "D" => dx = 1,
            _ => (),
        }

        for _ in 0..steps {
            rope[0] = (rope[0].0 + dx, rope[0].1 + dy);
            for i in 1..ROPE_SIZE {
                rope[i] = follow_knot(rope[i - 1], rope[i]);
                tail_history.push(rope[ROPE_SIZE - 1]);
            }
        }
    }
    tail_history = tail_history.into_iter().unique().collect::<Vec<_>>();
    println!(
        "Answer with Rope of size {} : {}",
        ROPE_SIZE,
        tail_history.len()
    );
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        solve::<2>(input.1);
        solve::<10>(input.1);
    }
}