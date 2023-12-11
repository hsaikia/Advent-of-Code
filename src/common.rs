use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash},
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
