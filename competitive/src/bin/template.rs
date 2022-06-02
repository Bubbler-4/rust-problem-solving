#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};
use std::io::{Read, Write, stdin, BufWriter};
use std::convert::TryInto;
use std::os::unix::prelude::FromRawFd;

type Is = dyn Iterator<Item=&'static str>;
struct I { i: Box<Is> }
impl I {
    fn new(s: &'static str) -> Self {
        let it = s.split_ascii_whitespace(); // change as necessary
        I { i: Box::new(it) }
    }
    fn g<T: Get>(&mut self) -> Option<T> { T::get(&mut self.i) }
}
macro_rules! g {
    ($i: ident, $t: ty) => { $i.g::<$t>() };
    ($i: ident, $t: ty, $e:expr $(, $ee:expr)*) => {
        {let n = $e;
        (|| {
            let mut vv = Vec::with_capacity(n);
            for _ in 0..n { vv.push(g!($i, $t $(, $ee)*)?); }
            Some(vv)
        })()}
    };
}
trait Get: Sized { fn get(it: &mut Is) -> Option<Self>; }
macro_rules! get {
    (p, $it:ident) => { $it.next()?.parse().ok() };
    (s, $it:ident) => { $it.next() };
    (b, $it:ident) => { Some($it.next()?.as_bytes()) };
    ($ty:ty, $tt:tt) => { impl Get for $ty { fn get(it: &mut Is) -> Option<Self> { get!($tt, it) }}};
}
get!(usize, p); get!(u128, p); get!(i64, p); get!(i128, p); get!(f64, p); get!(&str, s); get!(&[u8], b);
macro_rules! tup { ($($t:ident),*) => { impl<$($t: Get),*> Get for ($($t),*) { fn get(it: &mut Is) -> Option<Self> { Some(($($t::get(it)?),*)) }}}; }
tup!(T, U); tup!(T, U, V); tup!(T, U, V, W); tup!(T, U, V, W, X); tup!(T, U, V, W, X, Y);
impl<T: Get, const N: usize> Get for [T;N] { fn get(it: &mut Is) -> Option<Self> {
    let mut vv = Vec::with_capacity(N);
    for _ in 0..N { vv.push(T::get(it)?); }
    vv.try_into().ok()
}}

fn main() {
    let stdin = stdin();
    let stdin = &mut stdin.lock();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();
    let buf = Box::leak(buf.into_boxed_str());
    let ii = I::new(buf);
    let stdout = unsafe { std::fs::File::from_raw_fd(1) };
    let mut oo = BufWriter::new(stdout);
    solve(ii, &mut oo);
}

#[cfg(test)]
mod test {
    #[test]
    fn is_solved() {
        let url = format!("https://www.acmicpc.net/problem/{}", PROBLEM);
        let res = reqwest::blocking::get(url).unwrap().text().unwrap();
        let html = scraper::Html::parse_document(&res);
        let spj_selector = scraper::Selector::parse("span.problem-label-spj").unwrap();
        let mut it = html.select(&spj_selector);
        let spj = it.next().is_some();
        let selector = scraper::Selector::parse("pre.sampledata").unwrap();
        let mut it = html.select(&selector);
        while let Some(inel) = it.next() {
            let output = it.next().unwrap().inner_html();
            let input = Box::leak(inel.inner_html().into_boxed_str());
            let ii = crate::I::new(input);
            let mut oo = std::io::Cursor::new(Vec::<u8>::new());
            crate::solve(ii, &mut oo);
            let result = unsafe { String::from_utf8_unchecked(oo.into_inner()) };
            for (l1, l2) in result.lines().zip(output.lines()) {
                if !spj { assert_eq!(l1.trim_end(), l2.trim_end()); }
                else { println!("{} {}", l1.trim_end(), l2.trim_end()); }
            }
        }
    }
    const PROBLEM: usize = 1117;
}

fn solve<W: Write>(mut ii: I, oo: &mut W) -> Option<()> {
    None
}