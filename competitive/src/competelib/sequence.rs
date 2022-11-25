#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

fn matmul(mat1: &[usize; 4], mat2: &[usize; 4], p: usize) -> [usize; 4] {
    [
        (mat1[0] * mat2[0] + mat1[1] * mat2[2]) % p,
        (mat1[0] * mat2[1] + mat1[1] * mat2[3]) % p,
        (mat1[2] * mat2[0] + mat1[3] * mat2[2]) % p,
        (mat1[2] * mat2[1] + mat1[3] * mat2[3]) % p
    ]
}

fn matpow(mat: [usize; 4], pow: usize, p: usize) -> [usize; 4] {
    let mut matpow2 = mat;
    let mut curmat = [1usize, 0, 0, 1];
    let mut pow = pow;
    while pow > 0 {
        if pow % 2 == 1 {
            curmat = matmul(&curmat, &matpow2, p);
        }
        matpow2 = matmul(&matpow2, &matpow2, p);
        pow /= 2;
    }
    curmat
}

fn matmuln(mat1: &[Vec<usize>], mat2: &[Vec<usize>], p: usize) -> Vec<Vec<usize>> {
    let l = mat1.len();
    let mut ans = mat1.to_vec();
    for r in 0..l {
        for c in 0..l {
            ans[r][c] = (0..l).map(|m| mat1[r][m] * mat2[m][c] % p).sum::<usize>() % p;
        }
    }
    ans
}

fn matpown(mat: &[Vec<usize>], pow: usize, p: usize) -> Vec<Vec<usize>> {
    let mut matpow2 = mat.to_vec();
    let mut curmat = matpow2.clone();
    for (r, row) in curmat.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            *cell = (r == c) as usize;
        }
    }
    let mut pow = pow;
    while pow > 0 {
        if pow % 2 == 1 {
            curmat = matmuln(&curmat, &matpow2, p);
        }
        matpow2 = matmuln(&matpow2, &matpow2, p);
        pow /= 2;
    }
    curmat
}

fn mataddn(mat1: &[Vec<usize>], mat2: &[Vec<usize>], p: usize) -> Vec<Vec<usize>> {
    let mut ans = mat1.to_vec();
    for r in 0..mat1.len() {
        for c in 0..mat1[0].len() {
            ans[r][c] = (ans[r][c] + mat2[r][c]) % p;
        }
    }
    ans
}

/* 
summat = 0; powmat = I
for b in pow.bits (in descending order)
 summat = summat + summat * powmat
 powmat *= powmat
 if b
  powmat *= A
  summat = summat * A + I */
fn matpown_sum(mat: &[Vec<usize>], pow: usize, p: usize) -> Vec<Vec<usize>> {
    let l = mat.len();
    let mut summat = vec![vec![0; l]; l];
    let mut powmat = summat.clone();
    for (i, row) in powmat.iter_mut().enumerate() { row[i] = 1; }
    let id = powmat.clone();
    for bit in (0..64).rev() {
        let sumpow = matmuln(&summat, &powmat, p);
        summat = mataddn(&summat, &sumpow, p);
        powmat = matmuln(&powmat, &powmat, p);
        if (pow >> bit) % 2 > 0 {
            powmat = matmuln(&powmat, mat, p);
            summat = matmuln(&summat, mat, p);
            summat = mataddn(&summat, &id, p);
        }
    }
    summat
}
