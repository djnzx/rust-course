use std::collections::{HashMap, HashSet};

fn brackets_is_valid_full0(s: String) -> bool {
    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            '{' => stack.push(c),
            '(' => stack.push(c),
            '[' => stack.push(c),
            '<' => stack.push(c),
            '}' => match stack.pop() {
                None => return false,
                Some(top) => {
                    if top != '{' {
                        return false;
                    }
                }
            },
            ')' => match stack.pop() {
                None => return false,
                Some(top) => {
                    if top != '(' {
                        return false;
                    }
                }
            },
            ']' => match stack.pop() {
                None => return false,
                Some(top) => {
                    if top != '[' {
                        return false;
                    }
                }
            },
            '>' => match stack.pop() {
                None => return false,
                Some(top) => {
                    if top != '<' {
                        return false;
                    }
                }
            },
            _ => {}
        }
    }

    stack.is_empty()
}

fn brackets_is_valid_full1(s: String) -> bool {
    let open = HashSet::<char>::from_iter("{[(<".chars());
    let mut stack = Vec::new();

    fn matches(op: char, cur: char) -> bool {
        match (op, cur) {
            ('<', '>') => true,
            ('[', ']') => true,
            ('(', ')') => true,
            ('{', '}') => true,
            _ => false,
        }
    }

    for c in s.chars() {
        match c {
            c if open.contains(&c) => stack.push(c),
            c => match stack.pop() {
                None => return false,
                Some(x) => {
                    if !matches(x, c) {
                        return false;
                    }
                }
            },
            _ => {}
        }
    }

    stack.is_empty()
}

/// straightforward
fn brackets_is_valid_full2(s: String) -> bool {
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

fn brackets_is_valid_full3(s: String) -> bool {
    let open = HashSet::<char>::from_iter("{[(<".chars());
    let pairs = HashMap::from([('}', '{'), (')', '('), (']', '['), ('>', '<')]);

    let matches = |close: char, open: char| match pairs.get(&close) {
        Some(&top) => top == open,
        _ => false,
    };

    let mut stack = Vec::new();

    for c in s.chars() {
        match c {
            c if open.contains(&c) => stack.push(c),
            c if pairs.contains_key(&c) => match stack.pop() {
                Some(top) if matches(c, top) => {}
                _ => return false,
            },
            _ => continue,
        }
    }

    stack.is_empty()
}

/// more functional
fn brackets_is_valid_full4(s: String) -> bool {
    let pairs = HashMap::from([('}', '{'), (')', '('), (']', '['), ('>', '<')]);
    let open = HashSet::<char>::from_iter("{[(<".chars());
    let close = HashSet::<char>::from_iter("}])>".chars());

    enum State {
        Valid(Vec<char>),
        Invalid,
    }

    impl State {
        fn new() -> Self {
            State::Valid(Vec::new())
        }
    }

    let process = |mut st: State, c: char| match st {
        State::Valid(ref mut stack) => match c {
            c if open.contains(&c) => {
                stack.push(c);
                st
            }
            c if close.contains(&c) => match stack.pop() {
                Some(top) => match pairs.get(&c) {
                    Some(&op) if op == top => st,
                    _ => State::Invalid,
                },
                _ => State::Invalid,
            },
            _ => st,
        },
        _ => State::Invalid,
    };

    let s = s
        .chars()
        .fold(State::new(), process);

    match s {
        State::Valid(stack) => stack.is_empty(),
        State::Invalid => false,
    }
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
        assert_eq!(brackets_is_valid_full1(s.to_string()), r);
        assert_eq!(brackets_is_valid_full2(s.to_string()), r);
        assert_eq!(brackets_is_valid_full3(s.to_string()), r);
    }
}
