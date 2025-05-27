// homeworks/hw05.rs
pub fn gcd(a: u32, b: u32) -> u32 {
    let mut x = a;
    let mut y = b;
    
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        let test_cases = [
            ((24, 60), 12),
            ((15, 9), 3),
            ((15, 6), 3),
            ((140, 40), 20),
            ((24, 16), 8),
            ((100, 10), 10),
            ((120, 80), 40),
            ((80, 120), 40),
            ((100, 20), 20),
            ((37, 11), 1),
            ((120, 90), 30),
        ];

        for &((a, b), expected) in &test_cases {
            assert_eq!(expected, gcd(a, b));
        }
    }
}
