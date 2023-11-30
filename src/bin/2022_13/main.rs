const INPUT: [(&str, &str); 1] = [
    ("Sample Input", include_str!("sample_input.txt")),
    //("Input", include_str!("input.txt")),
];

fn check(str1: &str, str2: &str) -> bool {
    println!("Checking {} and {}", str1, str2);

    if str1 == str2 {
        return true;
    }

    if str1.find('[').is_none()
        && str1.find(']').is_none()
        && str1.find(',').is_none()
        && str2.find('[').is_none()
        && str2.find(']').is_none()
        && str2.find(',').is_none()
    {
        let n1 = str1.parse::<usize>().unwrap();
        let n2 = str2.parse::<usize>().unwrap();

        return n1 <= n2;
    }

    let l1 = str1.len();
    let l2 = str2.len();

    if l1 == 0 && l2 > 0 {
        return true;
    }

    let mut str1 = str1;
    let mut str2 = str2;
    if str1.starts_with('[') && str1.chars().nth(l1 - 1).unwrap() == ']' {
        str1 = &str1[1..l1 - 1];
    }
    if str2.starts_with('[') && str2.chars().nth(l2 - 1).unwrap() == ']' {
        str2 = &str2[1..l2 - 1];
    }
    let elems1 = str1.split(',').collect::<Vec<&str>>();
    let elems2 = str2.split(',').collect::<Vec<&str>>();

    let l_min = elems1.len().min(elems2.len());

    for i in 0..l_min {
        let res = check(elems1[i], elems2[i]);
        if !res {
            return false;
        }
    }

    true
}

fn part1(input: &str) {
    let mut idx = 0;
    let mut ans = 0;
    let input_lines = input.split('\n').collect::<Vec<_>>();
    while idx < input_lines.len() {
        println!("Checking Pair {}", idx / 2 + 1);
        if check(input_lines[idx], input_lines[idx + 1]) {
            println!("Correct Order");
            ans += 1;
        }
        idx += 3;
    }
    println!("Part 1 Answer is {}", ans);
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        part1(input.1);
        //part2(input.1);
    }
}
