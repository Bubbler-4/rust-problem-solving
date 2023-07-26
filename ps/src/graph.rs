use std::collections::*;
use std::cmp::Reverse;

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

///////////////////////////////////////////////////////////////////////
/// SPFA (single source all destination, works with negative edges, but cannot detect neg.cycles)
/// weighted adjacency list -> (dist, prev)
pub(crate) fn spfa(n: usize, g: &[Vec<(usize, i64)>], s: usize) -> (Vec<i64>, Vec<usize>) {
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
pub(crate) fn dijkstra(n: usize, g: &[Vec<(usize, usize)>], s: usize) -> (Vec<usize>, Vec<usize>) {
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
pub(crate) fn bellman_ford(n: usize, g: &[Vec<(usize, i64)>], s: usize) -> (Vec<i64>, Vec<usize>, bool) {
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