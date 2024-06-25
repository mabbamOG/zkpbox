#![forbid(unsafe_code)]

// 3 things need to implement now:
// 1. 64-bit prime Field w/ arithmetic
mod field;
// 2. univariate interpolation & evaluation over prime fields
mod poly;
// 3. merkle tree commitments
mod stark;

fn main() {

    //use field::Fp64;
    // println!("hello world!");
    // let x = Fp64::from(5);
    // let x_inv = x.inv();
    // println!("x: {:?}, x_inv: {:?}", x, x_inv);
    // println!("x * x_inv: {:?}", x * x_inv);
}
