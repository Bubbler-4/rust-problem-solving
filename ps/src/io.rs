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
    pub(crate) fn sep<T: Print, U: Print, Arr: IntoIterator<Item=T>>(&mut self, arr: Arr, sep: U) -> &mut Self {
        let mut first = true;
        for t in arr {
            if !first { sep.print(&mut self.oo); }
            t.print(&mut self.oo);
            first = false;
        }
        self
    }
    pub(crate) fn sp(&mut self) -> &mut Self {
        self.put(" ")
    }
    pub(crate) fn nl(&mut self) -> &mut Self {
        self.put("\n")
    }
    pub(crate) fn flush(&mut self) -> &mut Self {
        self.oo.flush().unwrap();
        self
    }
}
pub(crate) trait Print {
    fn print<W: Write>(&self, w: &mut W);
}
macro_rules! print_disp {
    ($($t:ty),+) => {
        $(
            impl Print for $t {
                fn print<W: Write>(&self, w: &mut W) {
                    write!(w, "{}", self).unwrap();
                }
            }
        )+
    }
}
print_disp!(usize, i64, String, &str, char);
print_disp!(u16, u32, u64, u128);
print_disp!(i16, i32, i128);
impl Print for (f64, usize) {
    fn print<W: Write>(&self, w: &mut W) {
        write!(w, "{:.*}", self.1, self.0).unwrap();
    }
}
impl Print for (usize, f64) {
    fn print<W: Write>(&self, w: &mut W) {
        write!(w, "{:.*}", self.0, self.1).unwrap();
    }
}
impl Print for [u8] {
    fn print<W: Write>(&self, w: &mut W) {
        w.write(self).unwrap();
    }
}
impl<T: Print> Print for [T] {
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
impl<T: Print + ?Sized> Print for &T {
    fn print<W: Write>(&self, w: &mut W) {
        (*self).print(w);
    }
}
impl<T> Print for Vec<T> where [T]: Print {
    fn print<W: Write>(&self, w: &mut W) {
        self[..].print(w);
    }
}
impl<T, const N: usize> Print for [T; N] where [T]: Print {
    fn print<W: Write>(&self, w: &mut W) {
        self[..].print(w);
    }
}
macro_rules! print_tuple {
    ($(($t: ident, $($u: ident),+))+) => {
        $(impl<$t: Print, $($u: Print),+> Print for ($t, $($u),+) {
            fn print<WW: Write>(&self, w: &mut WW) {
                #[allow(non_snake_case)]
                let ($t, $($u),+) = self;
                $t.print(w);
                $(w.write(b" ").unwrap(); $u.print(w);)+
            }
        })+
    }
}
print_tuple!((T, U) (T, U, V) (T, U, V, W) (T, U, V, W, X) (T, U, V, W, X, Y));
impl<'a> Print for std::fmt::Arguments<'a> {
    fn print<W: Write>(&self, w: &mut W) {
        w.write_fmt(*self).unwrap();
    }
}
#[macro_export]
macro_rules! f {
    ($($t: tt)*) => { format_args!($($t)*)}
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
    ($($t: ty),+) => {
        $(impl Fill for $t {
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
        })+
    }
}
fill_num!(usize, i64, f64);
fill_num!(u16, u32, u128);
fill_num!(i16, i32, i128);
#[derive(Clone)]
pub(crate) struct Line<T, const B: bool>(pub T);
impl Fill for String {
    fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
        i.rem = i.rem.trim_start_matches(WS);
        while i.rem.is_empty() {
            i.next_line()?;
            i.rem = i.rem.trim_start_matches(WS);
        }
        let tok = i.rem.split(WS).next().unwrap();
        i.rem = &i.rem[tok.len()..];
        self.push_str(tok);
        Some(())
    }
}
impl<const B: bool> Fill for Line<String, B> {
    fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
        if B {
            i.next_line()?;
        }
        let s = i.rem.strip_suffix('\n').unwrap_or(i.rem);
        i.rem = "";
        self.0.push_str(s);
        Some(())
    }
}
pub(crate) const S: String = String::new();
pub(crate) const L: Line<String, false> = Line(S);
pub(crate) const N: Line<String, true> = Line(S);
impl Fill for Vec<u8> {
    fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
        i.rem = i.rem.trim_start_matches(WS);
        while i.rem.is_empty() {
            i.next_line()?;
            i.rem = i.rem.trim_start_matches(WS);
        }
        let tok = i.rem.split(WS).next().unwrap();
        i.rem = &i.rem[tok.len()..];
        self.extend_from_slice(tok.as_bytes());
        Some(())
    }
}
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
pub(crate) const LB: Line<Vec<u8>, false> = Line(B);
pub(crate) const NB: Line<Vec<u8>, true> = Line(B);
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
macro_rules! fill_tuple {
    ($(($($t: ident),+))+) => {
        $(impl<$($t: Fill),+> Fill for ($($t),+) {
            fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
                #[allow(non_snake_case)]
                let ($($t),+) = self;
                $($t.fill_from_input(i)?;)+
                Some(())
            }
        })+
    }
}
fill_tuple!((T, U) (T, U, V) (T, U, V, W) (T, U, V, W, X) (T, U, V, W, X, Y));