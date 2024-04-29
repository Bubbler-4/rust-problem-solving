#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_doc_comments)]
use std::collections::*;
use std::cmp::{Reverse, Ordering, Ordering::*};
use std::iter::once_with;

#[allow(clippy::all)]
#[allow(unused_must_use, unused_doc_comments)]
fn solve<R: BufRead, W: Write, E: Write>(io: &mut IO<R, W, E>) -> Option<()> {
    let n = io.get(0usize)?;
    let mut v = vec![];
    let mut blanks = 0usize;
    let mut unused = (1..=n).collect::<BTreeSet<_>>();
    for _ in 0..n {
        let x = io.get(0i64)?;
        if x == -1 { v.push(usize::MAX); blanks += 1; }
        else { v.push(x as usize); unused.remove(&(x as usize)); }
    }
    let mut next = 0usize;
    while blanks > 8 {
        while v[next] != 0 { next += 1; }
        let mut to_insert = 0usize;
        for &x in &unused {
            if next > 0 && v[next-1].abs_diff(x) == 1 { continue; }
            if v[next+1].abs_diff(x) == 1 { continue; }
            to_insert = x; break;
        }
        v[next] = to_insert;
        unused.remove(&to_insert);
    }
    fn backtrack(v: &mut [usize], indices: &[usize], values: &[usize]) -> bool {
        if indices.is_empty() { return true; }
        let mut values2 = values.to_vec();
        let idx = indices[0];
        for i in 0..values2.len() {
            let val = values2[i];
            if (idx > 0 && v[idx-1].abs_diff(val) == 1) || (idx < v.len()-1 && v[idx+1].abs_diff(val) == 1) {
                continue;
            }
            values2.swap(0, i);
            if backtrack(v, &indices[1..], &values2[1..]) { return true; }
            values2.swap(0, i);
        }
        false
    }
    let mut indices = vec![];
    for i in 0..n {
        if v[i] == usize::MAX { indices.push(i); }
    }
    let values = unused.into_iter().collect::<Vec<_>>();
    if backtrack(&mut v, &indices, &values) {
        io.put(v);
    } else {
        io.put(-1);
    }
    None
}

#[cfg(test)]
mod test {
    use std::io::{BufRead, Write};
    use rand::Rng;

    use crate::{io::*, solve};
    use std::collections::*;
    #[allow(clippy::all)]
    #[allow(unused_must_use, unused_doc_comments)]
    fn solve2<R: BufRead, W: Write, E: Write>(io: &mut IO<R, W, E>) -> Option<()> {
        let n = io.get(0usize)?;
        let grid = io.get(vec![B; n])?;
        for i in 0..n {
            if grid[i][i] == b'1' { io.put(-1); None? }
        }
        let mut out_edges = vec![vec![]; n];
        let mut in_edges = vec![vec![]; n];
        let mut edges = vec![];
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == b'1' {
                    out_edges[i].push(j);
                    in_edges[j].push(i);
                    edges.push((i, j));
                }
            }
        }
        if crate::graph::toposort(n, &edges).is_none() { io.put(-1); None? }
        for mut i in 0..n.pow(n as u32) {
            let mut ids = vec![0usize; n];
            for j in 0..n {
                ids[n-1-j] = i % n + 1;
                i /= n;
            }
            let mut v = ids.clone();
            v.sort_unstable();
            v.dedup();
            if v.len() != n { continue; }
            if edges.iter().all(|&(x, y)| ids[x] < ids[y]) {
                io.put(ids); None?
            }
        }
        None
    }

    #[test]
    #[allow(unused_must_use)]
    fn check() {
        use std::io::Write;
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let mut r = rng.gen_range(1usize..=2);
            let mut c = rng.gen_range(1usize..=2);
            while r * c == 1 {
                r = rng.gen_range(1usize..=2);
                c = rng.gen_range(1usize..=2);
            }
            let mut grid = vec![vec![b'.'; c]; r];
            let mut cells = vec![];
            for rr in 0..r {
                for cc in 0..c {
                    cells.push((rr, cc));
                }
            }
            for i in 0..cells.len() {
                let j = rng.gen_range(i..cells.len());
                cells.swap(i, j);
            }
            let start = cells[0];
            let end = cells[1];
            for rr in 0..r {
                for cc in 0..c {
                    grid[rr][cc] = b".#"[rng.gen_range(0usize..2)];
                }
            }
            grid[start.0][start.1] = b'A';
            grid[end.0][end.1] = b'B';
            let tl = rng.gen_range(0usize..=(r * c - 2).min(10));
            let traffic = (0..tl).map(|i| {
                let dir = b"-|"[rng.gen_range(0usize..2)];
                let hor = rng.gen_range(1usize..=20);
                let ver = rng.gen_range(1usize..=20);
                (i, dir, hor, ver)
            });
            for i in 0..tl {
                let (rr, cc) = cells[i + 2];
                grid[rr][cc] = i as u8 + b'0';
            }

            let mut input: Vec<u8> = vec![];
            writeln!(input, "{} {}", r, c);
            for row in grid { input.write(&row); input.push(b'\n'); }
            for (i, dir, hor, ver) in traffic {
                writeln!(input, "{} {} {} {}", i, dir as char, hor, ver);
            }
            writeln!(input, "0 0");
            // println!("{}", String::from_utf8_lossy(&input));

            let mut output1: Vec<u8> = vec![];
            let mut output2: Vec<u8> = vec![];
            let mut io1 = IO::new(&input[..], &mut output1, &mut output2);
            solve(&mut io1);
            drop(io1);
        }
        // for i in 0..1usize<<25 {
        //     let mut input: Vec<u8> = vec![];
        //     let n = 5;
        //     writeln!(input, "{}", n);
        //     let mut grid = vec![vec![b'0'; n]; n];
        //     for j in 0..25 {
        //         if i >> j & 1 != 0 {
        //             grid[j/n][j%n] += 1;
        //         }
        //     }
        //     for row in grid { input.write(&row); input.push(b'\n'); }
            
        //     let mut output1: Vec<u8> = vec![];
        //     let mut output2: Vec<u8> = vec![];
        //     let mut output3: Vec<u8> = vec![];
        //     let mut io1 = IO::new(&input[..], &mut output1, &mut output3);
        //     solve(&mut io1);
        //     drop(io1);
        //     let mut io2 = IO::new(&input[..], &mut output2, &mut output3);
        //     solve2(&mut io2);
        //     drop(io2);
        //     assert_eq!(output1, output2, "failed on:\n{}; solve: {}, solve2: {}",
        //         String::from_utf8_lossy(&input),
        //         String::from_utf8_lossy(&output1),
        //         String::from_utf8_lossy(&output2),
        //     );
        // }
    }
}

mod prime;
mod ntheory;
mod sequence;
mod graph;
mod geometry;
mod segtree;
mod string;
mod fft;
mod flow;
mod lca;
mod random;
/// IO template
mod io;
use io::*;

pub fn main() {
    let stdin = stdin().lock();
    // let mut ii = I::new(stdin.lock());
    let stdout = stdout().lock();
    // let mut oo = BufWriter::new(stdout);
    let stderr = stderr().lock();
    let mut io = IO::new(stdin, stdout, stderr);
    solve(&mut io);
    io.eput("");
}