#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::*;
use std::cmp::{Reverse, Ordering, Ordering::*};

#[allow(clippy::all)]
#[allow(unused_must_use)]
fn solve<R: BufRead, W: Write>(ii: &mut I<R>, oo: &mut W) -> Option<()> {
    loop {
        let n = ii.get(0usize)?;
        if n == 0 { break; }
        let v = ii.get(vec![(0i64, 0i64); n])?;
        let maxy = v.iter().map(|xy| xy.1).max().unwrap();
        let max_xpy = v.iter().map(|xy| xy.0 + xy.1).max().unwrap();
        let maxx = v.iter().map(|xy| xy.0).max().unwrap();
        let max_xmy = v.iter().map(|xy| xy.0 - xy.1).max().unwrap();
        let miny = v.iter().map(|xy| xy.1).min().unwrap();
        let min_xpy = v.iter().map(|xy| xy.0 + xy.1).min().unwrap();
        let minx = v.iter().map(|xy| xy.0).min().unwrap();
        let min_xmy = v.iter().map(|xy| xy.0 - xy.1).min().unwrap();
        let b = (maxx + maxy - max_xpy) + (maxx - miny - max_xmy) + (min_xpy - minx - miny) + (min_xmy - minx + maxy);
        let a = (maxx - minx + maxy - miny) * 2 - b * 2;
        writeln!(oo, "{} {}", a, b);
    }
    None
}

mod prime;
mod ntheory;
mod graph;
mod geometry;
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