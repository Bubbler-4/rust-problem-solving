/// The element must be a `Monoid`. The example for a sum segment tree:
/// 
/// ```no_run
/// impl Monoid for i64 {
///     fn op(self, rhs: Self) -> Self {
///         self + rhs
///     }
///     fn identity() -> Self { 0 }
/// }
/// ```
#[derive(Debug)]
pub(crate) struct SegTree<T>(Vec<T>);

/// Requirements: op is associative and has an identity element
pub(crate) trait Monoid: Clone + Copy {
    fn op(self, rhs: Self) -> Self;
    fn identity() -> Self;
}

impl<T: Monoid> SegTree<T> {
    pub(crate) fn new(data: Vec<T>) -> Self {
        let base_len = data.len().next_power_of_two();
        let len = base_len * 2;
        let mut inner_data = vec![T::identity(); len];
        inner_data[base_len..base_len + data.len()].copy_from_slice(&data);
        for i in (1..base_len).rev() {
            inner_data[i] = inner_data[i*2].op(inner_data[i*2+1]);
        }
        SegTree(inner_data)
    }

    pub(crate) fn update(&mut self, idx: usize, data: T) {
        let base_len = self.0.len() / 2;
        let mut cur_idx = base_len + idx;
        self.0[cur_idx] = data;
        while cur_idx != 1 {
            let parent = cur_idx / 2;
            let left = cur_idx & !1;
            self.0[parent] = self.0[left].op(self.0[left + 1]);
            cur_idx = parent;
        }
    }

    pub(crate) fn query(&self, left: usize, right: usize) -> T {
        let base_len = self.0.len() / 2;
        let mut left = left + base_len;
        let mut right = right + base_len;
        let mut l_agg = T::identity();
        let mut r_agg = T::identity();
        while left <= right {
            if left % 2 == 1 {
                l_agg = l_agg.op(self.0[left]);
                left += 1;
            }
            if right % 2 == 0 {
                r_agg = self.0[right].op(r_agg);
                right -= 1;
            }
            left /= 2;
            right /= 2;
        }
        l_agg.op(r_agg)
    }
}

/// Requirements:
/// * op is associative and has an identity element
/// * compose: `apply(a, compose(f, g)) == apply(apply(a, g), f)`
/// * apply: `apply(op(a, b), f) == op(apply(a, f), apply(b, f))`
pub(crate) trait Lazyseg: Clone + Copy + std::fmt::Debug {
    type F: Clone + Copy + std::fmt::Debug;
    fn op(self, rhs: Self) -> Self;
    fn identity() -> Self;
    fn compose(f: Self::F, g: Self::F) -> Self::F;
    fn apply(self, f: Self::F) -> Self;
}

/// range sum with range add example:
/// `f(data, cnt) = (data + f * cnt, cnt)`
/// 
/// ```no_run
/// impl Lazyseg for (i64, i64) {
///     type F = i64; // represents "add X" function
///     fn op(self, rhs: Self) -> Self { (self.0 + rhs.0, self.1 + rhs.1) }
///     fn identity() -> Self { (0, 1) }
///     fn compose(f: Self::F, g: Self::F) -> Self::F { f + g }
///     fn apply(self, f: Self::F) -> Self { (self.0 + f * self.1, self.1) }
/// }
/// ```
pub(crate) struct LazySegTree<T> where T: Lazyseg {
    data: Vec<T>,
    app: Vec<Option<T::F>>,
}

impl<T: Lazyseg> LazySegTree<T> {
    pub(crate) fn new(init_data: &[T]) -> Self {
        let size = init_data.len().next_power_of_two();
        let mut data = vec![T::identity(); size];
        data.extend_from_slice(init_data);
        data.resize(size * 2, T::identity());
        let app = vec![None; size];

        let mut seg = Self { data, app };
        for p in (0..size).rev() {
            seg.pull(p);
        }
        seg
    }

    fn apply(&mut self, p: usize, f: T::F) {
        self.data[p] = self.data[p].apply(f);
        if let Some(lazy) = self.app.get_mut(p) {
            let h = match lazy {
                Some(g) => T::compose(f, *g),
                None => f
            };
            *lazy = Some(h);
        }
    }

    fn push(&mut self, p: usize) {
        if let Some(f) = self.app[p].take() {
            self.apply(p << 1, f);
            self.apply(p << 1 | 1, f);
        }
    }

    fn pull(&mut self, p: usize) {
        self.data[p] = self.data[p * 2].op(self.data[p * 2 + 1]);
    }

    fn push_to(&mut self, p: usize) {
        let one_plus_floor_log_p = (p + 1).next_power_of_two().trailing_zeros();
        for i in (1..one_plus_floor_log_p).rev() {
            self.push(p >> i);
        }
    }

    fn pull_from(&mut self, mut p: usize) {
        while p > 1 {
            p >>= 1;
            self.pull(p);
        }
    }

    pub(crate) fn update(&mut self, mut l: usize, mut r: usize, f: T::F) {
        l += self.app.len();
        r += self.app.len();
        if l < r {
            self.push_to(l);
        }
        self.push_to(r);
        let (mut l0, mut r0) = (1, 1);
        while l <= r {
            if l & 1 == 1 {
                self.apply(l, f);
                l0 = l0.max(l);
                l += 1;
            }
            if r & 1 == 0 {
                self.apply(r, f);
                r0 = r0.max(r);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        self.pull_from(l0);
        self.pull_from(r0);
    }

    pub(crate) fn query(&mut self, mut l: usize, mut r: usize) -> T {
        l += self.app.len();
        r += self.app.len();
        if l < r {
            self.push_to(l);
        }
        self.push_to(r);
        let (mut l_agg, mut r_agg) = (T::identity(), T::identity());
        while l <= r {
            if l & 1 == 1 {
                l_agg = l_agg.op(self.data[l]);
                l += 1;
            }
            if r & 1 == 0 {
                r_agg = self.data[r].op(r_agg);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        l_agg.op(r_agg)
    }
}