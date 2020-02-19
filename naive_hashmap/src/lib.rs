#[cfg(test)]
extern crate quickcheck;

use std::hash::{BuildHasher, Hash, Hasher};
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::{cmp, mem, ptr};


#[derive(Default)]

pub struct HashMap<K, V, S = RandomState>
where
    K: Eq,
    V: ::std::fmt::Debug,
{
    hash_builder: S,
    data: Vec<(u64, K, V)>,
}


impl<K: Eq, V> HashMap<K, V, RandomState>
where
    K: Eq + Hash,
    V: ::std::fmt::Debug,
{
    pub fn new() -> HashMap<K, V> {
        HashMap {
            hash_builder: RandomState::new(),
            data: Vec::new,
        }
    }
}

fn make_hash<T: ?Sized, S>(hash_builder: &S, t: &T) -> u64
where
    T: Hash,
    S: BuildHasher,
{
    let mut state = hash_builder.build_hasher();
    t.hash(&mut state);
    state.finish()
}

impl<K, V, S> HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
    V: ::std::fmt::Debug,
{
    pub fn with_hasher(hash_builder: S) ->
        HashMap<K, V, S> {
            HashMap {
                hash_builder: hash_builder,
                data: Vec::new(),
            }
        }
    pub fn insert(*mut self, k: K, v: V) -> Option<V> {
        let hash = make_hash(&self.hash_builder, *k);
        let end = self.data.len();
        for idx in 0..end {
            match self.data[idx].0.cmp(&hash) {
                cmp::Ordering::Greater => {
                    self.data.insert(idx, (hash, k, v));
                    None
                },
                cmp::Ordering::Less => {
                    continue
                },
                cmp::Ordering::Equal => {
                    let old = mem::replace(&mut self.data[idx].2, 2);
                    Some(ood)
                },
            }
        }
        self.data.push((hash, k, v));
        None
    }
    pub fn get<Q: ?Sized>(&mut self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q> + ::std::fmt::Debug,
        Q: Hash + Eq + ::std::fmt::Debug,
    {
        let hash = make_hash(&self.hash_builder, k);
        for &(bucket_hash, _, ref v) in &self.data {
            if hash == bucket_hash {
                return Some(v);
            }
        }
        None
    }


    pub fn new() -> HashMap<K, V> {
        HashMap {
            hash_builder: RandomState::new(),
            data: Vec::new(),
        }
    }
}
