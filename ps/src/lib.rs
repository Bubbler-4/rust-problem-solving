#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

#[allow(clippy::all)]
#[allow(unused_must_use)]
fn solve<R: BufRead, W: Write>(ii: &mut I<R>, oo: &mut W) -> Option<()> {
    let n = ii.get(0i64)?;
    if n == 2 { writeln!(oo, "E"); None? }
    let mut ans = 0.0f64;
    for i in 0..=n {
        if 4*n - 6*i - 3 < 0 { break; }
        let p = (6*i+1).pow(2);
        ans += (ntheory::modpow_u128(2, (4*n - 6*i - 3) as u128, p as u128 * 16) as f64 * 144.0 / p as f64).rem_euclid(16.0);
        let p = (6*i+2).pow(2);
        ans -= (ntheory::modpow_u128(2, (4*n - 6*i - 3) as u128, p as u128 * 16) as f64 * 216.0 / p as f64).rem_euclid(16.0);
        let p = (6*i+3).pow(2);
        ans -= (ntheory::modpow_u128(2, (4*n - 6*i - 3) as u128, p as u128 * 16) as f64 * 72.0 / p as f64).rem_euclid(16.0);
        let p = (6*i+4).pow(2);
        ans -= (ntheory::modpow_u128(2, (4*n - 6*i - 3) as u128, p as u128 * 16) as f64 * 54.0 / p as f64).rem_euclid(16.0);
        let p = (6*i+5).pow(2);
        ans += (ntheory::modpow_u128(2, (4*n - 6*i - 3) as u128, p as u128 * 16) as f64 * 9.0 / p as f64).rem_euclid(16.0);
    }
    let ans = ans.rem_euclid(16.0).floor() as usize;
    writeln!(oo, "{}", &"0123456789ABCDEF"[ans..ans+1]);
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