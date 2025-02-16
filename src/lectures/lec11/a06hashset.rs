use std::collections::*;

#[test]
fn test1_no_duplicates() {
    let mut pp = HashSet::new();
    pp.insert("Alex");
    pp.insert("Jim");
    pp.insert("Sergio");
    pp.insert("Alex");
    pp.insert("Jim");

    assert_eq!(pp.contains("Alex"), true);
    assert_eq!(pp.contains("Alexander"), false);
    assert_eq!(pp.len(), 3);
}

#[test]
fn test2_different_ordering() {
    let mut pp = HashSet::new();
    pp.insert("Alex");
    pp.insert("Jim");
    pp.iter()
        .for_each(|x| println!("{x}")); // Jim, Alex

    pp.insert("Sergio");
    pp.iter()
        .for_each(|x| println!("{x}")); // Sergio, Jim, Alex
}
