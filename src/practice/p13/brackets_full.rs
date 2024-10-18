use std::collections::{HashMap, HashSet};

fn brackets_is_valid_full(s: String) -> bool {
    let pairs = HashMap::from([('}', '{'), (')', '('), (']', '['), ('>', '<')]);
    let open = HashSet::<char>::from_iter("{[(<".chars());
    let close = HashSet::<char>::from_iter("}])>".chars());

    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            c if open.contains(&c) => stack.push(c),
            c if close.contains(&c) => match stack.pop() {
                None => return false,
                Some(top) => match pairs.get(&c) {
                    Some(&op) if op == top => (),
                    Some(_) | None => return false,
                },
            },
            _ => continue,
        }
    }

    stack.is_empty()
}

#[test]
fn is_valid_test2() {
    let test_data = [
        //input expected
        ("{}", true),
        ("[{(<>()[])}<>]", true),
        ("{}{}", true),
        ("{{}}", true),
        ("{{}{}}", true),
        ("{{}", false),
        ("{}}", false),
        ("}{", false),
        ("{", false),
        ("}", false),
        ("<}", false),
    ];

    for (s, r) in test_data {
        assert_eq!(brackets_is_valid_full(s.to_string()), r);
    }
}
