// homeworks/hw07.rs
pub fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invert_case() {
        let test_cases = [
            ("Hello", "hELLO"),
            ("Привет", "пРИВЕТ"),
        ];

        for &(input, expected) in &test_cases {
            assert_eq!(invert_the_case(input.to_string()), expected.to_string());
            assert_eq!(invert_the_case(expected.to_string()), input.to_string());
        }
    }
}
