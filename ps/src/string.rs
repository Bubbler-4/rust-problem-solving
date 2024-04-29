use std::collections::BTreeMap;

/// kmp failure function
/// fail[i] = max prefix len of s[0..<i] that == suffix of s[..=i]
pub(crate) fn kmp_failure<T: Eq>(needle: &[T]) -> Vec<usize> {
    let mut dp = vec![0usize; needle.len()];
    for i in 1..needle.len() {
        let mut cand_match_idx = dp[i-1];
        loop {
            if needle[cand_match_idx] == needle[i] { dp[i] = cand_match_idx + 1; break; }
            else if cand_match_idx == 0 { dp[i] = 0; break; }
            else { cand_match_idx = dp[cand_match_idx - 1]; }
        }
    }
    dp
}

/// kmp match array
/// match[i] = max prefix len of s that equals suffix of haystack[..=i]
pub(crate) fn kmp_match<T: Eq>(haystack: &[T], needle: &[T], failure: &[usize]) -> Vec<usize> {
    let mut mat_col = vec![0usize; haystack.len()];
    let mut mat = 0;
    for i in 0..haystack.len() {
        loop {
            if mat < needle.len() && needle[mat] == haystack[i] { mat += 1; break; }
            else if mat == 0  && needle[0] != haystack[i] { mat = 0; break; }
            else { mat = failure[mat-1]; }
        }
        mat_col[i] = mat;
    }
    mat_col
}

pub(crate) struct Trie<T> {
    subtree: BTreeMap<T, Trie<T>>
}

impl<T: std::cmp::Ord> Trie<T> {
    pub(crate) fn new() -> Self {
        Trie { subtree: BTreeMap::new() }
    }
    pub(crate) fn insert(&mut self, values: Vec<T>) {
        let mut cur_tree = &mut self.subtree;
        for v in values {
            let x = cur_tree.entry(v).or_insert(Trie { subtree: BTreeMap::new() });
            cur_tree = &mut x.subtree;
        }
    }
}

/// Suffix array (length n) and LCP array (length n-1)
pub(crate) fn suffix_array<T: Eq + Ord>(s: &[T]) -> (Vec<usize>, Vec<usize>) {
    let n = s.len();
    let mut sa = (0..n).collect::<Vec<_>>();
    sa.sort_by_key(|&i| &s[i]);
    let mut lcp = vec![0usize; n-1];
    for i in 0..n-1 {
        if s[sa[i]] == s[sa[i+1]] { lcp[i] = 1; }
    }
    let mut max_lcp = 1;
    loop {
        let mut inv = vec![0usize; n];
        let mut interval = vec![0usize; n];
        let mut int_cur = vec![];
        for i in 0..n {
            // inv[sa[i]] = i;
            if i > 0 {
                if lcp[i-1] == max_lcp { interval[i] = interval[i-1]; }
                else {
                    interval[i] = interval[i-1] + 1;
                    int_cur.push(i);
                }
            } else {
                int_cur.push(0usize);
            }
            inv[sa[i]] = interval[i];
        }
        if interval[n-1] == n-1 { break; }
        let mut next_sa = vec![0usize; n];
        for i in n .. n+max_lcp {
            let prev = i - max_lcp;
            let interval_id = inv[prev];
            next_sa[int_cur[interval_id]] = prev;
            int_cur[interval_id] += 1;
        }
        for i in 0 .. n {
            let pos = sa[i];
            if pos < max_lcp { continue; }
            let prev = pos - max_lcp;
            let interval_id = inv[prev];
            next_sa[int_cur[interval_id]] = prev;
            int_cur[interval_id] += 1;
        }
        let mut next_lcp = lcp.clone();
        for i in 0..n-1 {
            if interval[i] != interval[i+1] { continue; }
            let pos1 = next_sa[i];
            let pos2 = next_sa[i+1];
            if pos1 + max_lcp < n && pos2 + max_lcp < n {
                if inv[pos1 + max_lcp] == inv[pos2 + max_lcp] {
                    next_lcp[i] += max_lcp;
                    continue;
                }
            }
            // binary search the gap
            let mut yes = 0;
            let mut no = max_lcp;
            while yes + 1 < no {
                let mid = (yes + no) / 2;
                if pos1 + mid >= n || pos2 + mid >= n {
                    no = mid; continue;
                }
                if inv[pos1 + mid] == inv[pos2 + mid] {
                    yes = mid;
                } else {
                    no = mid;
                }
            }
            next_lcp[i] += yes;
        }
        max_lcp *= 2;
        sa = next_sa;
        lcp = next_lcp;
    }
    (sa, lcp)
}