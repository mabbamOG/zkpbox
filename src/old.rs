// x = x1 * 2^128 + x0
#[derive(Debug)]
struct U256(u128, u128);

impl From<&str> for U256 {
    fn from(s: &str) -> Self {
        const NUMBERS: [char;10] = ['0','1','2','3','4','5','6','7','8','9'];
        let (mut t0, mut t1) = (None, None);
        let mut tmp: u128 = 0;
        let mut multiplier: u128 = 1;
        for c in s.chars().rev() {
            assert!(NUMBERS.contains(&c), "U256::from: one of the characters is not a number!");
            let n = match (c as u128 - '0' as u128).checked_mul(multiplier) {
                Some(v) => v,
                None => {
                    match (t1, t0) {
                        (None, None) => t0 = Some(tmp),
                        (None, Some(_)) => t1 = Some(tmp),
                        (Some(_), Some(_)) => break,
                        (Some(_), None) => panic!()
                    };
                    tmp = c as u128 - '0' as u128;
                    multiplier = 10;
                    continue;
                }
            };
            tmp = match tmp.checked_add(n) {
                Some(v) => v,
                None => {
                    match (t1, t0) {
                        (None, None) => t0 = Some(tmp),
                        (None, Some(_)) => t1 = Some(tmp),
                        (Some(_), Some(_)) => break,
                        (Some(_), None) => panic!()
                    };
                    tmp = c as u128 - '0' as u128;
                    multiplier = 10;
                    continue;
                }
            };
            multiplier = match multiplier.checked_mul(10) {
                Some(v) => v,
                None => {
                    match (t1, t0) {
                        (None, None) => t0 = Some(tmp),
                        (None, Some(_)) => t1 = Some(tmp),
                        (Some(_), Some(_)) => break,
                        (Some(_), None) => panic!()
                    };
                    tmp = 0;
                    multiplier = 1;
                    continue;
                }
            };
        }
        match (t1, t0) {
            (Some(v1), Some(v0)) => U256(v1,v0),
            (None, Some(v0)) => U256(tmp, v0),
            (None, None) => U256(0, tmp),
            (Some(_), None) => panic!()
        }
    }
}
impl std::fmt::Display for U256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            U256(0, v0) => format!("{v0}"),
            U256(v1, v0) => format!("{v1}{v0:039}"),
        };
        write!(f, "{s}")
    }
}


fn main() {
    let a = "1234";
    println!("{a}\n{}", U256::from(a));
    let b = "182347932748923";
    println!("{b}\n{}", U256::from(b));
    let c = "9000008000007000006000005000004000003000002000001000001";
    println!("{c}\n{}", U256::from(c));
    let d = "100000000000000000000000000000000000000";
    println!("{d}\n{}", U256::from(d));
    println!("{:?}", U256::from(d));
}