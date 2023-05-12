#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::*;
use std::cmp::{Reverse, Ordering, Ordering::*};

#[allow(clippy::all)]
#[allow(unused_must_use, unused_doc_comments)]
fn solve<R: BufRead, W: Write>(io: &mut IO<R, W>) -> Option<()> {
    let [n, m] = io.get([0usize; 2])?;
    let mut v = io.get(vec![0usize; n])?;
    for _ in 0..m {
        let [cmd, x, y] = io.get([0usize; 3])?;
        match cmd {
            1 => { v[x-1] = y; }
            2 => { for i in x..=y { v[i-1] = 1 - v[i-1]; } }
            3 => { for i in x..=y { v[i-1] = 0; } }
            4 => { for i in x..=y { v[i-1] = 1; } }
            _ => {}
        }
    }
    io.put(&*v);
    None
}

mod prime;
mod ntheory;
mod sequence;
mod graph;
mod geometry;
mod string;
mod fft;
/// IO template
mod io;
use io::*;

pub fn main() {
    let stdin = stdin().lock();
    // let mut ii = I::new(stdin.lock());
    let stdout = stdout().lock();
    // let mut oo = BufWriter::new(stdout);
    let mut io = IO::new(stdin, stdout);
    solve(&mut io);
}