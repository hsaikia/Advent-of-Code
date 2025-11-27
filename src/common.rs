use core::fmt;
use std::{
    collections::{HashMap, HashSet},
    env,
    hash::{BuildHasher, Hash},
    time::Instant,
};

use crate::io;

pub trait HashMapVector<K: Copy + Hash + Eq + PartialEq, V> {
    fn add_to_vector_hashmap(&mut self, key: &K, value: V);
    fn contains(&self, key: &K, value: &V) -> bool
    where
        V: PartialEq;
}

impl<K: Copy + Hash + Eq + PartialEq, V, S: BuildHasher> HashMapVector<K, V>
    for HashMap<K, Vec<V>, S>
{
    fn add_to_vector_hashmap(&mut self, key: &K, value: V) {
        match self.get_mut(key) {
            Some(vals) => {
                vals.push(value);
            }
            None => {
                self.insert(*key, vec![value]);
            }
        }
    }

    fn contains(&self, key: &K, value: &V) -> bool
    where
        V: PartialEq,
    {
        if let Some(v) = self.get(key) {
            return v.contains(value);
        }
        false
    }
}

pub fn minmax<T: Ord + Copy>(x: &T, y: &T) -> (T, T) {
    (*x.min(y), *x.max(y))
}

pub fn timed<T: core::fmt::Debug>(input: &str, f: fn(&str) -> T, part1: bool) {
    let start = Instant::now();
    println!("Part #{} Answer {:?}", if part1 { 1 } else { 2 }, f(input));
    let duration = start.elapsed();
    println!(
        "Time elapsed in Part #{} is: {:?}",
        if part1 { 1 } else { 2 },
        duration
    );
}

/// # Panics
///
/// Panics if the file is not found or cannot be read.
#[must_use]
pub fn get_input() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let filepath = if args.len() == 1 {
        let tokens = io::tokenize(&args[0], "/");
        format!("./src/bin/{}/input.txt", tokens.last().unwrap())
    } else {
        args[1].clone()
    };
    println!("Reading file {filepath}");
    if let Ok(input) = std::fs::read_to_string(filepath) {
        Some(input)
    } else {
        None
    }
}

pub struct GridDisplay {
    pub rows: Vec<String>,
}

impl fmt::Display for GridDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.rows.join("\n");
        write!(f, "\n{s}\n")
    }
}

impl fmt::Debug for GridDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.rows.join("\n");
        write!(f, "\n{s}\n")
    }
}

pub fn dedup<T: Hash + Clone + Copy + std::cmp::Eq + PartialEq>(v: &mut Vec<T>) {
    let mut set = HashSet::<T>::new();
    v.retain(|x| set.insert(*x));
}
