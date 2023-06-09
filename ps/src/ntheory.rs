pub(crate) fn modpow(n: usize, pow: usize, p: usize) -> usize {
    let mut npow2 = n;
    let mut cur = 1;
    let mut pow = pow;
    while pow > 0 {
        if pow % 2 == 1 {
            cur = cur * npow2 % p;
        }
        npow2 = npow2 * npow2 % p;
        pow /= 2;
    }
    cur
}

pub(crate) fn modpow_u128(n: u128, pow: u128, p: u128) -> u128 {
    let mut npow2 = n;
    let mut cur = 1;
    let mut pow = pow;
    while pow > 0 {
        if pow % 2 == 1 {
            cur = cur * npow2 % p;
        }
        npow2 = npow2 * npow2 % p;
        pow /= 2;
    }
    cur
}

pub(crate) fn modinv(n: usize, p: usize) -> usize {
    modpow(n, p - 2, p)
}

pub(crate) fn isqrt(n: usize) -> usize {
    let mut x = 4294967295;
    while x * x > n {
        x = (x + n / x) / 2;
    }
    x
}

pub(crate) fn isqrt_u128(n: u128) -> u128 {
    let mut x = usize::MAX as u128;
    while x * x > n {
        x = (x + n / x) / 2;
    }
    x
}

pub(crate) fn isqrt_ceil(n: usize) -> usize {
    let mut x = 4294967295;
    while x * x > n {
        x = (x + n / x) / 2;
    }
    if x * x == n { x } else { x + 1 }
}

pub(crate) fn gcd(a: usize, b: usize) -> usize {
    let mut x = (a, b);
    while x.1 > 0 {
        x = (x.1, x.0 % x.1);
    }
    x.0
}

pub(crate) fn primroot(p: usize) -> usize {
    let mut pfactors = vec![];
    let mut q = p - 1;
    let mut curp = 2;
    while curp * curp <= q {
        if q % curp == 0 {
            pfactors.push(curp);
            while q % curp == 0 { q /= curp; }
        }
        if curp == 2 { curp = 3; }
        else { curp += 2; }
    }
    if q != 1 { pfactors.push(q); }
    for pr in 2..p {
        if pfactors.iter().all(|&pf| modpow(pr, (p - 1) / pf, p) != 1) {
            return pr;
        }
    }
    0
}

pub(crate) fn is_primroot(p: usize, proot: usize) -> bool {
    let mut pfactors = vec![];
    let mut q = p - 1;
    let mut curp = 2;
    while curp * curp <= q {
        if q % curp == 0 {
            pfactors.push(curp);
            while q % curp == 0 { q /= curp; }
        }
        if curp == 2 { curp = 3; }
        else { curp += 2; }
    }
    if q != 1 { pfactors.push(q); }
    if pfactors.iter().all(|&pf| modpow(proot, (p - 1) / pf, p) != 1) {
        return true;
    }
    false
}

/// Solves `base ^ x == res (mod p)`. Assumes `base` is a primitive root of `p` and `res` is not 0.
pub(crate) fn discrete_log(base: usize, res: usize, p: usize) -> usize {
    use std::collections::*;
    let sqrtp = isqrt_ceil(p);
    let mut small_inv = HashMap::new();
    // small * large = res; large = res * small_inv
    let mut cur_small_inv = res;
    let mut cur_large = 1;
    let small_inv_step = modinv(base, p);
    let large_step = modpow(base, sqrtp, p);
    for i in 0..=sqrtp {
        small_inv.insert(cur_small_inv, i);
        cur_small_inv = cur_small_inv * small_inv_step % p;
    }
    for i in 0..=sqrtp {
        if let Some(&small_idx) = small_inv.get(&cur_large) {
            return small_idx + sqrtp * i;
        }
        cur_large = cur_large * large_step % p;
    }
    unreachable!();
}

// (x, y, gcd, qa, qb): ax + by = gcd, qa = a/gcd, qb = b/gcd
// if gcd == 1, x = a^-1 mod b, but returned x may be negative
pub(crate) fn egcd(a: i64, b: i64) -> (i64, i64, i64, i64, i64) {
    let mut r = (a, b);
    let mut s = (1, 0);
    let mut t = (0, 1);
    while r.1 != 0 {
        let q = r.0 / r.1;
        r = (r.1, r.0 - q * r.1);
        s = (s.1, s.0 - q * s.1);
        t = (t.1, t.0 - q * t.1);
    }
    if r.0 < 0 { (-s.0, -t.0, -r.0, -t.1, -s.1) }
    else { (s.0, t.0, r.0, t.1, s.1) }
}

pub(crate) fn egcd_modinv(a: usize, m: usize) -> usize {
    let (x, _y, _gcd, _qa, _qb) = egcd(a as i64, m as i64);
    x.rem_euclid(m as i64) as usize
}