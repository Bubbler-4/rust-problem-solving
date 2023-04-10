use std::cmp::{Ordering, Ordering::*};

#[derive(Clone, Copy)]
pub(crate) struct Frac(pub(crate) i64, pub(crate) i64);
impl PartialEq for Frac {
    fn eq(&self, other: &Self) -> bool {
        self.0 * other.1 == self.1 * other.0
    }
}
impl Eq for Frac {}
impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (self.0 * other.1).partial_cmp(&(self.1 * other.0))
    }
}
impl Ord for Frac {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.0 * other.1).cmp(&(self.1 * other.0))
    }
}

/// (x, y) pts -> (upper hull, lower hull)
pub(crate) fn convex_hull<const B: bool>(pts: &[(i64, i64)]) -> (Vec<(i64, i64)>, Vec<(i64, i64)>) {
    let mut v = pts.to_vec();
    v.sort_unstable(); // min x; min y
    let mut lower = vec![v[0]];
    let mut upper = vec![v[0]];
    for &(x, y) in &v[1..] {
        'lower: {
            while lower.len() >= 2 {
                let (x2, y2) = lower[lower.len() - 1];
                let (x1, y1) = lower[lower.len() - 2];
                let prod = (x2 - x1) * (y - y2) - (x - x2) * (y2 - y1);
                match prod.cmp(&0) {
                    Less => { lower.pop(); }
                    Equal => { if !B { lower.pop(); } lower.push((x, y)); break 'lower; }
                    Greater => { lower.push((x, y)); break 'lower; }
                }
            }
            lower.push((x, y));
        }
        'upper: {
            while upper.len() >= 2 {
                let (x2, y2) = upper[upper.len() - 1];
                let (x1, y1) = upper[upper.len() - 2];
                let prod = (x2 - x1) * (y - y2) - (x - x2) * (y2 - y1);
                match prod.cmp(&0) {
                    Less => { upper.push((x, y)); break 'upper; }
                    Equal => { if !B { upper.pop(); } upper.push((x, y)); break 'upper; }
                    Greater => { upper.pop(); }
                }
            }
            upper.push((x, y));
        }
    }
    (upper, lower)
}