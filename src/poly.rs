trait Field: From<u64> 
+ std::ops::Mul<Self, Output = Self> 
+ std::ops::Add<Self, Output = Self> 
+ Copy 
+ std::ops::Sub<Self, Output=Self> 
+ std::ops::Div<Self, Output=Self> 
+ std::ops::Neg<Output=Self>
+ std::fmt::Display
{
    fn pow(self, exp: u64) -> Self;
    fn inv(self) -> Self;
}

// very inefficient and sparse!
pub struct Poly<F>(Vec<F>);
impl<F: Field> Poly<F> {
    fn deg(self) -> usize {
        self.0.len() - 1
    }
    // calculates multiplicative inverse using extended euclidean
    pub fn inv(self) -> Self {
        
    }
    fn eval(self, x: F) -> F {
        self.0.iter().fold((F::from(0), F::from(1)), |(result, power), coefficient| {
            (result + *coefficient * power, power * x)
        }).0
    }
    fn interp(points: &[(F,F)]) -> Self {
        let vanishing_poly = points.iter().fold(Self(vec![]), |acc, (index,_)| {
            acc * Self(vec![-*index, F::from(1)])
        });
        points.iter().fold(Self(vec![]), |acc, (index, value)| {
            let x_minus_index = Self::from((1,F::from(1))) - Self::from((0,*index));
            let quasi_vanishing_poly = vanishing_poly / x_minus_index;
            let lagrange_poly = quasi_vanishing_poly / Self::from((0, quasi_vanishing_poly.eval(*index).inv()));
            acc + Self::from((0,*value)) * lagrange_poly
        })
    }
}
impl<F:Field> From<(u64,F)> for Poly<F> {
    fn from((degree, value): (u64,F)) -> Self {
        let mut v = vec![F::from(0); (degree+1) as usize];
        v[degree as usize] = value;
        Self(v)
    }
}
impl<F:Field> std::ops::Add for Poly<F> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.0.iter_mut().zip(rhs.into_iter())

    }
}
impl<F:Field> std::ops::Mul for Poly<F> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {

    }
}
impl<F:Field> std::ops::Neg for Poly<F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(self.0.into_iter().map(|coeff| -coeff).collect())
    }
}
impl<F:Field> std::ops::Sub for Poly<F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        use std::ops::Neg;
        self + rhs.neg()
    }
}
impl<F:Field> std::ops::Div for Poly<F> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}
impl<F:Field> std::fmt::Display for Poly<F> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = if self.0.len() != 0 {
            self.0.iter().enumerate().skip(1).fold(format!("{}",self.0[0]), |acc, (index, coeff)| {
                let coeff = *coeff;
                // acc + format!(" + {*coeff}x^{index}")
                acc.push_str("".to_string())
            })
        } else {"".to_string()};
        write!(fmt, "{s}")
    }
}