use itertools::Itertools;
use std::{fmt::Debug, str::FromStr};

#[derive(Debug)]
pub struct AOCError;

pub fn tokenize_nums<T: FromStr>(line: &str, separator: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    tokenize(line, separator)
        .into_iter()
        .map(parse_num)
        .collect::<Vec<T>>()
}

pub fn tokenize<'a>(line: &'a str, separator: &str) -> Vec<&'a str> {
    line.split(separator)
        .filter(|s| !s.trim().is_empty())
        .collect::<_>()
}

pub fn parse_num<T: FromStr>(token: &str) -> T
where
    <T as FromStr>::Err: Debug,
{
    token
        .chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<String>()
        .parse::<T>()
        .unwrap()
}

pub fn line_batches(input: &str) -> Vec<Vec<&str>> {
    input
        .split("\n\n")
        .map(|x| x.lines().collect::<Vec<_>>())
        .collect_vec()
}
