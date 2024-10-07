fn is_valid(s: String) -> bool {
    let mut count = 0;
    for c in s.chars() {
        match c {
            '{' => count += 1,
            '}' if count == 0 => return false,
            '}' => count -= 1,
            _ => panic!("Invalid char: {}", c),
        }
    }
    count == 0
}

#[test]
fn is_valid_test() {
    let test_data = [
        ("{}{}{{{}}{}}{}{}", true),
        ("{}{}{{{}}{}}{}{}}", false),
        ("{}{}{{{}}{}}{}{", false),
        ("}{", false),
        ("}{}{}{{{}}{}}{}{}", false),
    ];

    test_data
        .iter()
        .for_each(|&(s, v)| assert_eq!(is_valid(s.to_string()), v));
}
