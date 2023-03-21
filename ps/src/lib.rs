#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

#[allow(clippy::all)]
#[allow(unused_must_use)]
fn solve<R: BufRead, W: Write>(ii: &mut I<R>, oo: &mut W) -> Option<()> {
    let [t, s] = ii.get([0usize; 2])?;
    match (t, s) {
        (12..=16, 0) => writeln!(oo, "320"),
        _ => writeln!(oo, "280"),
    };
    None
}

mod prime;
mod ntheory;
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