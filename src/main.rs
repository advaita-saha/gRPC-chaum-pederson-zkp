use rand::Rng;

fn exponentiate(num: u32, exp: u32, p: u32) -> u32 {
    num.pow(exp) % p
}

fn solve(x: u32, k: u32, c: u32, q: u32) -> u32 {
    // s = (k - c * x) mod q
    let s = (k as i32 - (c * x) as i32) % q as i32;
    if s >= 0 {
        s as u32
    } else {
        (q as i32 + s) as u32
    }
}

fn verify(p: u32, y1: u32, y2: u32, r1: u32, r2: u32, g: u32, h: u32, c: u32, s: u32) -> bool {
    // R1 = g ^ s * Y1 ^ c
    let eq1 = r1 == (exponentiate(g, s, p) % p * exponentiate(y1, c, p) % p) % p;
    // R2 = h ^ s * Y2 ^ c
    let eq2 = r2 == (exponentiate(h, s, p) % p * exponentiate(y2, c, p) % p) % p;

    eq1 && eq2
}

fn random_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

fn main() {
    println!("{}", random_number());
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponentiate() {
        assert_eq!(exponentiate(1, 2, 11), 1);
        assert_eq!(exponentiate(2, 3, 11), 8);
        assert_eq!(exponentiate(2, 4, 11), 5);
    }

    #[test]
    fn test_solve() {
        // (10 - 2 * 1) mod 101 = 8
        assert_eq!(solve(2, 10, 1, 101), 8);
        // (10 - 2 * 6) mod 101 = 99
        assert_eq!(solve(2, 10, 6, 101), 99);
    }

    #[test]
    fn test_verify() {
        // p = 23, g = 4, h = 9
        // x = 6, k = 7, c = 4, s = 5
        // y1 = 2, y2 = 3
        // r1 = 8, r2 = 4
        assert!(verify(23, 2, 3, 8, 4, 4, 9, 4, 5));
        assert!(!verify(23, 2, 3, 8, 4, 4, 9, 4, 6));
        assert!(!verify(23, 2, 3, 8, 4, 4, 9, 4, 7));
        assert!(!verify(23, 2, 3, 8, 4, 4, 9, 4, 8));
        assert!(!verify(23, 2, 3, 8, 4, 4, 9, 4, 2));
    }
}
