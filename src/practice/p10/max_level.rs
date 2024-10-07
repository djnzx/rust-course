use std::cmp::max;

fn max_level(s: String) -> u32 {
    let mut current_level = 0;
    let mut max_level = 0;
    for c in s.chars() {
        match c {
            '(' => {
                current_level += 1;
                max_level = max(max_level, current_level);
            }
            ')' => {
                current_level -= 1;
            }
            _ => panic!("Unexpected char {}", c),
        }
    }
    max_level
}

#[test]
fn max_level_test() {
    let test_data = [
        ("()()()", 1),
        ("()", 1),
        ("()(()())()", 2),
        ("()(()())((()()))(())()", 3),
    ];

    test_data
        .iter()
        .for_each(|&(s, n)| {
            assert_eq!(max_level(s.to_string()), n);
        });
}
