use core::fmt;
use std::{
    collections::HashMap,
    env,
    hash::{BuildHasher, Hash},
    time::Instant,
};

pub trait HashMapVector<K: Copy + Hash + Eq + PartialEq, V> {
    fn add_to_vector_hashmap(&mut self, key: &K, value: V);
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
}

pub fn minmax<T: Ord + Copy>(x: &T, y: &T) -> (T, T) {
    (*x.min(y), *x.max(y))
}

pub fn timed<T: core::fmt::Debug>(input: &str, f: fn(&str) -> T, part1: bool) {
    let start = Instant::now();
    println!(
        "\nPart #{} Answer {:?}",
        if part1 { 1 } else { 2 },
        f(input)
    );
    let duration = start.elapsed();
    println!(
        "Time elapsed in Part #{} is: {:?}",
        if part1 { 1 } else { 2 },
        duration
    );
}

pub fn get_input() -> String {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    std::fs::read_to_string(filepath).unwrap()
}

pub struct GridDisplay {
    pub rows: Vec<String>,
}

impl fmt::Display for GridDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.rows.join("\n");
        write!(f, "\n{}\n", s)
    }
}

impl fmt::Debug for GridDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.rows.join("\n");
        write!(f, "\n{}\n", s)
    }
}
