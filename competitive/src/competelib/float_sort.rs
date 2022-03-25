#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

#[derive(PartialEq, PartialOrd)]
struct TotalF64(f64);
impl Eq for TotalF64 {}
impl Ord for TotalF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.partial_cmp(other).unwrap() }
}