use std::collections::*;
#[allow(clippy::all)]
#[allow(unused_must_use, unused_doc_comments)]
fn solve<R: BufRead, W: Write>(io: &mut IO<R, W>) -> Option<()> {
	let n = io.get(0usize)?;
	let mut conn = vec![vec![]; n];
	let mut anticonn = vec![vec![]; n];
	let mut matrix = vec![vec![false; n]; n];
	for i in 0..n {
		let k = io.get(0usize)?;
		let v = io.get(vec![0usize; k])?;
		let mut non = 0;
		for x in v {
			conn[i].push(x as u16 - 1);
			matrix[i][x - 1] = true;
			for y in non..x - 1 {
				if y != i {
					anticonn[i].push(y as u16);
				}
			}
			non = x;
		}
		for y in non..n {
			if y != i {
				anticonn[i].push(y as u16);
			}
		}
	}
	#[derive(Clone, Copy, PartialEq, Eq, Debug)]
	enum State {
		Unused,
		Clique,
		Anti,
	}
	use State::*;
	let mut used = vec![Unused; n];
	let mut clique = vec![];
	let mut anticlique = vec![];
	let degmin = (0..n)
		.filter(|&i| used[i] == Unused)
		.min_by_key(|&i| conn[i].len())
		.unwrap();
	let degmax = (0..n)
		.filter(|&i| used[i] == Unused)
		.max_by_key(|&i| conn[i].len())
		.unwrap();
	if conn[degmin].len() == n - 1 || conn[degmax].len() == 0 {
		io.put(n);
		None?
	}
	let mut extra = 0;
	while let Some(degmin)
		= (0..n).filter(|&i| used[i] == Unused).min_by_key(|&i| conn[i].len())
	{
		let degmax = (0..n)
			.filter(|&i| used[i] == Unused)
			.max_by_key(|&i| conn[i].len())
			.unwrap();
		if conn[degmax].len() == clique.len() {
			io.put(n - clique.len() - anticlique.len() + 1 + extra);
			None?
		}
		if conn[degmin].len() == n - anticlique.len() - 1 {
			io.put(n - clique.len() - anticlique.len() + 1);
			None?
		}
		let mut anti_q = VecDeque::from(vec![degmin]);
		let mut cliq_q = VecDeque::new();
		used[degmin] = Anti;
		extra = 1;
		while let Some(anti) = anti_q.pop_front() {
			for &prev_anti in &anticlique {
				if matrix[anti][prev_anti] {
					io.put('0');
					None?
				}
			}
			anticlique.push(anti);
			for &cli in &conn[anti] {
				let cli = cli as usize;
				if used[cli] == Clique {
					continue;
				}
				if used[cli] == Anti {
					io.put('0');
					None?
				}
				used[cli] = Clique;
				cliq_q.push_back(cli);
				if anti != degmin {
					extra = 0;
				}
			}
			while let Some(cli) = cliq_q.pop_front() {
				for &prev_cli in &clique {
					if matrix[cli][prev_cli] {} else {
						io.put('0');
						None?
					}
				}
				clique.push(cli);
				for &anti in &anticonn[cli] {
					let anti = anti as usize;
					if used[anti] == Anti {
						continue;
					}
					if used[anti] == Clique {
						io.put('0');
						None?
					}
					used[anti] = Anti;
					anti_q.push_back(anti);
				}
			}
		}
	}
	io.put(1);
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
	print_disp!(usize, i64, String, & str, char);
	print_disp!(i16, i32, i128);
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
}
use io::*;
pub fn main() {
	let stdin = stdin().lock();
	let stdout = stdout().lock();
	let mut io = IO::new(stdin, stdout);
	solve(&mut io);
}
