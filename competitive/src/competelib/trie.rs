#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

struct Trie<T> {
    subtree: BTreeMap<T, Trie<T>>
}

impl<T: std::cmp::Ord> Trie<T> {
    fn new() -> Self {
        Trie { subtree: BTreeMap::new() }
    }
    fn insert(&mut self, values: Vec<T>) {
        let mut cur_tree = &mut self.subtree;
        for v in values {
            let x = cur_tree.entry(v).or_insert(Trie { subtree: BTreeMap::new() });
            cur_tree = &mut x.subtree;
        }
    }
}