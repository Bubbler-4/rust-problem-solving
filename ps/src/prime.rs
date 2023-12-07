pub(crate) fn prime_sieve(limit: usize) -> Vec<usize> {
    let mut table = vec![true; limit];
    table[0] = false;
    table[1] = false;
    let mut ans = Vec::with_capacity(limit);
    for i in 2..limit {
        if table[i] {
            ans.push(i);
            for j in (i * i .. limit).step_by(i) {
                table[j] = false;
            }
        }
    }
    ans
}

pub(crate) fn prime_sieve2(limit: usize) -> Vec<usize> {
    let mut table = vec![true; limit / 2];
    table[0] = false;
    let mut ans = Vec::with_capacity(limit);
    ans.push(2);
    for i in 1 .. limit/2 {
        if table[i] {
            ans.push(i * 2 + 1);
            for j in (i * (i + 1) * 2 .. limit/2).step_by(i*2+1) {
                table[j] = false;
            }
        }
    }
    ans
}

pub(crate) mod factorizer; 
