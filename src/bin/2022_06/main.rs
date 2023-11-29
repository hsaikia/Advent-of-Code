use aoc::io;

const FILES: [&str; 2] = [
    "./src/bin/2022_06/sample_input.txt",
    "./src/bin/2022_06/input.txt",
];

fn solve(input_lines: &Vec<String>, marker_size: usize) {
    for line in input_lines {
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
    for filename in FILES {
        println!("Input file {filename}");
        if let Ok(lines) = io::read_lines(filename) {
            let input_lines = lines.flatten().collect::<Vec<String>>();
            solve(&input_lines, 4);
            solve(&input_lines, 14);
        }
    }
}
