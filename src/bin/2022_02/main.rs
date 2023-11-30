const INPUT: [(&str, &str); 2] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

fn idx(x: &str) -> i32 {
    match x {
        "A" | "X" => 0,
        "B" | "Y" => 1,
        "C" | "Z" => 2,
        _ => panic!("Problem with Input!"),
    }
}

fn part1(input_lines: &str) {
    let mut score = 0;

    for line in input_lines.split('\n') {
        let idx: Vec<i32> = line.split(' ').map(idx).collect();
        score += idx[1] + 1;
        if idx[1] == (idx[0] + 1) % 3 {
            score += 6;
        } else if idx[0] == idx[1] {
            score += 3;
        }
    }

    println!("Part 1 Answer : {score}");
}

fn part2(input_lines: &str) {
    let mut score = 0;

    for line in input_lines.split('\n') {
        let idx: Vec<i32> = line.split(' ').map(idx).collect();
        if idx[1] == 1 {
            score += 3;
            score += idx[0] + 1;
        } else if idx[1] == 2 {
            score += 6;
            score += (idx[0] + 1) % 3 + 1;
        } else {
            score += (idx[0] + 2) % 3 + 1;
        }
    }

    println!("Part 2 Answer : {score}");
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        part1(input.1);
        part2(input.1);
    }
}
