#![forbid(clippy::unwrap_used)] 
#![forbid(unsafe_code)]
#![forbid(clippy::indexing_slicing)]
#![warn(clippy::restriction)]
#![warn(clippy::pedantic)]


// 3 things need to implement now:
// 1. 64-bit prime Field w/ arithmetic
mod fp64;
// 2. univariate interpolation & evaluation over prime fields
mod poly;
// 3. merkle tree commitments


use fp64::Fp64;
fn main() {
    println!("hello world!");
    let x = Fp64::from(5);
    let x_inv = x.inv();
    println!("x: {:?}, x_inv: {:?}", x, x_inv);
    println!("x * x_inv: {:?}", x * x_inv);
}