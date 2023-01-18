#![allow(unused_imports)]
#![allow(unused_must_use)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

#[allow(clippy::all)]
fn solve<R: BufRead, W: Write>(ii: &mut I<R>, oo: &mut W) -> Option<()> {
    writeln!(oo, "{}", ii.get(0usize)?);
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
                self.rem = unsafe { (&self.line[..] as *const str).as_ref()? };
                Some(())
            }
        }
        pub fn get<T: Fill>(&mut self, exemplar: T) -> Option<T> {
            let mut exemplar = exemplar;
            exemplar.fill_from_input(self)?;
            Some(exemplar)
        }
    }

    pub trait Fill : Sized {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()>;
    }
    trait Atom : FromStr {}
    macro_rules! atom { ($($x: ident)*) => { $(impl Atom for $x {})* } }
    atom!(u16 u32 usize u128 i16 i32 i64 i128 f64);
    impl Fill for String {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            self.clear();
            loop {
                if i.rem.is_empty() { i.next_line()?; }
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
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            loop {
                if i.rem.is_empty() { i.next_line()?; }
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
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            self.clear();
            loop {
                if i.rem.is_empty() { i.next_line()?; }
                i.rem = i.rem.trim_start_matches([' ', '\n', '\r']);
                if let Some(tok) = i.rem.split_ascii_whitespace().next() {
                    i.rem = &i.rem[tok.len()..];
                    self.extend_from_slice(tok.as_bytes());
                    return Some(());
                }
            }
        }
    }

    #[derive(Clone)]
    pub struct Line(pub String);
    #[derive(Clone)]
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
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            let s = i.rem.strip_suffix('\n').unwrap_or(i.rem);
            i.rem = "";
            self.0.push_str(s);
            Some(())
        }
    }
    impl Fill for NLine {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
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
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            for ii in self.iter_mut() {
                ii.fill_from_input(i)?;
            }
            Some(())
        }
    }
    impl<T: Fill> Fill for Vec<T> {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            for ii in self.iter_mut() {
                ii.fill_from_input(i)?;
            }
            Some(())
        }
    }
    macro_rules! tup {
        (($($t:ident),*), ($($v:ident),*)) => {
            impl<$($t: Fill),*> Fill for ($($t),*) {
                fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
                    let ($($v),*) = self;
                    $($v.fill_from_input(i)?;)*
                    Some(())
                }
            }
        }
    }
    tup!((T, U), (t, u));
    tup!((T, U, V), (t, u, v));
    tup!((T, U, V, W), (t, u, v, w));
    tup!((T, U, V, W, X), (t, u, v, w, x));
    tup!((T, U, V, W, X, Y), (t, u, v, w, x, y));
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