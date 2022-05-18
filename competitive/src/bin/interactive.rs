#![allow(unused_macros)]
#![allow(unused_imports)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};
use std::io::{prelude::*, stdin, stdout, BufWriter};
use std::convert::TryInto;
use std::error::Error;

#[allow(dead_code)]
fn fix<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into().unwrap_or_else(|_| panic!())
}

// [t;n] -> (0..n).map(|_| r!(it, t)).collect::<Vec<_>>()
// (t1, t2, ..., ti) -> (r!(it, t1), r!(it, t2), ...)
// &str -> it.next().unwrap()
// t -> it.next().unwrap().parse::<t>().unwrap()
macro_rules! i {
    () => {};
    ($i:ident, &str) => { $i.next().unwrap() };
    ($i:ident, &[u8]) => { $i.next().unwrap().as_bytes() };
    (@v $i:ident, ($($b:tt)*) ; $($l:tt)*) => { (0..$($l)*).map(|_| i!($i, $($b)*)).collect::<Vec<_>>() };
    (@v $i:ident, ($($b:tt)*) $t:tt $($l:tt)*) => { i!(@v $i, ($($b)* $t) $($l)* ) };
    ($i:ident, [$t:tt $($l:tt)*]) => { i!(@v $i, ($t) $($l)* ) };
    (@t $i:ident, ($($b:tt)*) ($($a:tt)*) ()) => { ($($b)*, i!($i, $($a)*)) };
    (@t $i:ident, () ($($a:tt)*) (, $t:tt $($p:tt)*)) => { i!(@t $i, (i!($i, $($a)*)) ($t) ($($p)*) ) };
    (@t $i:ident, ($($b:tt)*) ($($a:tt)*) (, $t:tt $($p:tt)*)) => { i!(@t $i, ($($b)*, i!($i, $($a)*)) ($t) ($($p)*) ) };
    (@t $i:ident, ($($b:tt)*) ($($a:tt)*) ($t:tt $($p:tt)*)) => { i!(@t $i, ($($b)*) ($($a)* $t) ($($p)*) ) };
    ($i:ident, ($t:tt $($p:tt)*)) => { i!(@t $i, () ($t) ($($p)*) ) };
    ($i:ident, $t:tt) => { $i.next().unwrap().parse::<$t>().unwrap() };
}
macro_rules! brk { ($i:ident) => { if $i.peek().is_none() { break; } }; }

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();
    let mut buf = String::with_capacity(1<<20);
    let mut ww;
    let mut stdout = stdout();

    //////////////////////////////
    stdin.read_line(&mut buf)?;
    ww = buf.split_ascii_whitespace();
    let [_n, k, m] = fix(i!(ww, [usize; 3]));
    for _ in 0..m {
        for i in 1..=k { print!("{} ", i); }
        println!();
        stdout.flush()?;
        buf.clear();
        stdin.read_line(&mut buf)?;
        //ww = buf.split_ascii_whitespace();
        //let _ = i!(ww, [&str; n-k]);
    }
    Ok(())
}
