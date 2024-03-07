
// 64-bit prime field, guarantees that all operations are in the field
#[derive(Debug, Clone, Copy)]
pub struct Fp64(u64);
impl Fp64 {
    // very large 64-bit prime
    const PRIME_MODULUS: u64 = 18446744073709551557;

    // calculates the additive inverse
    pub fn neg(self) -> Self {
        Fp64(Self::PRIME_MODULUS - self.0)
    }

    // calculates the multiplicative inverse
    pub fn inv(self) -> Self {
        // Fermat's little theorem
        self.pow(Self::PRIME_MODULUS - 2)
    }

    // modular exponentiation by repeated squaring
    pub fn pow(self, exp: u64) -> Self {
        if exp == 0 {
            Fp64(1)
        } else if exp == 1 {
            self
        } else {
            self.pow(exp % 2) * (self * self).pow(exp >> 1)
        }
    }
}
impl From<u64> for Fp64 {
    fn from(value: u64) -> Self {
        Fp64(value % Self::PRIME_MODULUS)
    }
}
impl std::ops::Add for Fp64 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let tmp: u128 = (self.0 as u128 + rhs.0 as u128) % Self::PRIME_MODULUS as u128;
        Fp64(tmp as u64)
    }
}
impl std::ops::Sub for Fp64 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.neg()
    }
}
impl std::ops::Mul for Fp64 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let tmp: u128 = (self.0 as u128 * rhs.0 as u128) % Self::PRIME_MODULUS as u128;
        Fp64(tmp as u64)
    }
}
impl std::ops::Div for Fp64 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}