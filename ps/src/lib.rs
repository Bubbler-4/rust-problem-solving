#![allow(dead_code)]
#![allow(unused_imports)]
use std::collections::*;
use std::cmp::{Reverse, Ordering, Ordering::*};

#[allow(clippy::all)]
#[allow(unused_must_use, unused_doc_comments)]
fn solve<R: BufRead, W: Write>(io: &mut IO<R, W>) -> Option<()> {
    
    /*
    cd ps
    ./go.sh 24444
    ./run.sh
    */
    None
}

#[cfg(test)]
mod test {
    use std::io::{BufRead, Write};
    use crate::{io::*, solve};
    use std::collections::*;
    #[allow(clippy::all)]
    #[allow(unused_must_use, unused_doc_comments)]
    fn solve2<R: BufRead, W: Write>(io: &mut IO<R, W>) -> Option<()> {
        let [r, c, k] = io.get([0usize; 3])?;
        let grid = io.get(vec![B; r])?;
        let mut ans = 0usize;
        // test all bitmasks
        'outer: for bits in 0..1usize<<(r*c) {
            let mut grid2 = grid.clone();
            let mut b = bits;
            let mut ts = 0usize;
            let mut somet = (r, c);
            let mut somed = (r, c);
            for rr in 0..r {
                for cc in 0..c {
                    if b % 2 == 0 { grid2[rr][cc] = b'T'; ts += 1; somet = (rr, cc); }
                    else { grid2[rr][cc] = b'D'; somed = (rr, cc); }
                    if grid[rr][cc] + grid2[rr][cc] == b'T' + b'D' { continue 'outer; }
                    b /= 2;
                }
            }
            if ts.abs_diff(r*c-ts) > k { continue; }
            for rr in 0..r-1 {
                for cc in 0..c-1 {
                    if grid2[rr][cc] == grid2[rr][cc+1] && grid2[rr][cc] == grid2[rr+1][cc] && grid2[rr][cc] == grid2[rr+1][cc+1] { continue 'outer; }
                }
            }
            let mut stepped = vec![vec![false; c]; r];
            if somed != (r, c) {
                let mut q = VecDeque::from(vec![somed]);
                stepped[somed.0][somed.1] = true;
                while let Some((rr, cc)) = q.pop_front() {
                    for (rrr, ccc) in [(rr-1, cc), (rr+1, cc), (rr, cc-1), (rr, cc+1)] {
                        if rrr < r && ccc < c && !stepped[rrr][ccc] && grid2[rrr][ccc] != b'T' {
                            stepped[rrr][ccc] = true;
                            q.push_back((rrr, ccc));
                        }
                    }
                }
            }
            if somet != (r, c) {
                let mut q = VecDeque::from(vec![somet]);
                stepped[somet.0][somet.1] = true;
                while let Some((rr, cc)) = q.pop_front() {
                    for (rrr, ccc) in [(rr-1, cc), (rr+1, cc), (rr, cc-1), (rr, cc+1)] {
                        if rrr < r && ccc < c && !stepped[rrr][ccc] && grid2[rrr][ccc] != b'D' {
                            stepped[rrr][ccc] = true;
                            q.push_back((rrr, ccc));
                        }
                    }
                }
            }
            if stepped.iter().flatten().all(|x| *x) {
                ans += 1;
            }
        }
        io.put(ans);
        None
    }

    #[test]
    #[allow(unused_must_use)]
    fn check() {
        use std::io::Write;
        for r in 3..=3 {
            for c in 4..=4 {
                for pat in 0..2usize.pow((r*c) as u32) {
                    let mut grid = vec![vec![0u8; c]; r];
                    let mut pat = pat;
                    for rr in 0..r {
                        for cc in 0..c {
                            grid[rr][cc] = b"DT"[pat % 2];
                            pat /= 2;
                        }
                    }
                    for k in r*c..=r*c {
                        let mut input: Vec<u8> = vec![];
                        writeln!(input, "{} {} {}", r, c, k);
                        for row in &grid {
                            input.write(row);
                            writeln!(input);
                        }
                        let mut output1: Vec<u8> = vec![];
                        let mut output2: Vec<u8> = vec![];
                        let mut io1 = IO::new(&input[..], &mut output1);
                        solve(&mut io1);
                        let mut io2 = IO::new(&input[..], &mut output2);
                        solve2(&mut io2);
                        drop(io1); drop(io2);
                        assert_eq!(output1, output2, "failed on:\n{}; solve: {}, solve2: {}",
                            String::from_utf8_lossy(&input),
                            String::from_utf8_lossy(&output1),
                            String::from_utf8_lossy(&output2),
                        );
                    }
                }
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
mod flow;
mod lca;
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