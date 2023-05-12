#[allow(clippy::all)]
#[allow(unused_must_use, unused_doc_comments)]
fn solve<R: BufRead, W: Write>(io: &mut IO<R, W>) -> Option<()> {
	let [n, m] = io.get([0usize; 2])?;
	let mut v = io.get(vec![0usize; n])?;
	for _ in 0..m {
		let [cmd, x, y] = io.get([0usize; 3])?;
		match cmd {
			1 => {
				v[x - 1] = y;
			}
			2 => {
				for i in x..=y {
					v[i - 1] = 1 - v[i - 1];
				}
			}
			3 => {
				for i in x..=y {
					v[i - 1] = 0;
				}
			}
			4 => {
				for i in x..=y {
					v[i - 1] = 1;
				}
			}
			_ => {}
		}
	}
	io.put(&*v);
	None
}
/// IO template
mod io {
	pub(crate) use std::io::{Write, stdin, stdout, BufWriter, BufRead};
	pub(crate) struct IO<R: BufRead, W: Write> {
		ii: I<R>,
		oo: BufWriter<W>,
	}
	impl<R: BufRead, W: Write> IO<R, W> {
		pub(crate) fn new(r: R, w: W) -> Self {
			Self {
				ii: I::new(r),
				oo: BufWriter::new(w),
			}
		}
		pub(crate) fn get<T: Fill>(&mut self, exemplar: T) -> Option<T> {
			self.ii.get(exemplar)
		}
		pub(crate) fn put<T: Print>(&mut self, t: T) -> &mut Self {
			t.print(&mut self.oo);
			self
		}
	}
	pub(crate) trait Print {
		fn print<W: Write>(&self, w: &mut W);
	}
	macro_rules! print_disp {
		($($t:ty),+) => {
			$(impl Print for $t { fn print < W : Write > (& self, w : & mut W) {
			write!(w, "{}", self) .unwrap(); } })+
		};
	}
	print_disp!(usize, i64, String, & str);
	impl<T: Print> Print for &[T] {
		fn print<W: Write>(&self, w: &mut W) {
			let mut iter = self.iter();
			if let Some(t) = iter.next() {
				t.print(w);
			}
			for t in iter {
				w.write(b" ").unwrap();
				t.print(w);
			}
		}
	}
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
	macro_rules! fill_num {
		($($t:ty),+) => {
			$(impl Fill for $t { fn fill_from_input < R : BufRead > (& mut self, i : &
			mut I < R >) -> Option < () > { i.rem = i.rem.trim_start_matches(WS); while i
			.rem.is_empty() { i.next_line() ?; i.rem = i.rem.trim_start_matches(WS); }
			let tok = i.rem.split(WS).next().unwrap(); i.rem = & i.rem[tok.len()..]; *
			self = tok.parse().ok() ?; Some(()) } })+
		};
	}
	fill_num!(usize, i64, f64);
	impl<T: Fill> Fill for Vec<T> {
		fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
			for ii in self.iter_mut() {
				ii.fill_from_input(i)?;
			}
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
	let stdin = stdin().lock();
	let stdout = stdout().lock();
	let mut io = IO::new(stdin, stdout);
	solve(&mut io);
}
