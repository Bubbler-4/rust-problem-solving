#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

#[allow(clippy::all)]
#[allow(unused_must_use)]
fn solve<R: BufRead, W: Write>(ii: &mut I<R>, oo: &mut W) -> Option<()> {
    let _dj = graph::DisjointSet::new(10);
    let lines = (0..5).map(|_| ii.get(NB).unwrap().0).collect::<Vec<_>>();
    let buf = (0..15).flat_map(|i| (0..5).map(move |j| (i, j))).flat_map(|(i,j)| lines[j].get(i)).copied().collect::<Vec<_>>();
    oo.write(&buf);
    None
}

mod prime;
mod graph;
/// IO template
mod io;
use io::*;

pub fn main() {
    let stdin = stdin();
    let mut ii = I::new(stdin.lock());
    let stdout = stdout();
    let stdout = stdout.lock();
    let mut oo = BufWriter::new(stdout);
    solve(&mut ii, &mut oo);
}