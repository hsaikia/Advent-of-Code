const INPUT: [(&str, &str); 2] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Input", include_str!("input.txt")),
];

fn solve(input_lines: &str, marker_size: usize) {
    for line in input_lines.split('\n') {
        for (i, marker) in line
            .chars()
            .collect::<Vec<char>>()
            .windows(marker_size)
            .enumerate()
        {
            //println!("Checking marker : {:?}", marker);
            if (1..marker_size).any(|j| marker[j..].contains(&marker[j - 1])) {
                continue;
            }
            println!(
                "Answer for Marker Size {} : {}",
                marker_size,
                i + marker_size
            );
            break;
        }
    }
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        solve(input.1, 4);
        solve(input.1, 14);
    }
}
