#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

fn modpow(n: usize, pow: usize, p: usize) -> usize {
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

fn modinv(n: usize, p: usize) -> usize {
    modpow(n, p - 2, p)
}

fn berlekamp_massey(x: &[usize], modulo: usize) -> Vec<usize> {
    let mut ls: Vec<usize> = vec![];
    let mut cur: Vec<usize> = vec![];
    let mut lf = 0usize;
    let mut ld = 0usize;
    for i in 0..x.len() {
        let mut t = 0;
        for j in 0..cur.len() {
            t = (t + x[i-j-1] * cur[j]) % modulo;
        }
        if t == x[i] { continue; }
        if cur.is_empty() {
            cur.resize(i+1, 0);
            lf = i;
            ld = (t + modulo - x[i]) % modulo;
            continue;
        }
        let k = (t + modulo - x[i]) % modulo * modinv(ld, modulo) % modulo;
        let mut c = vec![0; i - lf - 1];
        c.push(k);
        for &j in &ls { c.push((modulo - j) * k % modulo); }
        if c.len() < cur.len() { c.resize(cur.len(), 0); }
        for j in 0..cur.len() {
            c[j] = (c[j] + cur[j]) % modulo;
        }
        if i - lf + ls.len() >= cur.len() {
            ls = cur; lf = i; ld = (t + modulo - x[i]) % modulo;
        }
        cur = c;
    }
    for i in &mut cur { *i %= modulo; }
    cur.reverse();
    cur
}