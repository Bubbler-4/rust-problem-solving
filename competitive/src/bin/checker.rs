#![allow(dead_code)]
#![allow(unused_macros)]
use std::str::{Split, Lines, FromStr};
use std::any::Any;
use std::fmt::Display;

// exit codes
// OK: accepted, WA: wrong answer
// FAIL: unexpected situation (e.g. contestant found better sol than reference)
type ExitCode = i32;
const OK: ExitCode = 0;
const WA: ExitCode = 1;
const FAIL: ExitCode = 3;

macro_rules! wa {
    ($($t: tt)*) => { Err((WA, format!($($t)*))) }
}
macro_rules! fail {
    ($($t: tt)*) => { Err((FAIL, format!($($t)*))) }
}
macro_rules! err {
    ($code: expr, $($t: tt)*) => { Err(($code, format!($($t)*))) }
}

fn main() {
    std::process::exit({
        let mut args = std::env::args().skip(1);
        let input = std::fs::read_to_string(args.next().unwrap()).unwrap();
        let output = std::fs::read_to_string(args.next().unwrap()).unwrap();
        let reference = std::fs::read_to_string(args.next().unwrap()).unwrap();

        match test_main(&input, &output, &reference) {
            Ok(_) => { println!("OK"); 0 }
            Err((exitcode, s)) => { println!("{}", s); exitcode }
        }
    })
}
 
fn test_main(input: &str, output: &str, reference: &str) -> Result<(), (ExitCode, String)> {
    let mut input = Parser::new(input, FAIL)?;
    let mut output = Parser::new(output, WA)?;
    let mut reference = Parser::new(reference, FAIL)?;
    let x: i64 = input.tok_val()?;
    let y: i64 = input.tok_val()?;
    let o_cnt: usize = output.tok_val()?;
    //output.nl()?;
    let r_cnt: usize = reference.tok_val()?;
    if o_cnt > r_cnt { wa!("Move count not optimal")? }
    let mut last_coord = [0i64, 0];
    let mut set = std::collections::HashSet::new();
    for _ in 0..o_cnt {
        output.nl()?;
        let next_coord = [output.tok_val::<i64>()?, output.tok_val()?];
        if set.contains(&next_coord) { wa!("Moved into a square twice")? }
        let diffs = [(last_coord[0]-next_coord[0]).abs(), (last_coord[1]-next_coord[1]).abs()];
        if diffs != [x, y] && diffs != [y, x] { wa!("Illegal kknight move found")? }
        set.insert(last_coord);
        last_coord = next_coord;
    }
    let [last_x, last_y] = last_coord;
    let cands = [
        [last_x + x, last_y + y],
        [last_x + y, last_y + x],
        [last_x + y, last_y - x],
        [last_x + x, last_y - y],
        [last_x - x, last_y - y],
        [last_x - y, last_y - x],
        [last_x - y, last_y + x],
        [last_x - x, last_y + y]
    ];
    if cands.iter().any(|cand| !set.contains(cand)) { wa!("Available kknight move found after last move")? }
    if o_cnt < r_cnt { fail!("Move count better than reference")? }
    output.eof()?;
    Ok(())
}
 
struct Parser<'a> {
    lines: Lines<'a>,
    tokens: Split<'a, char>,
    cur_line: usize,
    cur_tok: usize,
    exitcode: ExitCode,
}

impl<'a> Parser<'a> {
    fn new(s: &'a str, exitcode: ExitCode) -> Result<Self, (ExitCode, String)> {
        let mut lines = s.lines();
        match lines.next() {
            None => wa!("Unexpected empty input"),
            Some(line) => {
                let tokens = line.split(' ');
                let cur_line = 1;
                let cur_tok = 1;
                Ok(Self { lines, tokens, cur_line, cur_tok, exitcode })
            }
        }
    }
 
    fn nl(&mut self) -> Result<(), (ExitCode, String)> {
        // verify that tokens is empty and next line exists, and then move into the line
        let cur_line = self.cur_line;
        let cur_tok = self.cur_tok;
        match self.tokens.next() {
            None => (),
            Some(_) => err!(self.exitcode, "Line #{}: Expected eol, found token #{}", cur_line, cur_tok)?,
        }
        match self.lines.next() {
            None => err!(self.exitcode, "Line #{}: Expected nl, found eof", cur_line)?,
            Some(line) => {
                self.tokens = line.split(' ');
                self.cur_line += 1;
                self.cur_tok = 1;
            }
        }
        Ok(())
    }
 
    fn eof(&mut self) -> Result<(), (ExitCode, String)> {
        // verify that tokens and lines are both empty
        let cur_line = self.cur_line;
        let cur_tok = self.cur_tok;
        match self.tokens.next() {
            None => (),
            Some(_) => err!(self.exitcode, "Line #{}: Expected eof, found token #{}", cur_line, cur_tok)?,
        }
        match self.lines.next() {
            None => (),
            Some(_) => err!(self.exitcode, "Line #{}: Expected eof, found nl", cur_line)?,
        }
        Ok(())
    }
 
    fn tok(&mut self) -> Result<String, (ExitCode, String)> {
        let cur_line = self.cur_line;
        let cur_tok = self.cur_tok;
        match self.tokens.next() {
            None => err!(self.exitcode, "Line #{}: Expected token #{}, found eol", cur_line, cur_tok)?,
            Some(tok) => {
                self.cur_tok += 1;
                Ok(tok.to_owned())
            }
        }
    }
 
    fn tok_val<T: Any + FromStr>(&mut self) -> Result<T, (ExitCode, String)> {
        let cur_line = self.cur_line;
        let cur_tok = self.cur_tok;
        let tok = self.tok()?;
        match tok.parse::<T>() {
            Ok(v) => Ok(v),
            Err(_) => err!(self.exitcode, "Line #{}: Malformed token #{} '{}' of type {}", cur_line, cur_tok, tok, std::any::type_name::<T>())?,
        }
    }
 
    fn tok_range<T>(&mut self, low: T, high: T) -> Result<T, (ExitCode, String)>
    where T: Any + FromStr + Ord + Copy + Display {
        let cur_line = self.cur_line;
        let cur_tok = self.cur_tok;
        let v: T = self.tok_val()?;
        if low <= v && v <= high { Ok(v) }
        else { err!(self.exitcode, "Line #{}: Token #{} '{}' out of range (expected {}..{})", cur_line, cur_tok, v, low, high) }
    }
 
    fn test(&self, f: impl Fn() -> bool, s: &str) -> Result<(), (ExitCode, String)> {
        let cur_line = self.cur_line;
        if f() { Ok(()) }
        else { err!(self.exitcode, "Line #{}: Assertion failed ({})", cur_line, s) }
    }
}
