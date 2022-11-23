#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

#[allow(dead_code)]
struct LCA {
    pub n: usize,
    pub parents: Vec<usize>,
    pub children: Vec<Vec<usize>>,
    pub depths: Vec<usize>,
    pub root: usize,
    pub table: Vec<Vec<usize>>,
}

#[allow(dead_code)]
impl LCA {
    pub fn from_parents(parents: Vec<usize>) -> Self {
        let n = parents.len();
        let table_len = 64 - n.leading_zeros() as usize;
        let mut children = vec![vec![]; n];
        let root = parents.iter().enumerate().find(|&(i, &x)| i == x).unwrap().0;
        for node in 0..n {
            if node != root { children[parents[node]].push(node); }
        }
        let mut depths = vec![0usize; n];
        let mut stack = vec![(root, 0)];
        while let Some((node, cur_depth)) = stack.pop() {
            depths[node] = cur_depth;
            for &i in &children[node] { stack.push((i, cur_depth + 1)); }
        }
        let mut table = vec![vec![0usize; n]; table_len];
        table[0].copy_from_slice(&parents);
        for i in 1..table_len {
            for node in 0..n {
                table[i][node] = table[i-1][table[i-1][node]];
            }
        }
        Self { n, parents, children, depths, root, table }
    }
    pub fn from_undirected_with_root(graph: &[Vec<usize>], root: usize) -> Self {
        let n = graph.len();
        let mut parents = (0..n).collect::<Vec<_>>();
        let mut visited = vec![false; n];
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if visited[node] { continue; }
            visited[node] = true;
            for &child in &graph[node] {
                if !visited[child] { parents[child] = node; }
                stack.push(child);
            }
        }
        Self::from_parents(parents)
    }
    pub fn from_undirected(graph: &[Vec<usize>]) -> Self {
        Self::from_undirected_with_root(graph, 0)
    }
    pub fn advance(&self, cur_node: usize, step: usize) -> usize {
        let mut cur_node = cur_node;
        for i in 0..self.table.len() {
            if (step & (1 << i)) != 0 {
                cur_node = self.table[i][cur_node];
            }
        }
        cur_node
    }
    pub fn find(&self, node1: usize, node2: usize) -> usize {
        let [mut node1, mut node2] = [node1, node2];
        if self.depths[node1] < self.depths[node2] { node2 = self.advance(node2, self.depths[node2] - self.depths[node1]); }
        else if self.depths[node1] > self.depths[node2] { node1 = self.advance(node1, self.depths[node1] - self.depths[node2]); }
        self.find_inner(node1, node2, self.table.len())
    }
    fn find_inner(&self, node1: usize, node2: usize, step: usize) -> usize {
        if node1 == node2 { node1 }
        else if step == 0 { self.table[step][node1] }
        else if self.table[step-1][node1] == self.table[step-1][node2] { self.find_inner(node1, node2, step - 1) }
        else { self.find_inner(self.table[step-1][node1], self.table[step-1][node2], step - 1) }
    }
}

#[allow(dead_code)]
struct LCA2<T: Copy + Sized> {
    pub n: usize,
    pub parents: Vec<usize>,
    pub children: Vec<Vec<usize>>,
    pub depths: Vec<usize>,
    pub root: usize,
    pub table: Vec<Vec<(usize, T)>>,
    pub weights: Vec<T>,
    pub identity: T,
    pub combine: Box<dyn Fn(T, T) -> T>
}

#[allow(dead_code)]
impl<T: Copy + std::fmt::Debug> LCA2<T> {
    pub fn from_parents(parents: Vec<usize>, weights: Vec<T>, identity: T, combine: impl Fn(T, T) -> T + 'static) -> Self {
        let n = parents.len();
        let table_len = 64 - n.leading_zeros() as usize;
        let mut children = vec![vec![]; n];
        let root = parents.iter().enumerate().find(|&(i, &x)| i == x).unwrap().0;
        for node in 0..n {
            if node != root { children[parents[node]].push(node); }
        }
        let mut depths = vec![0usize; n];
        let mut stack = vec![(root, 0)];
        while let Some((node, cur_depth)) = stack.pop() {
            depths[node] = cur_depth;
            for &i in &children[node] { stack.push((i, cur_depth + 1)); }
        }
        let mut row = vec![];
        for i in 0..n {
            row.push((parents[i], weights[i]));
        }
        let mut table = vec![row; table_len];
        // let mut table = vec![vec![0usize; n]; table_len];
        // table[0].copy_from_slice(&parents);
        for i in 1..table_len {
            for node in 0..n {
                let (node1, weight1) = table[i-1][node];
                let (node2, weight2) = table[i-1][node1];
                table[i][node] = (node2, combine(weight1, weight2));
            }
        }
        Self { n, parents, children, depths, root, table, weights, identity, combine: Box::new(combine) }
    }
    pub fn from_undirected_with_root(n: usize, graph: &[(usize, usize, T)], root: usize, identity: T, combine: impl Fn(T, T) -> T + 'static) -> Self {
        // let n = graph.len();
        let mut edges = vec![vec![]; n];
        for &(x, y, w) in graph {
            edges[x].push((y, w));
            edges[y].push((x, w));
        }
        let mut parents = (0..n).collect::<Vec<_>>();
        let mut weights = vec![graph[0].2; n];
        let mut visited = vec![false; n];
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if visited[node] { continue; }
            visited[node] = true;
            for &(child, w) in &edges[node] {
                if !visited[child] { parents[child] = node; weights[child] = w; }
                stack.push(child);
            }
        }
        weights[root] = identity;
        Self::from_parents(parents, weights, identity, combine)
    }
    pub fn from_undirected(n: usize, graph: &[(usize, usize, T)], identity: T, combine: impl Fn(T, T) -> T + 'static) -> Self {
        Self::from_undirected_with_root(n, graph, 0, identity, combine)
    }
    pub fn advance(&self, cur_node: usize, step: usize) -> (usize, T) {
        let mut cur_node = cur_node;
        let mut cur_weight = self.identity;
        for i in 0..self.table.len() {
            if (step & (1 << i)) != 0 {
                let (next_node, next_weight) = self.table[i][cur_node];
                cur_node = next_node;
                cur_weight = (self.combine)(cur_weight, next_weight);
            }
        }
        (cur_node, cur_weight)
    }
    pub fn find(&self, node1: usize, node2: usize) -> (usize, T) {
        let [mut node1, mut node2] = [node1, node2];
        let mut cur_weight = self.identity;
        if self.depths[node1] < self.depths[node2] {
            let (next_node, next_weight) = self.advance(node2, self.depths[node2] - self.depths[node1]);
            node2 = next_node;
            cur_weight = (self.combine)(cur_weight, next_weight);
        }
        else if self.depths[node1] > self.depths[node2] {
            let (next_node, next_weight) = self.advance(node1, self.depths[node1] - self.depths[node2]);
            node1 = next_node;
            cur_weight = (self.combine)(cur_weight, next_weight);
        }
        let (node, weight) = self.find_inner(node1, node2, self.table.len());
        (node, (self.combine)(cur_weight, weight))
    }
    fn find_inner(&self, node1: usize, node2: usize, step: usize) -> (usize, T) {
        if node1 == node2 { (node1, self.identity) }
        else if step == 0 {
            let (node1, weight1) = self.table[step][node1];
            let (_node2, weight2) = self.table[step][node2];
            (node1, (self.combine)(weight1, weight2))
        }
        else if self.table[step-1][node1].0 == self.table[step-1][node2].0 { self.find_inner(node1, node2, step - 1) }
        else {
            let (node1, weight1) = self.table[step-1][node1];
            let (node2, weight2) = self.table[step-1][node2];
            let (lca, weight3) = self.find_inner(node1, node2, step - 1);
            (lca, (self.combine)((self.combine)(weight1, weight2), weight3))
        }
    }
}