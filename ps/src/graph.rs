use std::collections::BTreeSet;

///////////////////////////////////////////////////////////////////////
/// Topological sort (Kahn)
/// Edge list -> Sorted nodes, if any
pub(crate) fn toposort(n: usize, g: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut incoming = vec![BTreeSet::new(); n];
    let mut outgoing = vec![BTreeSet::new(); n];
    let mut e = 0;
    for &(i, j) in g.iter() {
        e += incoming[j].insert(i) as usize;
        outgoing[i].insert(j);
    }
    let mut l = vec![];
    let mut s = vec![];
    for (i, incoming_it) in incoming.iter().enumerate() {
        if incoming_it.is_empty() {
            s.push(i);
        }
    }
    while !s.is_empty() {
        let n = s.pop().unwrap();
        l.push(n);
        for &i in outgoing[n].iter() {
            incoming[i].remove(&n);
            e -= 1;
            if incoming[i].is_empty() {
                s.push(i);
            }
        }
    }
    if e > 0 { None } else { Some(l) }
}

///////////////////////////////////////////////////////////////////////
/// Disjoint set

pub(crate) struct DisjointSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    pub(crate) fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let rank = vec![0; n];
        Self { parent, rank }
    }
    pub(crate) fn find(&mut self, n: usize) -> usize {
        let mut n = n;
        while self.parent[n] != n {
            self.parent[n] = self.parent[self.parent[n]];
            n = self.parent[n];
        }
        n
    }
    /// returns true if union operation was done (set of n1 != set of n2)
    pub(crate) fn union(&mut self, n1: usize, n2: usize) -> bool {
        let mut n1 = self.find(n1);
        let mut n2 = self.find(n2);
        if n1 == n2 { return false; }
        if self.rank[n1] < self.rank[n2] {
            std::mem::swap(&mut n1, &mut n2);
        }
        self.parent[n2] = n1;
        if self.rank[n1] == self.rank[n2] {
            self.rank[n1] += 1;
        }
        true
    }
}

///////////////////////////////////////////////////////////////////////
/// Minimum spanning tree (Kruskal)

pub(crate) fn spanning_tree(n: usize, g: &mut [(usize, usize, i64)]) -> (Vec<(usize, usize, i64)>, i64) {
    g.sort_unstable_by_key(|&x| x.2);
    let mut dj = DisjointSet::new(n);
    let mut edges = vec![];
    let mut total = 0;
    for &(i, j, w) in g.iter() {
        if dj.union(i, j) {
            edges.push((i, j, w));
            total += w;
        }
    }
    (edges, total)
}