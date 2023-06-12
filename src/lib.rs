use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::Rng;

pub const G: u32 = 4u32;
pub const H: u32 = 9u32;
pub const P: u32 = 23u32;
pub const Q: u32 = 11u32;

pub fn exponentiate(num: &BigUint, exp: &BigUint, p: &BigUint) -> BigUint {
    num.modpow(exp, p)
}

pub fn solve(x: &BigUint, k: &BigUint, c: &BigUint, q: &BigUint) -> BigUint {
    // s = (k - c * x) mod q
    // let s = (k as i32 - (c * x) as i32) % q as i32;
    // if s >= 0 {
    //     s as u32
    // } else {
    //     (q as i32 + s) as u32
    // }
    if k >= &(c * x) {
        k - (c * x).modpow(&BigUint::one(), q)
    } else {
        q - (c * x - k).modpow(&BigUint::one(), q)
    }
}

pub fn verify(
    p: &BigUint,
    y1: &BigUint,
    y2: &BigUint,
    r1: &BigUint,
    r2: &BigUint,
    g: &BigUint,
    h: &BigUint,
    c: &BigUint,
    s: &BigUint,
) -> bool {
    // R1 = g ^ s * Y1 ^ c
    let eq1 = *r1 == (exponentiate(g, s, p) * exponentiate(y1, c, p)).modpow(&BigUint::one(), p);
    // R2 = h ^ s * Y2 ^ c
    let eq2 = *r2 == (exponentiate(h, s, p) * exponentiate(y2, c, p)).modpow(&BigUint::one(), p);

    eq1 && eq2
}

pub fn random_number() -> BigUint {
    let mut rng = rand::thread_rng();
    rng.gen_biguint(256)
}

pub fn random_string(n: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponentiate() {
        assert_eq!(
            exponentiate(
                &BigUint::from(1u32),
                &BigUint::from(2u32),
                &BigUint::from(11u32)
            ),
            BigUint::from(1u32)
        );
        assert_eq!(
            exponentiate(
                &BigUint::from(2u32),
                &BigUint::from(3u32),
                &BigUint::from(11u32)
            ),
            BigUint::from(8u32)
        );
        assert_eq!(
            exponentiate(
                &BigUint::from(2u32),
                &BigUint::from(4u32),
                &BigUint::from(11u32)
            ),
            BigUint::from(5u32)
        );
    }

    #[test]
    fn test_solve() {
        // (10 - 2 * 1) mod 101 = 8
        assert_eq!(
            solve(
                &BigUint::from(2u32),
                &BigUint::from(10u32),
                &BigUint::from(1u32),
                &BigUint::from(101u32)
            ),
            BigUint::from(8u32)
        );
        // (10 - 2 * 6) mod 101 = 99
        assert_eq!(
            solve(
                &BigUint::from(2u32),
                &BigUint::from(10u32),
                &BigUint::from(6u32),
                &BigUint::from(101u32)
            ),
            BigUint::from(99u32)
        );
    }

    #[test]
    fn test_verify() {
        // p = 23, g = 4, h = 9
        // x = 6, k = 7, c = 4, s = 5
        // y1 = 2, y2 = 3
        // r1 = 8, r2 = 4
        let p = BigUint::from(23u32);
        let g = BigUint::from(4u32);
        let h = BigUint::from(9u32);
        let c = BigUint::from(4u32);
        let s = BigUint::from(5u32);
        let y1 = BigUint::from(2u32);
        let y2 = BigUint::from(3u32);
        let r1 = BigUint::from(8u32);
        let r2 = BigUint::from(4u32);

        assert!(verify(&p, &y1, &y2, &r1, &r2, &g, &h, &c, &s));
        assert!(!verify(
            &p,
            &y1,
            &y2,
            &r1,
            &r2,
            &g,
            &h,
            &c,
            &BigUint::from(6u32)
        ));
    }

    #[test]
    fn test_toy_example() {
        let q = BigUint::from(11u32);
        let p = BigUint::from(23u32);
        let g = BigUint::from(4u32);
        let h = BigUint::from(9u32);
        let x = BigUint::from(6u32);

        let y1 = exponentiate(&g, &x, &p);
        let y2 = exponentiate(&h, &x, &p);

        let k = random_number(); // limiting to 0-99 for test case simplicity

        let r1 = exponentiate(&g, &k, &p);
        let r2 = exponentiate(&h, &k, &p);

        let c = random_number();
        let s = solve(&x, &k, &c, &q);

        assert!(verify(&p, &y1, &y2, &r1, &r2, &g, &h, &c, &s));
    }
}
