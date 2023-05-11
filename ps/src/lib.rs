#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::*;
use std::cmp::{Reverse, Ordering, Ordering::*};

#[allow(clippy::all)]
#[allow(unused_must_use, unused_doc_comments)]
fn solve<R: BufRead, W: Write>(ii: &mut I<R>, oo: &mut W) -> Option<()> {
    let t = ii.get(0usize)?;
    for _ in 0..t {
        let line = ii.get(N)?.0.replace(' ', "");
        let mut cnts = vec![0usize; 128];
        for b in line.bytes() {
            cnts[b as usize] += 1;
        }
        let max = *cnts.iter().max().unwrap();
        let max_c = cnts.iter().enumerate().filter(|&(_, &x)| x == max).map(|(i, _)| i as u8 as char).collect::<String>();
        if max_c.len() != 1 { writeln!(oo, "?"); }
        else { writeln!(oo, "{}", max_c); }
    }
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
    let stdin = stdin();
    let mut ii = I::new(stdin.lock());
    let stdout = stdout();
    let stdout = stdout.lock();
    let mut oo = BufWriter::new(stdout);
    solve(&mut ii, &mut oo);
}