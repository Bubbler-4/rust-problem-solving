#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::*;
use std::cmp::{Reverse, Ordering, Ordering::*};

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
            matrix[i][x-1] = true;
            for y in non..x-1 {
                if y != i { anticonn[i].push(y as u16); }
            }
            non = x;
        }
        for y in non..n {
            if y != i { anticonn[i].push(y as u16); }
        }
    }
    // eprintln!("{:?}", conn);
    // eprintln!("{:?}", anticonn);
    // eprintln!("{:?}", matrix);
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum State { Unused, Clique, Anti }
    use State::*;
    let mut used = vec![Unused; n];
    let mut clique = vec![];
    let mut anticlique = vec![];
    let degmin = (0..n).filter(|&i| used[i] == Unused).min_by_key(|&i| conn[i].len()).unwrap();
    let degmax = (0..n).filter(|&i| used[i] == Unused).max_by_key(|&i| conn[i].len()).unwrap();
    if conn[degmin].len() == n-1 || conn[degmax].len() == 0 { io.put(n); None? }
    let mut extra = 0;
    while let Some(degmin) = (0..n).filter(|&i| used[i] == Unused).min_by_key(|&i| conn[i].len()) {
        let degmax = (0..n).filter(|&i| used[i] == Unused).max_by_key(|&i| conn[i].len()).unwrap();
        if conn[degmax].len() == clique.len() { io.put(n - clique.len() - anticlique.len() + 1 + extra); None? }
        if conn[degmin].len() == n - anticlique.len() - 1 { io.put(n - clique.len() - anticlique.len() + 1); None? }
        let mut anti_q = VecDeque::from(vec![degmin]);
        let mut cliq_q = VecDeque::new();
        used[degmin] = Anti;
        extra = 1;
        while let Some(anti) = anti_q.pop_front() {
            // eprintln!("anti {}", anti);
            for &prev_anti in &anticlique {
                if matrix[anti][prev_anti] { io.put('0'); None? }
            }
            anticlique.push(anti);
            for &cli in &conn[anti] {
                let cli = cli as usize;
                if used[cli] == Clique { continue; }
                if used[cli] == Anti { io.put('0'); None? }
                used[cli] = Clique;
                cliq_q.push_back(cli);
                if anti != degmin { extra = 0; }
            }
            while let Some(cli) = cliq_q.pop_front() {
                // eprintln!("cli {}", cli);
                for &prev_cli in &clique {
                    if matrix[cli][prev_cli] {} else { io.put('0'); None? }
                }
                clique.push(cli);
                for &anti in &anticonn[cli] {
                    let anti = anti as usize;
                    // eprintln!("inner anti {}", anti);
                    if used[anti] == Anti { continue; }
                    if used[anti] == Clique { io.put('0'); None? }
                    used[anti] = Anti;
                    anti_q.push_back(anti);
                }
            }
        }
    }
    io.put(1);
    None
}

#[cfg(test)]
mod test {
    use std::io::{BufRead, Write};
    use crate::{io::IO, solve};
    #[allow(clippy::all)]
    #[allow(unused_must_use, unused_doc_comments)]
    fn solve2<R: BufRead, W: Write>(io: &mut IO<R, W>) -> Option<()> {
        let n = io.get(0usize)?;
        let mut matrix = vec![vec![false; n]; n];
        for i in 0..n {
            let k = io.get(0usize)?;
            let v = io.get(vec![0usize; k])?;
            for x in v { matrix[i][x-1] = true; }
        }
        let mut ans = 0usize;
        let mut pats = vec![];
        'outer: for bits in 1..(1usize<<n)-1 {
            for i in 0..n {
                let bit_i = (bits >> i) & 1;
                for j in i+1..n {
                    let bit_j = (bits >> j) & 1;
                    if bit_i == 1 && bit_j == 1 {
                        if !matrix[i][j] { continue 'outer; }
                    }
                    if bit_i == 0 && bit_j == 0 {
                        if matrix[i][j] { continue 'outer; }
                    }
                }
            }
            ans += 1;
            pats.push(bits);
        }
        eprintln!("{:?}", pats);
        io.put(ans);
        None
    }

    #[test]
    #[allow(unused_must_use)]
    fn check() {
        use std::io::Write;
        for n in 2usize..=5 {
            let edges = (0..n).flat_map(|i| (i+1..n).map(move |j| (i, j))).collect::<Vec<_>>();
            for bits in 0usize..1<<edges.len() {
                let mut input: Vec<u8> = vec![];
                let mut desc = vec![vec![]; n];
                for b in 0..edges.len() {
                    if ((bits >> b) & 1) == 1 { let (x, y) = edges[b]; desc[x].push(y); desc[y].push(x); }
                }
                write!(input, "{}\n", n);
                for row in desc {
                    write!(input, "{}", row.len());
                    for x in row { write!(input, " {}", x + 1); }
                    write!(input, "\n");
                }
                let input1 = input;
                let mut output1: Vec<u8> = vec![];
                let mut output2: Vec<u8> = vec![];
                let mut io1 = IO::new(&input1[..], &mut output1);
                solve(&mut io1);
                let mut io2 = IO::new(&input1[..], &mut output2);
                solve2(&mut io2);
                drop(io1); drop(io2);
                assert_eq!(output1, output2, "failed on:\n{}; solve: {}, solve2: {}",
                    String::from_utf8_lossy(&input1),
                    String::from_utf8_lossy(&output1),
                    String::from_utf8_lossy(&output2),
                );
            }
        }
    }
}

mod prime;
mod ntheory;
mod sequence;
mod graph;
mod geometry;
mod string;
mod fft;
/// IO template
mod io;
use io::*;

pub fn main() {
    let stdin = stdin().lock();
    // let mut ii = I::new(stdin.lock());
    let stdout = stdout().lock();
    // let mut oo = BufWriter::new(stdout);
    let mut io = IO::new(stdin, stdout);
    solve(&mut io);
}