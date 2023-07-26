use std::cmp::{Ordering, Ordering::*};

pub(crate) struct Lca {
    pub(crate) n: usize,
    pub(crate) parents: Vec<usize>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depths: Vec<usize>,
    pub(crate) root: usize,
    pub(crate) table: Vec<Vec<usize>>,
}

impl Lca {
    pub(crate) fn from_parents(parents: Vec<usize>) -> Self {
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
    pub(crate) fn from_undirected_with_root(graph: &[Vec<usize>], root: usize) -> Self {
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
    pub(crate) fn from_undirected(graph: &[Vec<usize>]) -> Self {
        Self::from_undirected_with_root(graph, 0)
    }
    pub(crate) fn advance(&self, cur_node: usize, step: usize) -> usize {
        let mut cur_node = cur_node;
        for i in 0..self.table.len() {
            if (step & (1 << i)) != 0 {
                cur_node = self.table[i][cur_node];
            }
        }
        cur_node
    }
    pub(crate) fn find(&self, node1: usize, node2: usize) -> usize {
        let [mut node1, mut node2] = [node1, node2];
        match self.depths[node1].cmp(&self.depths[node2]) {
            Less => { node2 = self.advance(node2, self.depths[node2] - self.depths[node1]); }
            Equal => {}
            Greater => { node1 = self.advance(node1, self.depths[node1] - self.depths[node2]); }
        }
        self.find_inner(node1, node2, self.table.len())
    }
    fn find_inner(&self, node1: usize, node2: usize, step: usize) -> usize {
        if node1 == node2 { node1 }
        else if step == 0 { self.table[step][node1] }
        else if self.table[step-1][node1] == self.table[step-1][node2] { self.find_inner(node1, node2, step - 1) }
        else { self.find_inner(self.table[step-1][node1], self.table[step-1][node2], step - 1) }
    }
}