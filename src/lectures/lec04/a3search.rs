use std::ops::Index;

#[test]
fn search() {
    let s = "Hello!";

    let x = s.contains("ll");
    println!("{}", x); // true
    let x = s.contains("ee");
    println!("{}", x); // false

    let x = s.is_empty();
    println!("{}", x); // false

    let x = s.starts_with("He");
    println!("{}", x); // true
    let x = s.ends_with("!?");
    println!("{}", x); // false
}
