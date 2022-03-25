#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

///////////////////////////////////////////////////////////////////////
/// Strongly connected component (SCC)

fn scc(v: usize, edges: &[Vec<usize>]) -> Vec<Vec<usize>> {
    struct State {
        edges: Vec<Vec<usize>>,
        s: Vec<usize>,
        index: usize,
        vindex: Vec<Option<usize>>,
        vlowlink: Vec<usize>,
        vonstack: Vec<bool>,
        sccs: Vec<Vec<usize>>
    }
    let mut state = State {
        edges: edges.to_vec(),
        s: vec![],
        index: 0,
        vindex: vec![None; v],
        vlowlink: vec![usize::MAX; v],
        vonstack: vec![false; v],
        sccs: vec![]
    };
    fn strongconnect(v: usize, state: &mut State) {
        state.vindex[v] = Some(state.index);
        state.vlowlink[v] = state.index;
        state.index += 1;
        state.s.push(v);
        state.vonstack[v] = true;
        let edge_v = state.edges[v].clone();
        for &w in &edge_v {
            if state.vindex[w].is_none() {
                strongconnect(w, state);
                state.vlowlink[v] = state.vlowlink[v].min(state.vlowlink[w]);
            } else if state.vonstack[w] {
                state.vlowlink[v] = state.vlowlink[v].min(state.vindex[w].unwrap());
            }
        }
        if Some(state.vlowlink[v]) == state.vindex[v] {
            let mut new_scc = vec![];
            loop {
                let w = state.s.pop().unwrap();
                state.vonstack[w] = false;
                new_scc.push(w);
                if w == v { break; }
            }
            state.sccs.push(new_scc);
        }
    }
    for i in 0..v {
        if state.vindex[i].is_none() {
            strongconnect(i, &mut state);
        }
    }
    state.sccs
}

///////////////////////////////////////////////////////////////////////
/// 2SAT

// n = variables, clauses use 0..n for original vars and n..2n for negations
fn twosat(variables: usize, clauses: &[(usize, usize)]) -> Option<Vec<bool>> {
    // construct implication graph
    let v = variables * 2;
    let mut edges = vec![vec![]; v];
    let negate = |x| if x < variables { x + variables } else { x - variables };
    for &(a, b) in clauses {
        // a or b <=> not a -> b <=> not b -> a
        edges[negate(a)].push(b);
        edges[negate(b)].push(a);
    }
    // run SCC, assigning true to found SCC nodes (and false to negations), aborting if contradiction
    struct State {
        edges: Vec<Vec<usize>>,
        s: Vec<usize>,
        index: usize,
        vindex: Vec<Option<usize>>,
        vlowlink: Vec<usize>,
        vonstack: Vec<bool>,
        //sccs: Vec<Vec<usize>>,
        assignment: Vec<Option<bool>>,
        satisfiable: bool
    }
    let mut state = State {
        edges,
        s: vec![],
        index: 0,
        vindex: vec![None; v],
        vlowlink: vec![usize::MAX; v],
        vonstack: vec![false; v],
        //sccs: vec![]
        assignment: vec![None; v],
        satisfiable: true
    };
    fn strongconnect(v: usize, state: &mut State) {
        state.vindex[v] = Some(state.index);
        state.vlowlink[v] = state.index;
        state.index += 1;
        state.s.push(v);
        state.vonstack[v] = true;
        let edge_v = state.edges[v].clone();
        for &w in &edge_v {
            if state.vindex[w].is_none() {
                strongconnect(w, state);
                if !state.satisfiable { return; }
                state.vlowlink[v] = state.vlowlink[v].min(state.vlowlink[w]);
            } else if state.vonstack[w] {
                state.vlowlink[v] = state.vlowlink[v].min(state.vindex[w].unwrap());
            }
        }
        if Some(state.vlowlink[v]) == state.vindex[v] {
            //let mut new_scc = vec![];
            let mut skip_this_scc = false;
            let mut is_init = true;
            loop {
                let w = state.s.pop().unwrap();
                state.vonstack[w] = false;
                //new_scc.push(w);
                if is_init && state.assignment[w].is_some() {
                    skip_this_scc = true;
                }
                is_init = false;
                if !skip_this_scc {
                    if state.assignment[w] == Some(false) {
                        state.satisfiable = false;
                        return;
                    }
                    state.assignment[w] = Some(true);
                    let variables = state.edges.len() / 2;
                    state.assignment[if w < variables { w + variables } else { w - variables }] = Some(false);
                }
                if w == v { break; }
            }
            //state.sccs.push(new_scc);
        }
    }
    for i in 0..v {
        if state.vindex[i].is_none() {
            strongconnect(i, &mut state);
        }
    }
    if state.satisfiable { Some(state.assignment.iter().map(|x| x.unwrap()).collect()) } else { None }
}