pub(crate) fn prime_sieve(limit: usize) -> Vec<usize> {
    let mut table = vec![true; limit];
    table[0] = false;
    table[1] = false;
    for i in (2..).take_while(|x| x * x < limit) {
        if table[i] {
            for j in i..=(limit-1)/i {
                table[i*j] = false;
            }
        }
    }
    table.iter().enumerate().filter(|&(_, y)| *y).map(|(x, _)| x).collect()
}

mod factorizer; 
