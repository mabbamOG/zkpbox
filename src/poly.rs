//use std::{fmt::{self, write}, path::Display};
use std::hash::Hash;

pub trait Field:
    From<u64>
    + std::ops::Mul<Self, Output = Self>
    + std::ops::Add<Self, Output = Self>
    + Copy
    + std::ops::Sub<Self, Output = Self>
    + std::ops::Div<Self, Output = Self>
    + std::cmp::PartialEq
    + std::ops::Neg<Output = Self>
    + std::fmt::Display
    + std::fmt::Debug
    + Into<u64>
    + Hash
{
    const MODULUS: u64;
    const ZERO: Self;
    const ONE: Self;
    fn pow(self, exp: u64) -> Self;
    fn inv(self) -> Self;
}

#[derive(Clone, PartialEq, Debug, Hash)]
pub struct Monomial<F> {
    pub coeff: F,
    pub degree: u32,
}
impl<F: Field> Monomial<F> {
    // const ZERO: Self = Self {
    //     coeff: F::ZERO,
    //     degree: 0,
    // };
    // const ONE: Self = Self {
    //     coeff: F::ONE,
    //     degree: 0,
    // };
    // const X: Self = Self {
    //     coeff: F::ONE,
    //     degree: 1,
    // };
}
impl<F: Field> std::fmt::Display for Monomial<F> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let &Monomial { coeff, degree } = self;
        let coeff = coeff.into();
        match (coeff, degree) {
            (_, 0) => write!(fmt, "{coeff}"),
            (1, 1) => write!(fmt, "x"),
            (_, 1) => write!(fmt, "{coeff}x"),
            (1, _) => write!(fmt, "x^{degree}"),
            (_, _) => write!(fmt, "{coeff}x^{degree}"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Point<F> {
    pub x: F,
    pub y: F,
}
impl<F: Field> std::fmt::Display for Point<F> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Point { x, y } = self;
        write!(fmt, "({x}, {y})")
    }
}

// very inefficient and sparse!
#[derive(Clone, Debug, Hash)]
pub struct Poly<F>(Vec<Monomial<F>>);
impl<F: Field> Poly<F> {
    // TODO: remove!
    fn new() -> Self {
        Self(vec![])
    }

    // fn new_x() -> Self {
    //    Self::from_linear(F::ONE)
    // }

    // fn new_one() -> Self {
    //    Self::from_constant(F::ONE)
    // }

    // TODO: remove
    fn from_constant(coeff: F) -> Self {
        Self(vec![Monomial { coeff, degree: 0 }])
    }

    // TODO: remove
    fn from_linear(coeff: F) -> Self {
        Self(vec![Monomial { coeff, degree: 1 }])
    }

    fn is_zero(self) -> bool {
        match self.0[..] {
            [] => true,
            [Monomial { coeff, degree }] => coeff == F::ZERO && degree == 0,
            _ => false,
        }
    }

    fn simplify(self) -> Self {
        let mut tmp = self.0.clone();
        let mut result = vec![];
        for i in 0..tmp.len() {
            for j in i + 1..tmp.len() {
                if tmp[i].degree == tmp[j].degree {
                    tmp[i].coeff = tmp[i].coeff + tmp[j].coeff;
                    tmp[j].coeff = F::ZERO;
                }
            }
            if tmp[i].coeff != F::ZERO {
                result.push(tmp[i].clone());
            }
        }
        // eprintln!("simplify({} = {})", Self(self.0.clone()), Self(result.clone()));
        Self(result)
    }

    pub fn degree(&self) -> u32 {
        self.0.iter().map(|mon| mon.degree).max().unwrap_or(0)
    }
    // calculates multiplicative inverse using extended euclidean
    pub fn inv(self) -> Self {
        if let [Monomial { coeff, degree }] = self.0[..] {
            assert!(degree == 0);
            let coeff = coeff.inv();
            return Self(vec![Monomial { coeff, degree: 0 }]);
        }
        unimplemented!("{self}")
    }

    pub fn eval(&self, x: F) -> F {
        let mut result = F::ZERO;
        for Monomial { coeff, degree } in &self.0 {
            result = result + *coeff * x.pow(*degree as u64)
        }
        result
    }

    pub fn interpolate(points: &[Point<F>]) -> Self {
        // let vanishing_poly = {
        //     let mut vanishing_poly = Self::new();
        //     for point in points {
        //         let x_minus_index = Self::from_linear(F::ONE) - Self::from_constant(point.x);
        //         vanishing_poly = vanishing_poly * x_minus_index;
        //     }
        //     vanishing_poly
        // };
        eprint!("interpolating...");
        let mut result = Self::new();
        //let mut result = Self::from(Monomial::ZERO);
        for (i, point) in points.iter().enumerate() {
            let total = points.len();
            let threshold = if total > 10 { total / 10 } else { total }; // 10%
            if (i + 1) % threshold == 0 {
                eprint!("{}", (i + 1) / threshold);
                use std::io::Write;
                std::io::stderr().flush().unwrap();
            }
            // eprintln!("> doing point {i}: {point}");
            let lagrange_base = {
                // let x_minus_index = Self::from_linear(F::ONE) - Self::from_constant(point.x);
                // let quasi_vanishing_poly = vanishing_poly.clone() / x_minus_index;
                let quasi_vanishing_poly = {
                    let mut quasi_vanishing_poly = Self::from_constant(F::ONE);
                    for (j, point2) in points.iter().enumerate() {
                        if i != j {
                            let x_minus_index =
                                Self::from_linear(F::ONE) - Self::from_constant(point2.x);
                            // eprintln!(">> x_minus_index {i}: {} - {} = {x_minus_index}", Self::from_linear(F::ONE), Self::from_constant(point2.x));
                            quasi_vanishing_poly = quasi_vanishing_poly * x_minus_index;
                        }
                    }
                    quasi_vanishing_poly
                };
                // eprintln!("> got quasi_vanishing_poly: {quasi_vanishing_poly}");
                let denominator = Poly::from_constant(quasi_vanishing_poly.clone().eval(point.x));
                // eprintln!("> got denominator: {denominator}");
                quasi_vanishing_poly / denominator
            };
            // eprintln!("> got lagrange base: {lagrange_base}");
            let yi = Poly::from_constant(point.y);
            // eprintln!("> got yi: {yi}");
            let lagrange_term = yi * lagrange_base;
            // eprintln!("> got lagrange term: {lagrange_term}");
            result = result + lagrange_term;
            // eprintln!("> got partial sum: {result}");
            // eprintln!();
        }
        eprintln!("...done");
        result
    }
}
impl<F: Field> From<Monomial<F>> for Poly<F> {
    fn from(m: Monomial<F>) -> Self {
        Self(vec![m])
    }
}
impl<F: Field> std::ops::Add for Poly<F> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = vec![];
        for mon in &self.0 {
            result.push(mon.clone());
        }
        for mon in &rhs.0 {
            result.push(mon.clone());
        }
        let output = Self(result);
        // eprintln!("add({self} + {rhs} = {output})");
        output.simplify()
    }
}
impl<F: Field> std::ops::Mul for Poly<F> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = vec![];
        for left in &self.0 {
            for right in &rhs.0 {
                let coeff = left.coeff * right.coeff;
                let degree = left.degree + right.degree;
                result.push(Monomial { coeff, degree })
            }
        }
        let result = Self(result);
        // eprintln!("mul({self} * {rhs} = {result})");
        result.simplify()
    }
}
impl<F: Field> std::ops::Neg for Poly<F> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let mut result = vec![];
        for mon in self.0 {
            result.push(Monomial {
                coeff: -mon.coeff,
                degree: mon.degree,
            });
        }
        Self(result)
    }
}
impl<F: Field> std::ops::Sub for Poly<F> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        use std::ops::Neg;
        let neg = rhs.clone().neg();
        // eprintln!("sub({self} - {rhs} = {self} + {neg})");
        let output = self + neg;
        output
    }
}
impl<F: Field> std::ops::Div for Poly<F> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let inv = rhs.clone().inv();
        // eprintln!("div({self} / {rhs} = {self} * {inv})");
        let output = self * inv;
        output
    }
}
impl<F: Field> PartialEq for Poly<F> {
    fn eq(&self, other: &Self) -> bool {
        (self.clone() - other.clone()).is_zero()
    }
}
impl<F: Field> Eq for Poly<F> {}
impl<F: Field> std::fmt::Display for Poly<F> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "(")?;
        for mon in self.0.iter().take(1) {
            write!(fmt, "{mon}")?;
        }
        for mon in self.0.iter().skip(1) {
            write!(fmt, " + {mon}")?;
        }
        if self.0.len() == 0 {
            write!(fmt, "0")?;
        }
        write!(fmt, ")")?;
        Ok(())
    }
}

#[test]
fn polytest() {
    use crate::field::Fp64;
    impl Field for crate::field::Fp64 {
        const ZERO: Self = Self::ZERO;
        const ONE: Self = Self::ONE;
        const MODULUS: u64 = Self::MODULUS;
        fn pow(self, exp: u64) -> Self {
            Self::pow(self, exp)
        }
        fn inv(self) -> Self {
            Self::inv(self)
        }
    }
    let linear_points: [Point<Fp64>; 30] = std::array::from_fn(|i| Point {
        x: Fp64::from(i as u64),
        y: Fp64::from(i as u64),
    });
    let linear_poly = Poly::from_linear(Fp64::ONE);
    let interpolation = Poly::interpolate(&linear_points);
    assert!(
        interpolation == linear_poly,
        "expected {linear_poly}, got {interpolation}"
    );
    let evaluation: [Point<Fp64>; 30] = std::array::from_fn(|i| Point {
        x: Fp64::from(i as u64),
        y: interpolation.eval(Fp64::from(i as u64)),
    });
    assert!(evaluation == linear_points);

    let square_poly = linear_poly.clone() * linear_poly;
    let square_points: [Point<Fp64>; 30] = std::array::from_fn(|i| Point {
        x: Fp64::from(i as u64),
        y: Fp64::from(i as u64).pow(2),
    });
    let interpolation = Poly::interpolate(&square_points);
    assert!(
        interpolation == square_poly,
        "expected {square_poly}, got {interpolation}"
    );
    let evaluation: [Point<Fp64>; 30] = std::array::from_fn(|i| Point {
        x: Fp64::from(i as u64),
        y: interpolation.eval(Fp64::from(i as u64)),
    });
    assert!(evaluation == square_points);

    let m3x3 = Monomial {
        coeff: Fp64::from(3),
        degree: 3,
    };
    let m6x5 = Monomial {
        coeff: Fp64::from(6),
        degree: 5,
    };
    let left = Poly(vec![m3x3.clone(), m6x5.clone()]);
    let right = Poly(vec![m6x5.clone(), m3x3.clone()]);
    assert!(left == right, "expected {left} == {right}");
    let left = Poly::from(m3x3.clone()) + Poly::from(m6x5.clone());
    let right = Poly::from(m6x5) + Poly::from(m3x3);
    assert!(left == right, "expected {left} == {right}");

    let m_99x99 = Poly::from(Monomial {
        coeff: Fp64::from(99),
        degree: 99,
    });
    let m_11x11 = Poly::from(Monomial {
        coeff: Fp64::from(11),
        degree: 11,
    });
    let m_33x33 = Poly::from(Monomial {
        coeff: Fp64::from(33),
        degree: 33,
    });
    let special_poly = m_99x99 + m_11x11 + m_33x33;
    let special_points: [Point<Fp64>; 100] = std::array::from_fn(|i| Point {
        x: Fp64::from(i as u64),
        y: special_poly.eval(Fp64::from(i as u64)),
    });
    let interpolation = Poly::interpolate(&special_points);
    assert!(
        special_poly == interpolation,
        "expected {special_poly}, got {interpolation}"
    );
    let evaluation: [Point<Fp64>; 100] = std::array::from_fn(|i| Point {
        x: Fp64::from(i as u64),
        y: interpolation.eval(Fp64::from(i as u64)),
    });
    assert!(evaluation == special_points);
}
