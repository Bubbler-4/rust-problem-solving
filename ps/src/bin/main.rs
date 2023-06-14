#[allow(clippy::all)]
#[allow(unused_must_use)]
fn solve<R: BufRead, W: Write>(ii: &mut I<R>, oo: &mut W) -> Option<()> {
    loop {
        let n = ii.get(0usize)?;
        if n == 0 {
            break;
        }
        let v = ii.get(vec![(0i64, 0i64); n])?;
        let maxy = v.iter().map(|xy| xy.1).max().unwrap();
        let max_xpy = v.iter().map(|xy| xy.0 + xy.1).max().unwrap();
        let maxx = v.iter().map(|xy| xy.0).max().unwrap();
        let max_xmy = v.iter().map(|xy| xy.0 - xy.1).max().unwrap();
        let miny = v.iter().map(|xy| xy.1).min().unwrap();
        let min_xpy = v.iter().map(|xy| xy.0 + xy.1).min().unwrap();
        let minx = v.iter().map(|xy| xy.0).min().unwrap();
        let min_xmy = v.iter().map(|xy| xy.0 - xy.1).min().unwrap();
        let b = (maxx + maxy - max_xpy) + (maxx - miny - max_xmy)
            + (min_xpy - minx - miny) + (min_xmy - minx + maxy);
        let a = (maxx - minx + maxy - miny) * 2 - b * 2;
        writeln!(oo, "{} {}", a, b);
    }
    None
}
mod geometry {
    use std::cmp::Ordering;
    #[derive(Clone, Copy)]
    pub(crate) struct Frac(pub(crate) i64, pub(crate) i64);
    impl PartialEq for Frac {
        fn eq(&self, other: &Self) -> bool {
            self.0 * other.1 == self.1 * other.0
        }
    }
    impl Eq for Frac {}
    impl PartialOrd for Frac {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            (self.0 * other.1).partial_cmp(&(self.1 * other.0))
        }
    }
}
/// IO template
mod io {
    pub(crate) use std::io::{Write, stdin, stdout, BufWriter, BufRead};
    pub(crate) struct I<R: BufRead> {
        r: R,
        line: String,
        rem: &'static str,
    }
    impl<R: BufRead> I<R> {
        pub(crate) fn new(r: R) -> Self {
            Self {
                r,
                line: String::new(),
                rem: "",
            }
        }
        pub(crate) fn next_line(&mut self) -> Option<()> {
            self.line.clear();
            (self.r.read_line(&mut self.line).unwrap() > 0)
                .then(|| {
                    self
                        .rem = unsafe {
                        (&self.line[..] as *const str).as_ref().unwrap()
                    };
                })
        }
        pub(crate) fn get<T: Fill>(&mut self, exemplar: T) -> Option<T> {
            let mut exemplar = exemplar;
            exemplar.fill_from_input(self)?;
            Some(exemplar)
        }
    }
    pub(crate) trait Fill {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()>;
    }
    const WS: [char; 3] = [' ', '\n', '\r'];
    impl Fill for usize {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            i.rem = i.rem.trim_start_matches(WS);
            while i.rem.is_empty() {
                i.next_line()?;
                i.rem = i.rem.trim_start_matches(WS);
            }
            let tok = i.rem.split(WS).next().unwrap();
            i.rem = &i.rem[tok.len()..];
            *self = tok.parse().ok()?;
            Some(())
        }
    }
    impl Fill for i64 {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            i.rem = i.rem.trim_start_matches(WS);
            while i.rem.is_empty() {
                i.next_line()?;
                i.rem = i.rem.trim_start_matches(WS);
            }
            let tok = i.rem.split(WS).next().unwrap();
            i.rem = &i.rem[tok.len()..];
            *self = tok.parse().ok()?;
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
    impl<T: Fill, U: Fill> Fill for (T, U) {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            #[allow(non_snake_case)]
            let (T, U) = self;
            T.fill_from_input(i)?;
            U.fill_from_input(i)?;
            Some(())
        }
    }
}
use io::*;
pub fn main() {
    let stdin = stdin();
    let mut ii = I::new(stdin.lock());
    let stdout = stdout();
    let stdout = stdout.lock();
    let mut oo = BufWriter::new(stdout);
    solve(&mut ii, &mut oo);
}
