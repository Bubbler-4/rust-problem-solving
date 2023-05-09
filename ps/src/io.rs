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
impl Fill for f64 {
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
impl Fill for u16 {
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
impl Fill for u32 {
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
impl Fill for u128 {
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
impl Fill for i16 {
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
impl Fill for i32 {
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
impl Fill for i128 {
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
impl<T: Fill, U: Fill> Fill for (T, U) {
    fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
        #[allow(non_snake_case)]
        let (T, U) = self;
        T.fill_from_input(i)?;
        U.fill_from_input(i)?;
        Some(())
    }
}
impl<T: Fill, U: Fill, V: Fill> Fill for (T, U, V) {
    fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
        #[allow(non_snake_case)]
        let (T, U, V) = self;
        T.fill_from_input(i)?;
        U.fill_from_input(i)?;
        V.fill_from_input(i)?;
        Some(())
    }
}
impl<T: Fill, U: Fill, V: Fill, W: Fill> Fill for (T, U, V, W) {
    fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
        #[allow(non_snake_case)]
        let (T, U, V, W) = self;
        T.fill_from_input(i)?;
        U.fill_from_input(i)?;
        V.fill_from_input(i)?;
        W.fill_from_input(i)?;
        Some(())
    }
}
impl<T: Fill, U: Fill, V: Fill, W: Fill, X: Fill> Fill for (T, U, V, W, X) {
    fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
        #[allow(non_snake_case)]
        let (T, U, V, W, X) = self;
        T.fill_from_input(i)?;
        U.fill_from_input(i)?;
        V.fill_from_input(i)?;
        W.fill_from_input(i)?;
        X.fill_from_input(i)?;
        Some(())
    }
}
impl<T: Fill, U: Fill, V: Fill, W: Fill, X: Fill, Y: Fill> Fill
for (T, U, V, W, X, Y) {
    fn fill_from_input<R: BufRead>(&mut self, i: &mut I<R>) -> Option<()> {
        #[allow(non_snake_case)]
        let (T, U, V, W, X, Y) = self;
        T.fill_from_input(i)?;
        U.fill_from_input(i)?;
        V.fill_from_input(i)?;
        W.fill_from_input(i)?;
        X.fill_from_input(i)?;
        Y.fill_from_input(i)?;
        Some(())
    }
}