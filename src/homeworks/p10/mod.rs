pub fn is_palindrome(x: u32) -> bool {
    if x < 10 {
        return true;
    }

    let s = x.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();

    for i in 0..len / 2 {
        if bytes[i] != bytes[len - 1 - i] {
            return false;
        }
    }
    true
}

#[test]
fn test() {
    let data = [
        (123, false),
        (121, true),
        (1221, true),
        (1, true),        // single digit
        (11, true),       // even length
        (12321, true),    // odd length
        (1234321, true),  // larger palindrome
        (123456, false),  // non-palindrome
    ];

    data.iter().for_each(|(n, exp)| {
        assert_eq!(is_palindrome(*n), *exp);
    });
}
