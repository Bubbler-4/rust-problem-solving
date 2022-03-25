#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

///////////////////////////////////////////////////////////////////////
/// SPFA (single source all destination, works with negative edges, but cannot detect neg.cycles)
/// weighted adjacency list -> (dist, prev)
fn spfa(n: usize, g: &[Vec<(usize, i64)>], s: usize) -> (Vec<i64>, Vec<usize>) {
    let mut d = vec![i64::MAX; n];
    let mut pred = vec![usize::MAX; n];
    d[s] = 0;
    pred[s] = s;
    let mut q = VecDeque::new();
    q.push_back(s);
    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        for &(v, w) in &g[u] {
            if d[u] + w < d[v] {
                d[v] = d[u] + w;
                pred[v] = u;
                if !q.contains(&v) { q.push_back(v); }
            }
        }
    }
    (d, pred)
}

///////////////////////////////////////////////////////////////////////
/// Dijkstra (single source all destination, fast, doesn't work with negative edges)
/// weighted adjacency list -> (dist, prev)
fn dijkstra(n: usize, g: &[Vec<(usize, usize)>], s: usize) -> (Vec<usize>, Vec<usize>) {
    let mut dist = vec![usize::MAX; n];
    dist[s] = 0;
    let mut prev = vec![usize::MAX; n];
    prev[s] = s;
    let mut finalized = vec![false; n];
    // min heap storing (distance, node)
    let mut q = BinaryHeap::<Reverse<(usize, usize)>>::new();
    q.push(Reverse((0, s)));
    while !q.is_empty() {
        let Reverse((_d, node)) = q.pop().unwrap();
        if finalized[node] { continue; }
        finalized[node] = true;
        for &(m, w) in &g[node] {
            if finalized[m] { continue; }
            let newdist = dist[node] + w;
            if newdist < dist[m] {
                dist[m] = newdist;
                prev[m] = node;
                q.push(Reverse((newdist, m)));
            }
        }
    }
    (dist, prev)
}

///////////////////////////////////////////////////////////////////////
/// Bellman-Ford (single source all destination, works with negative edges and detects neg.cycles)
/// weighted adjacency list -> (dist, prev, has_neg_cycle)
fn bellman_ford(n: usize, g: &[Vec<(usize, i64)>], s: usize) -> (Vec<i64>, Vec<usize>, bool) {
    let mut d = vec![i64::MAX / 2; n];
    let mut pred = vec![usize::MAX; n];
    d[s] = 0;
    pred[s] = s;
    for _ in 1..n {
        for u in 0..n {
            for &(v, w) in &g[u] {
                if d[u] + w < d[v] {
                    d[v] = d[u] + w;
                    pred[v] = u;
                }
            }
        }
    }
    let mut has_neg_cycle = false;
    'l: for u in 0..n {
        for &(v, w) in &g[u] {
            if d[u] + w < d[v] {
                has_neg_cycle = true;
                break 'l;
            }
        }
    }
    (d, pred, has_neg_cycle)
}