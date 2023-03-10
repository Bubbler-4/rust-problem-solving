use std::cmp::Ordering::*;
#[allow(clippy::all)]
#[allow(unused_must_use)]
fn solve<R: BufRead, W: Write>(ii: &mut I<R>, oo: &mut W) -> Option<()> {
    let a = ii.get([0usize; 10])?;
    let b = ii.get([0usize; 10])?;
    let a_wins = (0..10).filter(|&i| a[i] > b[i]).count();
    let b_wins = (0..10).filter(|&i| a[i] < b[i]).count();
    let ans = match a_wins.cmp(&b_wins) {
        Less => 'B',
        Equal => 'D',
        Greater => 'A',
    };
    writeln!(oo, "{}", ans);
    None
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
    impl<T: Fill, const N: usize> Fill for [T; N] {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            for ii in self.iter_mut() {
                ii.fill_from_input(i)?;
            }
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
