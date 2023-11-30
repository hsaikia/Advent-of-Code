const INPUT: [(&str, &str); 2] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

fn part1(input: &str) {
    let mut curr: usize = 0;
    let mut best: usize = 0;

    for line in input.split('\n') {
        if line.is_empty() {
            best = best.max(curr);
            curr = 0;
        } else {
            curr += line.parse::<usize>().unwrap();
        }
    }

    println!("Part 1 Answer : {best}");
}

fn part2(input: &str) {
    let mut curr: usize = 0;
    let mut best: Vec<usize> = Vec::new();

    for line in input.split('\n') {
        if line.is_empty() {
            best.push(curr);
            best.sort_by(|a, b| b.cmp(a));
            best.truncate(3);
            curr = 0;
        } else {
            curr += line.parse::<usize>().unwrap();
        }
    }

    println!("Part 2 Answer : {}", best.iter().sum::<usize>());
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        part1(input.1);
        part2(input.1);
    }
}
