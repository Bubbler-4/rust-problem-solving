#![allow(unused_imports)]
#![allow(unused_must_use)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

fn solve<R: BufRead, W: Write>(ii: &mut I<R>, oo: &mut W) -> Option<()> {
    let [n, m] = ii.get::<[usize; 2]>()?;
    for _ in 0..m {
        let v = ii.getn::<i64>(n)?;
        let mut s = v.iter().sum::<i64>();
        let mut idx = 0usize;
        while s != 0 {
            s -= v[idx] * 2;
            idx += 1;
        }
        for _ in 0..idx { write!(oo, "-1 "); }
        for _ in idx..n { write!(oo, "1 "); }
        writeln!(oo);
    }
    None
}


// Nothing to see after this line (I/O template)
#[allow(dead_code)]
mod template {
    pub use std::io::{Write, stdin, stdout, BufWriter, BufRead, Cursor};
    use std::str::FromStr;
    use std::ops::{Index, IndexMut, Range, Deref};
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Dim<T: Sized, const N: usize> {
        dims: [usize; N],
        slice: Vec<T>
    }
    impl<T, const N: usize> Dim<T, N> {
        pub fn new(dims: [usize; N], slice: Vec<T>) -> Self { Self { dims, slice } }
        pub fn into_flat(self) -> Vec<T> { self.slice }
    }
    impl<T, const N: usize> Index<[usize; N]> for Dim<T, N> {
        type Output = T;
        fn index(&self, index: [usize; N]) -> &Self::Output {
            let mut idx = index[0];
            for (dim, ind) in self.dims.iter().zip(&index).skip(1) {
                idx = idx * dim + ind;
            }
            &self.slice[idx]
        }
    }
    impl<T, const N: usize> IndexMut<[usize; N]> for Dim<T, N> {
        fn index_mut(&mut self, index: [usize; N]) -> &mut Self::Output {
            let mut idx = index[0];
            for (dim, ind) in self.dims.iter().zip(&index).skip(1) {
                idx = idx * dim + ind;
            }
            &mut self.slice[idx]
        }
    }
    pub struct Nd<const N: usize>(pub Range<[usize; N]>);
    impl<const N: usize> Iterator for Nd<N> {
        type Item = [usize; N];
        fn next(&mut self) -> Option<Self::Item> {
            let ret = self.0.start;
            if self.0.start[0] >= self.0.end[0] { return None; }
            for i in (0..N).rev() {
                self.0.start[i] += 1;
                if self.0.start[i] < self.0.end[i] { break; }
                else if i > 0 { self.0.start[i] = 0; }
            }
            Some(ret)
        }
    }

    pub struct I<R: BufRead> {
        r: R,
        line: String,
        rem: &'static str,
    }

    impl<R: BufRead> I<R> {
        pub fn new(r: R) -> Self {
            Self { r, line: String::new(), rem: "" }
        }
        pub fn get<T: Get2>(&mut self) -> Option<T> {
            T::get(self)
        }
        fn next_line(&mut self) -> Option<()> {
            self.line.clear();
            let eof = self.r.read_line(&mut self.line).unwrap() == 0;
            if eof { None } else {
                unsafe { self.rem = std::mem::transmute(&self.line[..]); }
                Some(())
            }
        }
        pub fn getn<T: Get2>(&mut self, n: usize) -> Option<Vec<T>> {
            let mut res = Vec::with_capacity(n);
            for _ in 0..n { res.push(self.get()?); }
            Some(res)
        }
        pub fn getd<T: Get2, const N: usize>(&mut self, dims: [usize; N]) -> Option<Dim<T, N>> {
            let size = dims.iter().product::<usize>();
            let slice = self.getn(size)?;
            Some(Dim::new(dims, slice))
        }
    }

    pub trait Get2 : Sized {
        fn get<R: BufRead>(i: &mut I<R>) -> Option<Self>;
    }
    trait Atom : FromStr {}
    impl Atom for u32 {} impl Atom for usize {} impl Atom for u128 {} impl Atom for i64 {} impl Atom for f64 {} impl Atom for String {}
    impl<T> Get2 for T where T: Atom {
        fn get<R: BufRead>(i: &mut I<R>) -> Option<Self> {
            loop {
                if i.rem.is_empty() { i.next_line()?; }
                i.rem = i.rem.trim_start_matches([' ', '\n', '\r']);
                if let Some(tok) = i.rem.split_ascii_whitespace().next() {
                    i.rem = &i.rem[tok.len()..];
                    return tok.parse().ok();
                }
            }
        }
    }
    impl Get2 for Vec<u8> {
        fn get<R: BufRead>(i: &mut I<R>) -> Option<Self> {
            <String as Get2>::get(i).map(|s| s.into_bytes())
        }
    }

    pub struct Line(pub String);
    impl Deref for Line {
        type Target = str;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl Get2 for Line {
        fn get<R: BufRead>(i: &mut I<R>) -> Option<Self> {
            let s = i.rem.strip_suffix('\n').unwrap_or(i.rem).to_owned();
            i.rem = "";
            Some(Line(s))
        }
    }
    impl<T: Get2 + Default, const N: usize> Get2 for [T; N] {
        fn get<R: BufRead>(i: &mut I<R>) -> Option<Self> {
            let mut ret = [(); N].map(|_| T::default());
            for cell in &mut ret {
                *cell = <T as Get2>::get(i)?;
            }
            Some(ret)
        }
    }
    macro_rules! tup { ($($t:ident),*) => { impl<$($t: Get2),*> Get2 for ($($t),*) { fn get<R: BufRead>(i: &mut I<R>) -> Option<Self> { Some(($(<$t as Get2>::get(i)?),*)) }}}; }
    tup!(T, U); tup!(T, U, V); tup!(T, U, V, W); tup!(T, U, V, W, X); tup!(T, U, V, W, X, Y);
}
use template::*;

fn main() {
    let stdin = stdin();
    let mut ii = I::new(stdin.lock());
    let stdout = stdout();
    let stdout = stdout.lock();
    let mut oo = BufWriter::new(stdout);
    solve(&mut ii, &mut oo);
}