const INPUT: [(&str, &str, (usize, usize)); 2] = [
    (
        "Sample Input",
        include_str!("sample_input.txt"),
        (71530, 940200),
    ),
    (
        "Input",
        include_str!("input.txt"),
        (53916768, 250133010811025),
    ),
];

fn solve(t: usize, d: usize) -> usize {
    (0..t).map(|i| i * (t - i)).filter(|x| *x > d).count()
}

fn part1(input: &str) {
    let lines = input.split('\n').collect::<Vec<_>>();
    let ts = lines[0]
        .split(' ')
        .skip(1)
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let ds = lines[1]
        .split(' ')
        .skip(1)
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut ans: usize = 1;
    for (t, d) in ts.iter().zip(ds.iter()) {
        let num_ways = solve(*t, *d);
        ans *= num_ways;
    }

    println!("Part 1 Ans {}", ans);
}

fn part2(t: usize, d: usize) {
    let num_ways = solve(t, d);
    println!("Part 2 Ans {}", num_ways);
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        part1(input.1);
        part2(input.2 .0, input.2 .1);
    }
}
