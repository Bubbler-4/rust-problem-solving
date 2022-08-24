#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};
use std::io::{Read, Write, stdin, stdout, BufWriter};
use std::convert::TryInto;

type Is = dyn Iterator<Item=&'static str>;
struct I { i: Box<Is> }
impl I {
    fn new(s: &'static str) -> Self {
        let it = s.split_ascii_whitespace(); // change as necessary
        I { i: Box::new(it) }
    }
    fn g<T: Get>(&mut self) -> Option<T> { T::get(&mut self.i) }
}
macro_rules! g {
    ($i: ident, $t: ty) => { $i.g::<$t>() };
    ($i: ident, $t: ty, $e:expr $(, $ee:expr)*) => {
        {let n = $e;
        (|| {
            let mut vv = Vec::with_capacity(n);
            for _ in 0..n { vv.push(g!($i, $t $(, $ee)*)?); }
            Some(vv)
        })()}
    };
}
trait Get: Sized { fn get(it: &mut Is) -> Option<Self>; }
macro_rules! get {
    (p, $it:ident) => { $it.next()?.parse().ok() };
    (s, $it:ident) => { $it.next() };
    (b, $it:ident) => { Some($it.next()?.as_bytes()) };
    ($ty:ty, $tt:tt) => { impl Get for $ty { fn get(it: &mut Is) -> Option<Self> { get!($tt, it) }}};
}
get!(usize, p); get!(u128, p); get!(u32, p); get!(i64, p); get!(i128, p); get!(f64, p); get!(&str, s); get!(&[u8], b);
macro_rules! tup { ($($t:ident),*) => { impl<$($t: Get),*> Get for ($($t),*) { fn get(it: &mut Is) -> Option<Self> { Some(($($t::get(it)?),*)) }}}; }
tup!(T, U); tup!(T, U, V); tup!(T, U, V, W); tup!(T, U, V, W, X); tup!(T, U, V, W, X, Y);
impl<T: Get, const N: usize> Get for [T;N] { fn get(it: &mut Is) -> Option<Self> {
    let mut vv = Vec::with_capacity(N);
    for _ in 0..N { vv.push(T::get(it)?); }
    vv.try_into().ok()
}}
#[cfg(not(test))] macro_rules! dbg { ($($t:tt)*) => {}; }

fn main() {
    let stdin = stdin();
    let stdin = &mut stdin.lock();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();
    let buf = preprocess(buf);
    let buf = Box::leak(buf.into_boxed_str());
    let ii = I::new(buf);
    let stdout = stdout();
    let stdout = stdout.lock();
    let mut oo = BufWriter::new(stdout);
    solve(ii, &mut oo);
}

fn preprocess(buf: String) -> String {
    buf
    //.replace(' ', "@")
}

fn solve<W: Write>(mut ii: I, oo: &mut W) -> Option<()> {
    let n = g!(ii, usize)?;
    for t in 1..=n {
        let [m, k] = g!(ii, [usize; 2])?;
        let v = g!(ii, usize, m)?;
        let avgs = v.windows(k).map(|w| w.iter().sum::<usize>() / w.len()).collect::<Vec<_>>();
        let max = avgs.iter().max()?;
        let min = avgs.iter().min()?;
        writeln!(oo, "Data Set {}:", t);
        writeln!(oo, "{}", max - min);
        writeln!(oo);
    }
    None
}

#[cfg(test)]
mod test {
    const PROBLEM: usize = 5121;
    use console::Style;
    use similar::{ChangeTag, TextDiff};
    use reqwest::blocking::get;
    use scraper::{Html, Selector};
    //#[test]
    #[allow(dead_code)]
    fn custom() {
        fn check(rows: usize, cols: usize, r1: usize, c1: usize, r2: usize, c2: usize, ans: &str) {
            let mut lines = ans.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
            if lines[0] == b"NO" { return; }
            lines.remove(0);
            let error = |msg: &str| {
                println!("Input: {} {} {} {} {} {}", rows, cols, r1, c1, r2, c2);
                println!("Output: {}", ans);
                println!("{}", msg);
                assert!(false);
            };
            if lines.len() != rows { error("row count mismatch"); }
            if lines.iter().any(|l| l.len() != cols) { error("col count mismatch"); }
            if lines[r1-1][c1-1] != b'#' { error("r1c1 is not #"); }
            if lines[r2-1][c2-1] != b'.' { error("r2c2 is not ."); }
            let mut visited = vec![vec![false; cols]; rows];
            let mut q = std::collections::VecDeque::new();
            q.push_back((r1-1, c1-1));
            while let Some((r, c)) = q.pop_front() {
                if visited[r][c] { error("cycle found for #"); }
                visited[r][c] = true;
                if r > 0 && lines[r-1][c] == b'#' && !visited[r-1][c] { q.push_back((r-1, c)); }
                if r < rows-1 && lines[r+1][c] == b'#' && !visited[r+1][c] { q.push_back((r+1, c)); }
                if c > 0 && lines[r][c-1] == b'#' && !visited[r][c-1] { q.push_back((r, c-1)); }
                if c < cols-1 && lines[r][c+1] == b'#' && !visited[r][c+1] { q.push_back((r, c+1)); }
            }
            q.push_back((r2-1, c2-1));
            while let Some((r, c)) = q.pop_front() {
                if visited[r][c] { error("cycle found for ."); }
                visited[r][c] = true;
                if r > 0 && lines[r-1][c] == b'.' && !visited[r-1][c] { q.push_back((r-1, c)); }
                if r < rows-1 && lines[r+1][c] == b'.' && !visited[r+1][c] { q.push_back((r+1, c)); }
                if c > 0 && lines[r][c-1] == b'.' && !visited[r][c-1] { q.push_back((r, c-1)); }
                if c < cols-1 && lines[r][c+1] == b'.' && !visited[r][c+1] { q.push_back((r, c+1)); }
            }
            if visited.iter().any(|l| l.iter().any(|x| !*x)) { error("unreachable cell found"); }
        }
        // stress
        let max = 14;
        for rows in 2..=max {
            for cols in 2..=max {
                for r1 in 1..=rows {
                    for c1 in 1..=cols {
                        for r2 in 1..=rows {
                            for c2 in 1..=cols {
                                if (r1, c1) == (r2, c2) { continue; }
                                let input = format!("{} {} {} {} {} {}", rows, cols, r1, c1, r2, c2);
                                let ii = crate::I::new(Box::leak(input.into_boxed_str()));
                                let mut oo = std::io::Cursor::new(Vec::<u8>::new());
                                crate::solve(ii, &mut oo);
                                let result = unsafe { String::from_utf8_unchecked(oo.into_inner()) };
                                check(rows, cols, r1, c1, r2, c2, &result);
                            }
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn is_solved() {
        let url = format!("https://www.acmicpc.net/problem/{}", PROBLEM);
        let res = get(url).unwrap().text().unwrap();
        let html = Html::parse_document(&res);
        let spj_selector = Selector::parse("span.problem-label-spj").unwrap();
        let mut it = html.select(&spj_selector);
        let spj = it.next().is_some();
        let selector = Selector::parse("pre.sampledata").unwrap();
        let mut it = html.select(&selector);
        while let Some(inel) = it.next() {
            let output = it.next().unwrap().text().collect::<String>();
            let input = Box::leak(crate::preprocess(inel.text().collect::<String>()).into_boxed_str());
            let ii = crate::I::new(input);
            let mut oo = std::io::Cursor::new(Vec::<u8>::new());
            let now = std::time::Instant::now();
            crate::solve(ii, &mut oo);
            let elapsed = now.elapsed().as_micros();
            let result = unsafe { String::from_utf8_unchecked(oo.into_inner()) };
            let output = output.trim_end().lines().map(|l| l.trim_end()).collect::<Vec<_>>().join("\n");
            let result = result.trim_end().lines().map(|l| l.trim_end()).collect::<Vec<_>>().join("\n");
            let diff = TextDiff::from_lines(&result, &output);
            let styles = if spj { (Style::new(), Style::new(), Style::new()) } else { (Style::new().red(), Style::new().green(), Style::new()) };
            let mut failed = false;
            for op in diff.ops() {
                for change in diff.iter_changes(op) {
                    let (sign, style) = match change.tag() {
                        ChangeTag::Delete => { failed = true; ("-", &styles.0) },
                        ChangeTag::Insert => { failed = true; ("+", &styles.1) },
                        ChangeTag::Equal => (" ", &styles.2),
                    };
                    print!("{}{}", style.apply_to(sign), style.apply_to(change));
                }
            }
            if !spj && failed { assert!(false, "incorrect output"); }
            println!("Elapsed: {}.{:06}", elapsed / 1000000, elapsed % 1000000);
        }
    }
}