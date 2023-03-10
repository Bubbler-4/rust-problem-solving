#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

#[allow(clippy::all)]
#[allow(unused_must_use)]
fn solve<R: BufRead, W: Write>(ii: &mut I<R>, oo: &mut W) -> Option<()> {
    let a = ii.get([0usize; 10])?;
    let b = ii.get([0usize; 10])?;
    let a_wins = (0..10).filter(|&i| a[i] > b[i]).count();
    let b_wins = (0..10).filter(|&i| a[i] < b[i]).count();
    let ans = match a_wins.cmp(&b_wins) {
        Less => 'B',
        Equal => 'D',
        Greater => 'A',
    };
    writeln!(oo, "{}", ans);
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