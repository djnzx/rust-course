use std::collections::BTreeSet;

#[test]
fn test1() {
    let mut xs = BTreeSet::new();

    xs.insert(1);
    xs.insert(3);
    xs.insert(3);
    xs.insert(5);
    xs.insert(7);
    xs.insert(8);

    assert_eq!(xs.len(), 5);
    assert_eq!(xs.contains(&6), false);
}

#[test]
fn test2_range() {
    let mut xs = BTreeSet::new();

    xs.insert(1);
    xs.insert(3);
    xs.insert(5);
    xs.insert(7);
    xs.insert(9);
    xs.insert(11);

    let lower = xs
        .range(..6)
        .next_back();
    let upper = xs.range(6..).next();
    assert_eq!(lower, Some(&5));
    assert_eq!(upper, Some(&7));
}

#[test]
fn test3_min_max() {
    let xs = BTreeSet::from([1, 2, 3, 5, 7, 9, 11]);

    assert_eq!(xs.first(), Some(&1));
    assert_eq!(xs.last(), Some(&11));
}

#[test]
fn test4_iter() {
    let xs = BTreeSet::from([11, 2, 7, 5, 3, 9, 1]);

    xs.iter()
        .for_each(|x| print!("{x}, "));
    // 1, 2, 3, 5, 7, 9, 11,
}
