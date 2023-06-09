fn exponentiate(num: u32, exp: u32, p:u32) -> u32 {
    num.pow(exp) % p
}

fn main() {
    println!("Hello, world!");
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponentiate(){
        assert_eq!(exponentiate(1, 2, 11), 1);
        assert_eq!(exponentiate(2, 3, 11), 8);
        assert_eq!(exponentiate(2, 4, 11), 5);
    }
}
