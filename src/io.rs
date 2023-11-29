use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

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
