use std::ops::{Mul,Add, Sub, Div, Neg};

trait Field: From<u64> + Mul<Self, Output = Self> + Add<Self, Output = Self> + Copy + Sub<Self, Output=Self> + Div<Self, Output=Self> + Neg<Output=Self>{}

struct Polynomial<F>  {
    coefficients: Vec<F>,
} 

impl<F: Field> Polynomial<F> {
    fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    fn evaluate(&self, x: F) -> F {
        let mut result = F::from(0);
        let mut x_power = F::from(1);
        for i in 0..self.coefficients.len() {
            result = result + self.coefficients[i] * x_power;
            x_power = x_power * x;
        }
        result
    }

    fn interpolate(points:&[(F, F)]) -> Vec<F> {
        todo!()
    }

    // fn interpolate(points: &[(F, F)]) -> Self {
    //     // let mut coefficients = vec![F::from(0); points.len()];
    //     // for i in 0..points.len() {
    //     //     let mut term = F::from(1);
    //     //     for j in 0..points.len() {
    //     //         if i != j {
    //     //             term = term * (points[j].0 - points[i].0);
    //     //         }
    //     //     }
    //     //     term = points[i].1 / term;
    //     //     for j in 0..points.len() {
    //     //         if i != j {
    //     //             term = term * (F::from(-1) * points[j].0);
    //     //         }
    //     //     }
    //     //     coefficients[i] = term;
    //     // }
    //     // Self { coefficients }
    // }
}

impl<F:Field> From<&[(F,F)]> for Polynomial<F> {
    fn from(points: &[(F,F)]) -> Self {
        let coefficients = Self::interpolate(&points);
        Self { coefficients }
    }
}