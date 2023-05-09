pub(crate) fn matmul(mat1: &[usize; 4], mat2: &[usize; 4], p: usize) -> [usize; 4] {
    [
        (mat1[0] * mat2[0] + mat1[1] * mat2[2]) % p,
        (mat1[0] * mat2[1] + mat1[1] * mat2[3]) % p,
        (mat1[2] * mat2[0] + mat1[3] * mat2[2]) % p,
        (mat1[2] * mat2[1] + mat1[3] * mat2[3]) % p
    ]
}

pub(crate) fn matpow(mat: [usize; 4], pow: usize, p: usize) -> [usize; 4] {
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

pub(crate) fn matmuln(mat1: &[Vec<usize>], mat2: &[Vec<usize>], p: usize) -> Vec<Vec<usize>> {
    let l = mat1.len();
    let mut ans = mat1.to_vec();
    for r in 0..l {
        for c in 0..l {
            ans[r][c] = (0..l).map(|m| mat1[r][m] * mat2[m][c] % p).sum::<usize>() % p;
        }
    }
    ans
}

pub(crate) fn matpown(mat: &[Vec<usize>], pow: usize, p: usize) -> Vec<Vec<usize>> {
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
    p[0] * super::ntheory::modinv(q[0], modulo) % modulo
}

pub(crate) fn nth_term(recurrence: &[usize], initial: &[usize], n: usize, modulo: usize) -> usize {
    let d = recurrence.len();
    let mut q: Vec<usize> = Vec::with_capacity(d+1);
    q.push(1);
    for &ci in recurrence.iter().rev() { q.push((modulo - ci) % modulo); }
    let mut p = poly_mul(initial, &q, modulo);
    p.resize(d, 0);
    one_coeff(&mut p, &mut q, n, modulo)
}

pub(crate) fn berlekamp_massey(x: &[usize], modulo: usize) -> Vec<usize> {
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
        let k = (t + modulo - x[i]) % modulo * super::ntheory::modinv(ld, modulo) % modulo;
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