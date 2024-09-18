#[test]
fn range_syntax1() {
    let r1 = 1..5; // 1,2,3,4
    let r2 = 1..=5; // 1,2,3,4,5

    r1.for_each(|i| print!("{} ", i));
    println!();
    r2.for_each(|i| print!("{} ", i));
}

#[ignore]
#[test]
fn range_syntax2() {
    let r3 = 3..;

    r3.for_each(|i| print!("{} ", i)); // will hang
}

#[test]
fn range_syntax3() {
    /// do not have iterators
    let r4 = ..5;
    let r5 = ..=5;
}
