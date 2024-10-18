fn shoes_groups(s: String) -> usize {
    let mut groups = 0;
    let mut count = 0;
    for c in s.chars() {
        match c {
            'L' => count += 1,
            'R' => count -= 1,
            _ => continue,
        }
        if count == 0 {
            groups += 1;
        }
    }
    groups
}

#[test]
fn shoes_groups_test() {
    let cases = [
        //input expected
        ("RL", 1),
        ("RLRRLLRLRRLL", 4),
        ("RLLLRRRLLR", 4),
        ("LLRLRLRLRLRLRR", 1),
    ];

    cases
        .iter()
        .for_each(|(input, expected)| {
            let real = shoes_groups(input.to_string());
            println!("Case: {:15}: expected: {}, real: {}", input, expected, real);
            assert_eq!(real, *expected);
        });
}
