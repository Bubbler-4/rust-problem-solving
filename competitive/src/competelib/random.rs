#![allow(unused_macros)]
#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;
use std::cmp::{Reverse, Ordering::{self, *}};

fn rand() -> usize {
    use std::mem;
    use core::arch::x86_64::_rdrand64_step as rdrand_step;
    unsafe {
        loop {
            let mut el = mem::zeroed();
            if rdrand_step(&mut el) == 1 {
                if el != 0 && el != !0 {
                    return el as usize;
                }
            }
        }
    }
}