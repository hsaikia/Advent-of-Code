use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
pub struct AOCError;

pub fn tokenize<'a>(line: &'a str, separator: &str) -> Vec<&'a str> {
    line.split(separator)
        .filter(|s| !s.trim().is_empty())
        .collect::<_>()
}

pub fn parse_num<T: FromStr>(token: &str) -> Result<T, AOCError> {
    token
        .chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<String>()
        .parse::<T>()
        .map_err(|_| AOCError)
}

pub fn line_batches(input: &str) -> Vec<Vec<&str>> {
    input
        .split("\n\n")
        .map(|x| x.lines().collect::<Vec<_>>())
        .collect_vec()
}
