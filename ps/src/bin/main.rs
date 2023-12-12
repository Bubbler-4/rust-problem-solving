#[allow(clippy::all)]
#[allow(unused_must_use, unused_doc_comments)]
fn solve<R: BufRead, W: Write, E: Write>(io: &mut IO<R, W, E>) -> Option<()> {
	let n = io.get(0usize)?;
	let v = io.get(vec![0usize; n])?;
	let mut ans = v.iter().sum::<usize>() * 2 + n * 2 + v[0] + v[n - 1];
	for w in v.windows(2) {
		ans += w[0].abs_diff(w[1]);
	}
	io.put(ans);
	io.eput("");
	None
}
/// IO template
mod io {
	pub(crate) use std::io::{Write, stdin, stdout, stderr, BufWriter, BufRead};
	pub(crate) struct IO<R: BufRead, W: Write, E: Write> {
		ii: I<R>,
		oo: BufWriter<W>,
		ee: BufWriter<E>,
	}
	impl<R: BufRead, W: Write, E: Write> IO<R, W, E> {
		pub(crate) fn new(r: R, w: W, e: E) -> Self {
			Self {
				ii: I::new(r),
				oo: BufWriter::new(w),
				ee: BufWriter::new(e),
			}
		}
		pub(crate) fn get<T: Fill>(&mut self, exemplar: T) -> Option<T> {
			self.ii.get(exemplar)
		}
		pub(crate) fn put<T: Print>(&mut self, t: T) -> &mut Self {
			t.print(&mut self.oo);
			self
		}
		pub(crate) fn eput<T: Print>(&mut self, t: T) -> &mut Self {
			t.print(&mut self.ee);
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
	print_disp!(usize, i64, String, & str, char);
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
	fn ws(c: char) -> bool {
		c <= ' '
	}
	macro_rules! fill_num {
		($($t:ty),+) => {
			$(impl Fill for $t { fn fill_from_input < R : BufRead > (& mut self, i : &
			mut I < R >) -> Option < () > { i.rem = i.rem.trim_start_matches(ws); while i
			.rem.is_empty() { i.next_line() ?; i.rem = i.rem.trim_start_matches(ws); }
			let tok = i.rem.split(ws).next().unwrap(); i.rem = & i.rem[tok.len()..]; *
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
}
use io::*;
pub fn main() {
	let stdin = stdin().lock();
	let stdout = stdout().lock();
	let stderr = stderr().lock();
	let mut io = IO::new(stdin, stdout, stderr);
	solve(&mut io);
}
