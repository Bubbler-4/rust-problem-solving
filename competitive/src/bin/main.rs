#![allow(unused_imports)]
#![allow(unused_must_use)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

#[allow(clippy::all)]
fn solve<R: BufRead, W: Write>(_ii: &mut I<R>, _oo: &mut W) -> Option<()> {
    None
}

// Nothing to see after this line (I/O template)
#[allow(dead_code)]
mod template {
    pub use std::io::{Write, stdin, stdout, BufWriter, BufRead};
    use std::str::FromStr;

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

    pub trait Fill { fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()>; }

    trait Set { fn set(&mut self, s: &str) -> Option<()>; }
    macro_rules! set {
        ($t: ty, $self: ident, $s: ident, $e: stmt) => {
            impl Set for $t { fn set(&mut $self, $s: &str) -> Option<()> { $e Some(()) }}
        }
    }
    macro_rules! atom { ($($x: ident)*) => { $(set!($x, self, s, *self = s.parse().ok()?); )* }}
    atom!(u16 u32 usize u128 i16 i32 i64 i128 f64);
    set!(String, self, s, self.push_str(s));
    set!(Vec<u8>, self, s, self.extend_from_slice(s.as_bytes()));
    const WS: [char; 3] = [' ', '\n', '\r'];
    macro_rules! tok { ($($x: ty)*) => { $(
        impl Fill for $x {
            fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
                i.rem = i.rem.trim_start_matches(WS);
                while i.rem.is_empty() { i.next_line()?; i.rem = i.rem.trim_start_matches(WS); }
                let tok = i.rem.split(WS).next().unwrap();
                i.rem = &i.rem[tok.len()..];
                return self.set(tok);
            }
        }
    )*}}
    tok!(u16 u32 usize u128 i16 i32 i64 i128 f64 String Vec<u8>);

    #[derive(Clone)]
    pub struct Line<T, const B: bool>(pub T);
    impl<T: Set, const B: bool> Fill for Line<T, B> {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            if B { i.next_line()?; }
            let s = i.rem.strip_suffix('\n').unwrap_or(i.rem);
            i.rem = "";
            self.0.set(s);
            Some(())
        }
    }

    pub const S: String = String::new();
    pub const L: Line<String, false> = Line(S);
    pub const N: Line<String, true> = Line(S);
    pub const B: Vec<u8> = Vec::new();
    pub const LB: Line<Vec<u8>, false> = Line(B);
    pub const NB: Line<Vec<u8>, true> = Line(B);

    macro_rules! iter { () => {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            for ii in self.iter_mut() { ii.fill_from_input(i)?; } Some(())
        }
    }}
    impl<T: Fill> Fill for Vec<T> { iter!(); }
    impl<T: Fill, const N: usize> Fill for [T; N] { iter!(); }

    macro_rules! tup { ($(($($t:ident),*)),*) => { $(
        impl<$($t: Fill),*> Fill for ($($t),*) {
            fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
                #[allow(non_snake_case)] let ($($t),*) = self;
                $($t.fill_from_input(i)?;)* Some(())
            }
        }
    )*}}
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