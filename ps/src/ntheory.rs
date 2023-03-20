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

pub(crate) fn isqrt_ceil(n: usize) -> usize {
    let mut x = 4294967295;
    while x * x > n {
        x = (x + n / x) / 2;
    }
    if x * x == n { x } else { x + 1 }
}