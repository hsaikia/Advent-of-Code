const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const INPUT: [(&str, &str); 3] = [
    ("Sample Input", include_str!("sample_input.txt")),
    ("Sample Input", include_str!("sample_input2.txt")),
    ("Input", include_str!("input.txt")),
];

fn solve(input: &str, all : bool) {
    let mut ans: usize = 0;

    let mut digit_map : Vec<(String, usize)> = Vec::new();
    for i in 1..=9 {
        let s = i.to_string();
        digit_map.push((s, i));
    }

    if all {
        for (i, digit) in DIGITS.iter().enumerate() {
            digit_map.push((digit.to_string(), i + 1));
        }
    }

    for line in input.split('\n') {
        let mut first = (input.len(), None);
        let mut last = (0, None);

        for (x, i) in &digit_map {
            if let Some(idx) = line.find(x) {
                if first.0 >= idx {
                    first.0 = idx;
                    first.1 = Some(i);
                }
            }
        }

        for (x, i) in &digit_map {
            if let Some(idx) = line.rfind(x) {
                if last.0 <= idx {
                    last.0 = idx;
                    last.1 = Some(i);
                }
            }
        }

        if first.1.is_none() || last.1.is_none() {
            println!("Some Inputs Invalid..");
            continue;
        }

        //println!("{} => {} and {}", line, first.1.unwrap(), last.1.unwrap());

        ans += 10 * first.1.unwrap();
        ans += last.1.unwrap();
    }

    println!("Answer with all digits ({}) : {}", all, ans);
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        solve(input.1, false);
        solve(input.1, true);
    }
}
