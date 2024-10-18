use std::cmp::max;

fn max_level(s: String) -> usize {
    let mut count = 0;
    let mut max_level = 0;
    for c in s.chars() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => continue,
        }
        max_level = max(max_level, count);

        // if count > max_level {
        //     max_level = count;
        // }
    }

    max_level
}

#[test]
fn max_level_test() {
    let test_data = [
        ("(a)", 1),
        ("()", 1),
        ("(b)(c)", 1),
        ("(())", 2),
        ("(()())", 2),
        ("((()))", 3),
        ("()(x)()", 1),
        ("((()((()))())())", 5),
    ];

    test_data
        .iter()
        .for_each(|(input, level_expected)| {
            let real = max_level(input.to_string());
            println!(
                "case: {:16} real:{} expected:{}",
                input, real, level_expected
            );
            assert_eq!(real, *level_expected);
        });
}
