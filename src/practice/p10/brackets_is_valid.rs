fn is_valid(s: String) -> bool {
    let mut count = 0;
    for c in s.chars() {
        match c {
            '{' => count += 1,
            '}' if count == 0 => return false,
            '}' => count -= 1,
            _ => {}
        }
    }
    count == 0
}

#[test]
fn is_valid_test2() {
    let test_data = [
        //input expected
        ("{}", true),
        ("{}{}", true),
        ("{{}}", true),
        ("{{}{}}", true),
        ("{{}", false),
        ("{}}", false),
        ("}{", false),
        ("{", false),
        ("}", false),
    ];

    for (s, r) in test_data {
        assert_eq!(is_valid(s.to_string()), r);
    }
}

#[test]
fn is_valid_test() {
    let cases = [
        //input expected
        ("{}", true),
        ("{}{}", true),
        ("{{}}", true),
        ("{{}{}}", true),
        ("{{}", false),
        ("{}}", false),
        ("}{", false),
        ("{", false),
        ("}", false),
    ];

    cases
        .iter()
        .for_each(|(input, expected)| {
            let real = is_valid(input.to_string());
            println!("Case {}: expected: {}, real: {}", input, expected, real);
            assert_eq!(real, *expected);
        });
}
