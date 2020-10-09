use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::mem::replace;

fn fib(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    let mut previous = 0u64;
    let mut current = 1u64;
    let mut tmp: u64;
    for _ in 1..n {
        tmp = current;
        current = current + previous;
        previous = tmp;
    }
    current
}

// Calculate large fibonacci numbers.
fn fib_bigint(n: u64) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    f0
}

fn main() {
    let n = std::env::args()
        .nth(1)
        .expect("Usage: fib [n]")
        .parse()
        .expect("Could not parse n");
    if n > 93 {
        println!("{}", fib_bigint(n))
    } else {
        println!("{}", fib(n))
    }
}
