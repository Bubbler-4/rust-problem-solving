#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::*;
use std::cmp::{Reverse, Ordering, Ordering::*};

#[allow(clippy::all)]
#[allow(unused_must_use, unused_doc_comments)]
fn solve<R: BufRead, W: Write>(io: &mut IO<R, W>) -> Option<()> {
    let k = io.get(0usize)?;
    let b = io.get(B)?;
    let ans = b.into_iter().step_by(k).collect::<Vec<_>>();
    io.put(ans);
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