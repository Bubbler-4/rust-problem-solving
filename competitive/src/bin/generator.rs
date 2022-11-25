use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
 
fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let mut rand = Rand::new(&args);
    // chocolate-7
    // [binaryname, nmin, nmax, ...]
    let nmin = args[1].parse::<usize>().unwrap();
    let nmax = args[2].parse::<usize>().unwrap();
    let n = rand.roll(nmax - nmin + 1) + nmin;
    print!("{}\r\n", n);
}
 
// WyRand impl from nanorand crate
#[allow(dead_code)]
struct Rand {
    seed: u64,
}
 
#[allow(dead_code)]
impl Rand {
    fn new<T: Hash>(seed: &T) -> Self {
        let mut hasher = DefaultHasher::new();
        seed.hash(&mut hasher);
        Rand { seed: hasher.finish() }
    }
 
    fn rand(&mut self) -> usize {
        self.seed = self.seed.wrapping_add(0xa0761d6478bd642f);
        let t: u128 = (self.seed as u128).wrapping_mul((self.seed ^ 0xe7037ed1a0b428db) as u128);
        (t.wrapping_shr(64) ^ t) as usize
    }
 
    fn roll(&mut self, limit: usize) -> usize {
        if limit.is_power_of_two() {
            self.rand() % limit
        } else {
            let bound = usize::MAX / limit * limit;
            loop {
                let val = self.rand();
                if val < bound { break val % limit; }
            }
        }
    }
 
    fn rollf(&mut self) -> f64 {
        let left = self.roll(1<<26) << 27;
        let right = self.roll(1<<27);
        (left + right) as f64 / (1usize << 53) as f64
    }
 
    fn deal(&mut self, domain: usize, count: usize) -> Vec<usize> {
        let mut buf: Vec<usize> = (0..domain).collect();
        for i in 0..count {
            let next_id = self.roll(domain - i) + i;
            buf.swap(i, next_id);
        }
        buf.truncate(count);
        buf
    }
}
