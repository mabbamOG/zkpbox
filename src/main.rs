#![forbid(clippy::unwrap_used)] 
#![forbid(unsafe_code)]
#![forbid(clippy::indexing_slicing)]
#![warn(clippy::restriction)]
#![warn(clippy::pedantic)]


// 3 things need to implement now:
// 64-bit prime Field w/ arithmetic
// univariate interpolation & evaluation over prime fields
// merkle tree commitments

// 64-bit prime field
#[derive(Debug, Clone, Copy)]
struct Fp64(u64);
impl Fp64 {
    const PRIME_MODULUS: u64 = 18446744073709551557; // very large 64-bit prime
}
impl From<u64> for Fp64 {
    fn from(value: u64) -> Self {
        Fp64(value % Self::PRIME_MODULUS)
    }
}
impl std::ops::Add for Fp64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        todo!();
    }
}
impl std::ops::Sub for Fp64 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        todo!();
    }
}
impl std::ops::Mul for Fp64 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        todo!();
    }
}
impl std::ops::Div for Fp64 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        todo!();
    }
}

fn main() {
    println!("hello world!");
}