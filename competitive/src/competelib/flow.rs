#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

///////////////////////////////////////////////////////////////////////
/// Maximum flow (with min cost)
/// no. of vertices n, graph [(from, to, capacity, cost)], source s, sink t -> (max flow, min cost, graph)
fn mcmf(n: usize, g: &[(usize, usize, i64, i64)], s: usize, t: usize) -> (i64, i64, Vec<Vec<(i64, i64)>>) {
    let mut cur_flow = 0;
    let mut cur_cost = 0;
    // adj. matrix
    let mut f = vec![vec![(0, 0); n]; n];
    let mut edges = vec![vec![]; n];
    for &(i, j, cap, cost) in g {
        f[i][j] = (cap, cost);
        f[j][i] = (0, -cost);
        edges[i].push(j);
        edges[j].push(i);
    }
    loop {
        // find min-cost, non-zero-cap path from s to t
        let mut d = vec![i64::MAX; n];
        let mut pred = vec![usize::MAX; n];
        d[s] = 0;
        pred[s] = s;
        let mut q = VecDeque::new();
        let mut in_q = vec![false; n];
        q.push_back(s);
        in_q[s] = true;
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            in_q[u] = false;
            for &v in &edges[u] {
                let (cap, w) = f[u][v];
                if cap == 0 { continue; }
                if d[u] + w < d[v] {
                    d[v] = d[u] + w;
                    pred[v] = u;
                    if !in_q[v] {
                        q.push_back(v);
                        in_q[v] = true;
                    }
                }
            }
        }
        if pred[t] == usize::MAX { break; }
        let mut cur = t;
        let mut pred_cur = pred[cur];
        let mut path = vec![t];
        let mut min_cap = i64::MAX;
        let mut cost_sum = 0;
        while cur != pred_cur {
            min_cap = min_cap.min(f[pred_cur][cur].0);
            cost_sum += f[pred_cur][cur].1;
            path.push(pred_cur);
            pred_cur = pred[pred_cur];
            cur = pred[cur];
        }
        cur_flow += min_cap;
        cur_cost += min_cap * cost_sum;
        let mut prev = path.pop().unwrap();
        while !path.is_empty() {
            let cur = path.pop().unwrap();
            f[prev][cur].0 -= min_cap;
            f[cur][prev].0 += min_cap;
            prev = cur;
        }
    }
    (cur_flow, cur_cost, f)
}

///////////////////////////////////////////////////////////////////////
/// Bipartite matching (Hopcroft-Karp)
/// |U|, |V|, pairings [(one from U, one from V)] -> max matching
fn hopcroft_karp(u: usize, v: usize, g: &[(usize, usize)]) -> usize {
    let pair_u = vec![usize::MAX; u];
    let pair_v = vec![u; v];
    let mut matching = 0;
    let mut adj = vec![vec![]; u];
    for &(ui, vi) in g { adj[ui].push(vi); }
    let dist_u = vec![usize::MAX; u+1];
    struct State {
        u: usize,
        pair_u: Vec<usize>,
        pair_v: Vec<usize>,
        dist_u: Vec<usize>
    }
    let mut state = State { u, pair_u, pair_v, dist_u };
    fn bfs(state: &mut State, adj: &[Vec<usize>]) -> bool {
        let mut q = VecDeque::new();
        for ui in 0..state.u {
            if state.pair_u[ui] == usize::MAX {
                state.dist_u[ui] = 0;
                q.push_back(ui);
            } else {
                state.dist_u[ui] = usize::MAX;
            }
        }
        state.dist_u[state.u] = usize::MAX;
        while !q.is_empty() {
            let ui = q.pop_front().unwrap();
            if state.dist_u[ui] < state.dist_u[state.u] {
                for &v in &adj[ui] {
                    if state.dist_u[state.pair_v[v]] == usize::MAX {
                        state.dist_u[state.pair_v[v]] = state.dist_u[ui] + 1;
                        q.push_back(state.pair_v[v]);
                    }
                }
            }
        }
        state.dist_u[state.u] != usize::MAX
    }
    fn dfs(state: &mut State, adj: &[Vec<usize>], ui: usize) -> bool {
        if ui != state.u {
            for &v in &adj[ui] {
                if state.dist_u[state.pair_v[v]] == state.dist_u[ui] + 1 {
                    if dfs(state, adj, state.pair_v[v]) {
                        state.pair_v[v] = ui;
                        state.pair_u[ui] = v;
                        return true;
                    }
                }
            }
            state.dist_u[ui] = usize::MAX;
            return false;
        }
        true
    }
    while bfs(&mut state, &adj) {
        for ui in 0..u {
            if state.pair_u[ui] == usize::MAX {
                if dfs(&mut state, &adj, ui) {
                    matching += 1;
                }
            }
        }
    }
    matching
}

///////////////////////////////////////////////////////////////////////
/// Maximum flow (Dinic)
/// n=|V|, network [(u, v, cap)], source s, sink t -> max flow
fn dinic(n: usize, g: &[(usize, usize, usize)], s: usize, t: usize) -> usize {
    let mut f = vec![vec![0; n]; n];
    let mut adj = vec![vec![]; n];
    let level = vec![0usize; n];
    let ptr = vec![0usize; n];
    for &(u, v, cap) in g {
        f[u][v] = cap;
        adj[u].push(v);
        adj[v].push(u);
    }
    struct State {
        s: usize, t: usize,
        f: Vec<Vec<usize>>, adj: Vec<Vec<usize>>,
        level: Vec<usize>, ptr: Vec<usize>,
    }
    let mut state = State { s, t, f, adj, level, ptr };
    fn bfs(state: &mut State) -> bool {
        let mut q = VecDeque::new();
        q.push_back(state.s);
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            for i in 0..state.adj[u].len() {
                let v = state.adj[u][i];
                if state.f[u][v] == 0 { continue; }
                if state.level[v] != usize::MAX { continue; }
                state.level[v] = state.level[u] + 1;
                q.push_back(v);
            }
        }
        state.level[state.t] != usize::MAX
    }
    fn dfs(state: &mut State, u: usize, pushed: usize) -> usize {
        if pushed == 0 { return 0; }
        if u == state.t { return pushed; }
        while state.ptr[u] < state.adj[u].len() {
            let v = state.adj[u][state.ptr[u]];
            if state.level[u] + 1 == state.level[v] && state.f[u][v] != 0 {
                let tr = dfs(state, v, pushed.min(state.f[u][v]));
                if tr != 0 {
                    state.f[u][v] -= tr;
                    state.f[v][u] += tr;
                    return tr;
                }
            }
            state.ptr[u] += 1;
        }
        0
    }
    let mut cur_flow = 0;
    loop {
        state.level.fill(usize::MAX);
        state.level[s] = 0;
        if !bfs(&mut state) { break; }
        state.ptr.fill(0);
        loop {
            let pushed = dfs(&mut state, s, usize::MAX);
            if pushed == 0 { break; }
            cur_flow += pushed;
        }
    }
    cur_flow
}