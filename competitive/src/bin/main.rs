#![allow(unused_imports)]
#![allow(unused_must_use)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

fn solve<R: BufRead, W: Write>(ii: &mut I<R>, oo: &mut W) -> Option<()> {
    None
}

// Nothing to see after this line (I/O template)
#[allow(dead_code)]
mod template {
    pub use std::io::{Write, stdin, stdout, BufWriter, BufRead};
    use std::str::FromStr;
    use std::ops::Deref;

    pub struct I<R: BufRead> {
        r: R,
        line: String,
        rem: &'static str,
    }

    impl<R: BufRead> I<R> {
        pub fn new(r: R) -> Self {
            Self { r, line: String::new(), rem: "" }
        }
        pub fn next_line(&mut self) -> Option<()> {
            self.line.clear();
            let eof = self.r.read_line(&mut self.line).unwrap() == 0;
            if eof { None } else {
                unsafe { self.rem = std::mem::transmute(&self.line[..]); }
                Some(())
            }
        }
        pub fn get<T: Fill>(&mut self, exemplar: T) -> Option<T> {
            let mut exemplar = exemplar;
            exemplar.fill(self)?;
            Some(exemplar)
        }
    }

    pub trait Fill : Sized {
        fn fill<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()>;
    }
    trait Atom : FromStr {}
    impl Atom for u32 {} impl Atom for usize {} impl Atom for u128 {} impl Atom for i64 {} impl Atom for f64 {}
    impl Fill for String {
        fn fill<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            self.clear();
            loop {
                if i.rem.len() == 0 { i.next_line()?; }
                i.rem = i.rem.trim_start_matches([' ', '\n', '\r']);
                if let Some(tok) = i.rem.split_ascii_whitespace().next() {
                    i.rem = &i.rem[tok.len()..];
                    self.push_str(tok);
                    return Some(());
                }
            }
        }
    }
    impl<T> Fill for T where T: Atom {
        fn fill<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            loop {
                if i.rem.len() == 0 { i.next_line()?; }
                i.rem = i.rem.trim_start_matches([' ', '\n', '\r']);
                if let Some(tok) = i.rem.split_ascii_whitespace().next() {
                    i.rem = &i.rem[tok.len()..];
                    *self = tok.parse().ok()?;
                    return Some(());
                }
            }
        }
    }
    impl Fill for Vec<u8> {
        fn fill<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            self.clear();
            loop {
                if i.rem.len() == 0 { i.next_line()?; }
                i.rem = i.rem.trim_start_matches([' ', '\n', '\r']);
                if let Some(tok) = i.rem.split_ascii_whitespace().next() {
                    i.rem = &i.rem[tok.len()..];
                    self.extend_from_slice(tok.as_bytes());
                    return Some(());
                }
            }
        }
    }

    pub struct Line(pub String);
    pub struct NLine(pub String);
    impl Deref for Line {
        type Target = str;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl Deref for NLine {
        type Target = str;
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl Fill for Line {
        fn fill<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            let s = i.rem.strip_suffix('\n').unwrap_or(i.rem);
            i.rem = "";
            self.0.push_str(s);
            Some(())
        }
    }
    impl Fill for NLine {
        fn fill<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            i.next_line()?;
            let s = i.rem.strip_suffix('\n').unwrap_or(i.rem);
            i.rem = "";
            self.0.push_str(s);
            Some(())
        }
    }
    pub const S: String = String::new();
    pub const L: Line = Line(S);
    pub const N: NLine = NLine(S);
    pub const B: Vec<u8> = vec![];
    impl<T: Fill, const N: usize> Fill for [T; N] {
        fn fill<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            for ii in self.iter_mut() {
                ii.fill(i)?;
            }
            Some(())
        }
    }
    impl<T: Fill> Fill for Vec<T> {
        fn fill<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            for ii in self.iter_mut() {
                ii.fill(i)?;
            }
            Some(())
        }
    }
    macro_rules! tupf {
        (($($t:ident),*), ($($v:ident),*)) => {
            impl<$($t: Fill),*> Fill for ($($t),*) {
                fn fill<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
                    let ($($v),*) = self;
                    $($v.fill(i)?;)*
                    Some(())
                }
            }
        }
    }
    tupf!((T, U), (t, u));
    tupf!((T, U, V), (t, u, v));
    tupf!((T, U, V, W), (t, u, v, w));
    tupf!((T, U, V, W, X), (t, u, v, w, x));
    tupf!((T, U, V, W, X, Y), (t, u, v, w, x, y));
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