#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

///////////////////////////////////////////////////////////////////////
/// Faster sequencing; https://hal.inria.fr/hal-02917827v2/document

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

// naive polymul
fn poly_mul(p: &[usize], q: &[usize], modulo: usize) -> Vec<usize> {
    let mut ans = vec![0; p.len() + q.len() - 1];
    for i in 0..p.len() {
        for j in 0..q.len() {
            ans[i+j] = (ans[i+j] + p[i] * q[j]) % modulo;
        }
    }
    ans
}

fn one_coeff(p: &mut [usize], q: &mut [usize], mut n: usize, modulo: usize) -> usize {
    while n >= 1 {
        let d = p.len();
        let q_minus: Vec<usize> = q.iter().enumerate().map(|(i, &q_i)| if i % 2 == 0 { q_i } else { (modulo - q_i) % modulo }).collect();
        let u = poly_mul(p, &q_minus, modulo);
        let n_bit = (n % 2) as usize;
        for i in 0..d {
            p[i] = u[2 * i + n_bit];
        }
        let a = poly_mul(q, &q_minus, modulo);
        for i in 0..=d {
            q[i] = a[2 * i];
        }
        n /= 2;
    }
    p[0] * modinv(q[0], modulo) % modulo
}

fn nth_term(recurrence: &[usize], initial: &[usize], n: usize, modulo: usize) -> usize {
    let d = recurrence.len();
    let mut q: Vec<usize> = Vec::with_capacity(d+1);
    q.push(1);
    for &ci in recurrence.iter().rev() { q.push((modulo - ci) % modulo); }
    let mut p = poly_mul(initial, &q, modulo);
    p.resize(d, 0);
    one_coeff(&mut p, &mut q, n, modulo)
}