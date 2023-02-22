fn mul_mod(x: usize, y: usize, m: usize) -> usize {
    //(x * y + m - m * (1.0 / m as f64 * x as f64 * y as f64) as usize) % m
    (x as u128 * y as u128 % m as u128) as usize
}
fn pow_mod(n: usize, p: usize, m: usize) -> usize {
    let mut ans = 1;
    let mut p = p;
    let mut n = n;
    while p > 0 {
        if p % 2 != 0 { ans = mul_mod(ans, n, m); }
        n = mul_mod(n, n, m);
        p /= 2;
    }
    ans
}
pub(crate) fn is_prime(n: usize) -> bool {
    if n < 2 || n % 6 % 4 != 1 { return (n | 1) == 3; }
    let s = (n-1).trailing_zeros();
    let d = n >> s;
    for &a in &[2, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        let mut p = pow_mod(a % n, d, n);
        let mut i = s;
        while p != 1 && p != n - 1 && a % n != 0 && i != 0 {
            p = mul_mod(p, p, n);
            i -= 1;
        }
        if p != n - 1 && i != s { return false; }
    }
    true
}
fn gcd(a: usize, b: usize) -> usize {
    let mut x = (a, b);
    while x.1 > 0 {
        x = (x.1, x.0 % x.1);
    }
    x.0
}
fn pollard(n: usize) -> usize {
    let f = |x| mul_mod(x, x, n) + 1;
    let mut x = 0;
    let mut y = 0;
    let mut t = 30usize;
    let mut prd = 2;
    let mut i = 1;
    while t % 40 != 0 || gcd(prd, n) == 1 {
        if x == y {
            i += 1;
            x = i;
            y = f(x);
        }
        let q = mul_mod(prd, x.max(y) - x.min(y), n);
        if q != 0 { prd = q; }
        x = f(x);
        y = f(f(y));
        t += 1;
    }
    gcd(prd, n)
}
pub(crate) fn factorize(n: usize) -> Vec<usize> {
    if n == 1 { return vec![]; }
    if is_prime(n) { return vec![n]; }
    let x = pollard(n);
    let mut v1 = factorize(x);
    v1.append(&mut factorize(n / x));
    v1
}