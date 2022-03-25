#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

fn gcd(a: usize, b: usize) -> usize {
    let mut x = (a, b);
    while x.1 > 0 {
        x = (x.1, x.0 % x.1);
    }
    x.0
}

// (x, y, gcd, qa, qb): ax + by = gcd, qa = a/gcd, qb = b/gcd
// if gcd == 1, x = a^-1 mod b, but returned x may be negative
fn egcd(a: i64, b: i64) -> (i64, i64, i64, i64, i64) {
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

fn egcd_modinv(a: usize, m: usize) -> usize {
    let (x, _y, _gcd, _qa, _qb) = egcd(a as i64, m as i64);
    x.rem_euclid(m as i64) as usize
}

// x = a1 mod m1, a2 mod m2 -> (u, v) where x = u mod v
fn crt(a1: i64, m1: i64, a2: i64, m2: i64) -> Option<(i64, i64)> {
    // has solution if g divides a1 - a2; if um + vn = g, x = (avn + bum)/g mod (mn/g).
    let (x, y, gcd, _qa, _qb) = egcd(m1, m2);
    if (a1 - a2) % gcd == 0 {
        let lcm = (m1 / gcd * m2).abs();
        Some((((a1 * m2 * y + a2 * m1 * x) / gcd).rem_euclid(lcm), lcm))
    } else { None }
}