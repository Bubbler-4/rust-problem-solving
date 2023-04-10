#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::*;
use std::cmp::{Reverse, Ordering, Ordering::*};

#[allow(clippy::all)]
#[allow(unused_must_use)]
fn solve<R: BufRead, W: Write>(ii: &mut I<R>, oo: &mut W) -> Option<()> {
    let t = ii.get(0usize)?;
    for _ in 0..t {
        let n = ii.get(0usize)?;
        let mut v = vec![];
        for i in 0..n {
            v.push((ii.get(0i64)?, ii.get(0i64)?, i));
        }
        let xsum = v.iter().map(|x| x.0).sum::<i64>();
        let ysum = v.iter().map(|y| y.1).sum::<i64>();
        for i in 0..n {
            v[i].0 = v[i].0 * n as i64 - xsum;
            v[i].1 = v[i].1 * n as i64 - ysum;
        }
        use geometry::Frac;
        v.sort_unstable_by_key(|pt| {
            let &(x, y, _) = pt;
            if x == 0 && y == 0 { (0, Frac(0, 1), 0) }
            else if x > 0 && y >= 0 { (1, Frac(y, x), x+y) }
            else if x <= 0 && y > 0 { (2, Frac(-x, y), y-x) }
            else if x < 0 && y <= 0 { (3, Frac(-y, -x), -x-y) }
            else { (4, Frac(x, -y), x-y) }
        });
        for &(_, _, i) in &v {
            write!(oo, "{} ", i);
        }
        writeln!(oo);
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