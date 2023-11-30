use std::str::FromStr;

pub fn tokenize<'a>(line: &'a str, separator: &str) -> Vec<&'a str> {
    line.split(separator).collect::<_>()
}

pub fn parse_num<T: FromStr>(token: &str) -> Result<T, <T as FromStr>::Err> {
    token
        .chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<String>()
        .parse::<T>()
}
