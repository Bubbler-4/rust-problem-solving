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