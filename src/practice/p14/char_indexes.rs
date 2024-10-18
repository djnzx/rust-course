use itertools::Itertools;
use std::collections::HashMap;

fn char_indexes(text: &String) -> HashMap<char, Vec<usize>> {
    let mut outcome: HashMap<char, Vec<usize>> = HashMap::new();
    text.chars()
        .into_iter()
        .zip(1..)
        .filter(|&(c, _)| c.is_alphabetic())
        .for_each(|(c, idx)| {
            outcome
                .entry(c.to_ascii_lowercase())
                .or_insert_with(Vec::new)
                .push(idx);
        });

    outcome
}

// with help of https://docs.rs/itertools/latest/itertools/
fn char_indexes2(text: &String) -> HashMap<char, Vec<usize>> {
    text.chars()
        .into_iter()
        .zip(1..)
        .filter(|&(c, _)| c.is_alphabetic())
        .map(|(c, idx)| (c.to_ascii_lowercase(), idx))
        .into_group_map()
}

#[test]
fn char_indexes_test() {
    let s = String::from("Hello, world!");

    let indexes1 = char_indexes(&s);
    println!("{:?}", indexes1);

    let indexes2 = char_indexes2(&s);
    println!("{:?}", indexes2);

    assert_eq!(indexes1, indexes2);
}
