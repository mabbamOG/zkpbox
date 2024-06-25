use crate::poly::{Field, Poly};
use std::hash::{DefaultHasher, Hash, Hasher};
fn schwartz_zippel<F: Field>(left: &Poly<F>, right: &Poly<F>, error: f64) -> bool {
    let degree = left.degree().max(right.degree());
    let domain_size: u64 = (degree as f64 / error) as u64;
    // assert!(domain_size <= F::MODULUS, "wanted to run Schwartz-Zippel check with domain {domain_size}, but it's larger than our field modulus by {} elements", domain_size - F::MODULUS);
    if domain_size > F::MODULUS {
        let min_error = degree as f64 / F::MODULUS as f64;
        eprintln!("(was too large, going to need {:.2} iterations at 2^{:.2} to get to error 2^{:.2}) (improvement 2^{:.2} -> 2^{:.2})", error.log2() / min_error.log2(), min_error.log2(), error.log2(), error.log2(), (error/min_error).log2());
        return schwartz_zippel(&left, &right, error / min_error)
            & schwartz_zippel(&left, &right, min_error);
    }
    eprintln!(
        "(not too large at 2^{:.2} and |D| = 2^{:.2})",
        error.log2(),
        (domain_size as f64).log2()
    );
    let random = {
        let mut hasher = DefaultHasher::new();
        let fake_random = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_nanos();
        fake_random.hash(&mut hasher);
        left.hash(&mut hasher);
        right.hash(&mut hasher);
        error.to_ne_bytes().hash(&mut hasher);
        F::from(hasher.finish() % domain_size)
    };
    let left_eval = left.eval(random);
    let right_eval = right.eval(random);
    // eprintln!("query (d={degree} e={error} |D|~2^{domain_size}) at {random}: {left_eval} == {right_eval}: {}!", left_eval == right_eval);
    return left_eval == right_eval;
}

#[test]
fn starktest() {
    use crate::field::Fp64;
    use crate::poly::{Monomial, Point};
    type F = Fp64;
    const DEGREE: u32 = 50;
    let left = Poly::from(Monomial {
        coeff: F::from(3),
        degree: DEGREE,
    });
    let right = {
        let points: [Point<F>; DEGREE as usize + 1] = std::array::from_fn(|i| Point {
            x: F::from(i as u64),
            y: left.eval(F::from(i as u64)),
        });
        let wrong_points = {
            let mut wrong_points = points.clone();
            wrong_points[DEGREE as usize / 2].y = wrong_points[DEGREE as usize / 2].y + F::ONE;
            wrong_points
        };
        Poly::interpolate(&wrong_points)
    };
    eprintln!("attacker: {left} -> {right}");
    let error = 2_f64.powi(-100);
    let total = 10000;
    let mut count = 0;
    for _ in 0..total {
        let query = schwartz_zippel(&left, &right, error);
        count += query as u32;
    }
    eprintln!(
        "results: {count}/{total} successful attacks ({:.0}%), expected {:.0}%",
        count as f64 / total as f64 * 100.0,
        error * 100.0
    );
}
