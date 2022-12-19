/* 14. Write a function that computes the list of the first 100 Fibonacci numbers.
    The first two Fibonacci numbers are 1 and 1.
    The n+1-st Fibonacci number can be computed by adding the n-th and the n-1-th Fibonacci number.
    The first few are therefore 1, 1, 1+1=2, 1+2=3, 2+3=5, 3+5=8.

    // this was actually the example when I looked up how to use big int :shrug: so this if from rust docs
*/

use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::mem::replace;

// Calculate large fibonacci numbers.
fn fib(n: usize) -> BigUint {
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
    // This is a very large number.
    println!("fib(100) = {}", fib(100));
}
