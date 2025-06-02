fn is_valid(s: &str) -> bool {
    let mut count = 0;

    for c in s.chars() {
        if c == '{' {
            count += 1;
        } else if c == '}' {
            count -= 1;
        }
        if count < 0 {
            break;
        }
    }

    count == 0
}

#[test]
fn test_brackets() {
    let valid = ["", "{}", "{}{}", "{{}}", "{{}{}}"];

    let invalid = ["{", "}", "{{}", "{}}", "}{"];

    for s in valid {
        let result = is_valid(s);
        println!("input: {s}, expected: true, actual: {result}");
        assert_eq!(result, true);
    }

    for s in invalid {
        let result = is_valid(s);
        println!("input: {s}, expected: true, actual: {result}");
        assert_eq!(result, false);
    }
}
