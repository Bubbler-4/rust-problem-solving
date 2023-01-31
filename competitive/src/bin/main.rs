#![allow(unused_imports)]
#![allow(unused_must_use)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

#[allow(clippy::all)]
fn solve<R: BufRead, W: Write>(ii: &mut I<R>, oo: &mut W) -> Option<()> {
    let [a, b] = ii.get([0usize; 2])?;
    None
}

// Nothing to see after this line (I/O template)
#[allow(dead_code)]
mod template {
    pub use std::io::{Write, stdin, stdout, BufWriter, BufRead};
    use std::str::FromStr;
    //use std::ops::Deref;

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
        pub fn get<U, T: Fill<U>>(&mut self, exemplar: T) -> Option<T> {
            let mut exemplar = exemplar;
            exemplar.fill_from_input(self)?;
            Some(exemplar)
        }
    }

    pub trait Fill<T> : Sized {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()>;
    }
    trait Atom : FromStr {}
    macro_rules! atom { ($($x: ident)*) => { $(impl Atom for $x {})* } }
    atom!(u16 u32 usize u128 i16 i32 i64 i128 f64);

    trait Set { fn set(&mut self, s: &str) -> Option<()>; }
    impl<T: Atom> Set for T { fn set(&mut self, s: &str) -> Option<()> { *self = s.parse().ok()?; Some(()) }}
    impl Set for String { fn set(&mut self, s: &str) -> Option<()> { self.push_str(s); Some(()) }}
    impl Set for Vec<u8> { fn set(&mut self, s: &str) -> Option<()> { self.extend_from_slice(s.as_bytes()); Some(()) }}
    impl<T: Set> Fill<Self> for T {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            loop {
                if i.rem.is_empty() { i.next_line()?; }
                i.rem = i.rem.trim_start_matches([' ', '\n', '\r']);
                if let Some(tok) = i.rem.split_ascii_whitespace().next() {
                    i.rem = &i.rem[tok.len()..];
                    self.set(tok);
                    return Some(());
                }
            }
        }
    }

    #[derive(Clone)]
    pub struct Line<T, const N: usize>(pub T);
    impl<T: Set, const N: usize> Fill<Self> for Line<T, N> {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            if N > 0 { i.next_line()?; }
            let s = i.rem.strip_suffix('\n').unwrap_or(i.rem);
            i.rem = "";
            self.0.set(s);
            Some(())
        }
    }
    pub const S: String = String::new();
    pub const L: Line<String, 0> = Line(S);
    pub const N: Line<String, 1> = Line(S);
    pub const B: Vec<u8> = Vec::new();
    pub const LB: Line<Vec<u8>, 0> = Line(B);
    pub const NB: Line<Vec<u8>, 1> = Line(B);
    
    impl<T: Fill<T>, U: std::borrow::BorrowMut<[T]>> Fill<&mut [T]> for U {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            for ii in self.borrow_mut().iter_mut() {
                ii.fill_from_input(i)?;
            }
            Some(())
        }
    }

    macro_rules! tup {
        ($(($($t:ident),*)),*) => {
            $(impl<$($t: Fill<$t>),*> Fill<Self> for ($($t),*) {
                fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
                    #[allow(non_snake_case)] let ($($t),*) = self;
                    $($t.fill_from_input(i)?;)*
                    Some(())
                }
            })*
        }
    }
    tup!((T, U), (T, U, V), (T, U, V, W), (T, U, V, W, X), (T, U, V, W, X, Y));
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