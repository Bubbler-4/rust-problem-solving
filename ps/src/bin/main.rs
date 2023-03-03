#[allow(clippy::all)]
#[allow(unused_must_use)]
fn solve<R: BufRead, W: Write>(ii: &mut I<R>, oo: &mut W) -> Option<()> {
    let _dj = graph::DisjointSet::new(10);
    let lines = (0..5).map(|_| ii.get(NB).unwrap().0).collect::<Vec<_>>();
    let buf = (0..15)
        .flat_map(|i| (0..5).map(move |j| (i, j)))
        .flat_map(|(i, j)| lines[j].get(i))
        .copied()
        .collect::<Vec<_>>();
    oo.write(&buf);
    None
}
mod graph {
    /// Disjoint set
    pub(crate) struct DisjointSet {
        parent: Vec<usize>,
        rank: Vec<usize>,
    }
    impl DisjointSet {
        pub(crate) fn new(n: usize) -> Self {
            let parent = (0..n).collect();
            let rank = vec![0; n];
            Self { parent, rank }
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
    pub(crate) struct Line<T, const B: bool>(pub T);
    impl<const B: bool> Fill for Line<Vec<u8>, B> {
        fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
            if B {
                i.next_line()?;
            }
            let s = i.rem.strip_suffix('\n').unwrap_or(i.rem);
            i.rem = "";
            self.0.extend_from_slice(s.as_bytes());
            Some(())
        }
    }
    pub(crate) const B: Vec<u8> = Vec::new();
    pub(crate) const NB: Line<Vec<u8>, true> = Line(B);
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
