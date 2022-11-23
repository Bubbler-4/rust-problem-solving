use std::io::{prelude::*, stdin};
use std::str::{Split, Lines, FromStr};
use std::any::Any;
use std::fmt::Display;
 
fn main() {
    std::process::exit({
        let stdin = stdin();
        let stdin = &mut stdin.lock();
        let mut buf = String::with_capacity(1<<20);
        stdin.read_to_string(&mut buf).unwrap();
        match test_main(&buf) {
            Ok(_) => { println!("OK"); 0 }
            Err(s) => { println!("{}", s); 1 }
        }
    })
}
 
fn test_main(input: &str) -> Result<(), String> {
    let mut parser = Parser::new(input)?;
    let n = parser.tok_range(2usize, 10)?;
    parser.test(|| n % 2 == 0, "n is not even")?;
    let mut sum = 0;
    for _ in 0..n {
        parser.nl()?;
        for _ in 0..n {
            let color = parser.tok_range(0usize, 1)?;
            sum += color;
        }
    }
    parser.test(|| sum == n * n / 2, "#white and #gray are not equal")?;
    parser.nl()?;
    let k = parser.tok_range(0usize, n * n)?;
    let mut pts = vec![];
    for _ in 0..k {
        parser.nl()?;
        let r = parser.tok_range(1, n)?;
        let c = parser.tok_range(1, n)?;
        let _k = parser.tok_range(1, n*n/2)?;
        pts.push([r, c]);
    }
    pts.sort_unstable();
    pts.dedup();
    parser.test(|| pts.len() == k, "duplicate points in number list")?;
    let mut grid = vec![];
    for _ in 0..n*2+1 {
        parser.nl()?;
        let line = parser.tok()?;
        grid.push(line.as_bytes().to_vec());
    }
    parser.test(|| grid.iter().all(|row| row.len() == n*2+1), "incorrect row length")?;
    for r in 0..=n*2 {
        for c in 0..=n*2 {
            if r%2==0 && c%2==0 { parser.test(|| grid[r][c] == b'+', "format spec 1 violation")?; }
            else if r%2==1 && c%2==1 { parser.test(|| grid[r][c] == b'.', "format spec 2 violation")?; }
            else if r%2==0 && c%2==1 {
                parser.test(|| grid[r][c] == b'.' || grid[r][c] == b'-', "format spec 3-1 violation")?;
                if r == 0 || r == 2*n {
                    parser.test(|| grid[r][c] == b'-', "format spec 3-2 violation")?;
                }
            }
            else if r%2==1 && c%2==0 {
                parser.test(|| grid[r][c] == b'.' || grid[r][c] == b'|', "format spec 4-1 violation")?;
                if c == 0 || c == 2*n {
                    parser.test(|| grid[r][c] == b'|', "format spec 4-2 violation")?;
                }
            }
        }
    }
    parser.eof()?;
    Ok(())
}
 
struct Parser<'a> {
    lines: Lines<'a>,
    tokens: Split<'a, char>,
    cur_line: usize,
    cur_tok: usize,
}

#[allow(dead_code)]
impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Result<Self, String> {
        let mut lines = s.lines();
        match lines.next() {
            None => unreachable!(),
            Some(line) => {
                let tokens = line.split(' ');
                let cur_line = 1;
                let cur_tok = 1;
                Ok(Self { lines, tokens, cur_line, cur_tok })
            }
        }
    }
 
    fn nl(&mut self) -> Result<(), String> {
        // verify that tokens is empty and next line exists, and then move into the line
        let cur_line = self.cur_line;
        let cur_tok = self.cur_tok;
        match self.tokens.next() {
            None => (),
            Some(_) => Err(format!("Line #{}: Expected eol, found token #{}", cur_line, cur_tok))?,
        }
        match self.lines.next() {
            None => Err(format!("Line #{}: Expected nl, found eof", cur_line))?,
            Some(line) => {
                self.tokens = line.split(' ');
                self.cur_line += 1;
                self.cur_tok = 1;
            }
        }
        Ok(())
    }
 
    fn eof(&mut self) -> Result<(), String> {
        // verify that tokens and lines are both empty
        let cur_line = self.cur_line;
        let cur_tok = self.cur_tok;
        match self.tokens.next() {
            None => (),
            Some(_) => Err(format!("Line #{}: Expected eof, found token #{}", cur_line, cur_tok))?,
        }
        match self.lines.next() {
            None => (),
            Some(_) => Err(format!("Line #{}: Expected eof, found nl", cur_line))?,
        }
        Ok(())
    }
 
    fn tok(&mut self) -> Result<String, String> {
        let cur_line = self.cur_line;
        let cur_tok = self.cur_tok;
        match self.tokens.next() {
            None => Err(format!("Line #{}: Expected token #{}, found eol", cur_line, cur_tok))?,
            Some(tok) => {
                self.cur_tok += 1;
                Ok(tok.to_owned())
            }
        }
    }
 
    fn tok_val<T: Any + FromStr>(&mut self) -> Result<T, String> {
        let cur_line = self.cur_line;
        let cur_tok = self.cur_tok;
        let tok = self.tok()?;
        match tok.parse::<T>() {
            Ok(v) => Ok(v),
            Err(_) => Err(format!("Line #{}: Malformed token #{} '{}' of type {}", cur_line, cur_tok, tok, std::any::type_name::<T>()))?,
        }
    }
 
    fn tok_range<T>(&mut self, low: T, high: T) -> Result<T, String>
    where T: Any + FromStr + Ord + Copy + Display {
        let cur_line = self.cur_line;
        let cur_tok = self.cur_tok;
        let v: T = self.tok_val()?;
        if low <= v && v <= high { Ok(v) }
        else { Err(format!("Line #{}: Token #{} '{}' out of range (expected {}..{})", cur_line, cur_tok, v, low, high)) }
    }
 
    fn test(&self, f: impl Fn() -> bool, s: &str) -> Result<(), String> {
        let cur_line = self.cur_line;
        if f() { Ok(()) }
        else { Err(format!("Line #{}: Assertion failed ({})", cur_line, s)) }
    }
}
