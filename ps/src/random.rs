fn seed() -> usize {
    #[cfg(target="x86_64")]
    {
        use std::mem;
        use core::arch::x86_64::_rdrand64_step as rdrand_step;
        unsafe {
            loop {
                let mut el = mem::zeroed();
                if rdrand_step(&mut el) == 1 && el != 0 && el != !0 {
                    return el as usize;
                }
            }
        }
    }
    P0 ^ P1
}

const P0: usize = 0xa076_1d64_78bd_642f;
const P1: usize = 0xe703_7ed1_a0b4_28db;

fn wymum(a: usize, b: usize) -> usize {
    let r = a as u128 * b as u128;
    ((r >> 64) ^ r) as usize
}

fn wyrng(seed: &mut usize) -> usize {
    *seed = seed.wrapping_add(P0);
    wymum(*seed, *seed ^ P1)
}

pub(crate) struct Rng {
    inner: usize,
}

impl Rng {
    pub(crate) fn new() -> Self {
        Self { inner: seed() }
    }
    pub(crate) fn next(&mut self) -> usize {
        wyrng(&mut self.inner)
    }
}