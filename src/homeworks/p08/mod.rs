// homeworks/hw08.rs
pub fn is_prime(n: &u32) -> bool {
    match *n {
        0 | 1 => false,
        2 => true,
        _ if *n % 2 == 0 => false,
        _ => {
            let sqrt_n = (*n as f64).sqrt() as u32;
            !(3..=sqrt_n).step_by(2).any(|i| *n % i == 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let test_data = [
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (100, false),
            (10007, true),
        ];

        test_data
            .iter()
            .for_each(|(n, prime)| assert_eq!(is_prime(n), *prime))
    }
}
