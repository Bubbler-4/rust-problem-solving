#![allow(unused_imports)]
#![allow(unused)]
use rand::{thread_rng, Rng};
fn main() {
    println!("{}", S); return;
    println!("500 500");
    for _ in 0..500 {
        for _ in 0..500 {
            print!("1 ");
        }
        println!();
    }
    // let mut rng = thread_rng();

    // let n = 100000;
    // let m = 100000;
    // println!("{}", n);
    // for i in 1..n { println!("{} {}", i, i+1); }
    // println!("{}", m);
    // for _ in 0..m { println!("{} {}", rng.gen_range(1..=n), rng.gen_range(1..=n)); }
}

const S: &str = include_str!("../../input.txt");