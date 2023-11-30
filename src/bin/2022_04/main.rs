const INPUT: [(&str, &str); 2] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

fn part1(input_lines: &str) {
    let mut ans: usize = 0;

    for line in input_lines.split('\n') {
        let ranges: Vec<_> = line.split(',').collect();
        let idx1: Vec<_> = ranges[0]
            .split('-')
            .flat_map(|s| s.parse::<usize>())
            .collect();
        let idx2: Vec<_> = ranges[1]
            .split('-')
            .flat_map(|s| s.parse::<usize>())
            .collect();

        if (idx2[0] >= idx1[0] && idx2[1] <= idx1[1]) || (idx1[0] >= idx2[0] && idx1[1] <= idx2[1])
        {
            ans += 1;
        }
    }

    println!("Part 1 Answer : {ans}");
}

fn part2(input_lines: &str) {
    let mut ans: usize = 0;

    for line in input_lines.split('\n') {
        let ranges: Vec<_> = line.split(',').collect();
        let idx1: Vec<_> = ranges[0]
            .split('-')
            .flat_map(|s| s.parse::<usize>())
            .collect();
        let idx2: Vec<_> = ranges[1]
            .split('-')
            .flat_map(|s| s.parse::<usize>())
            .collect();

        let idx_l = idx1[0].max(idx2[0]);
        let idx_r = idx1[1].min(idx2[1]);

        if idx_l <= idx_r {
            ans += 1;
        }
    }

    println!("Part 2 Answer : {ans}");
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        part1(input.1);
        part2(input.1);
    }
}
