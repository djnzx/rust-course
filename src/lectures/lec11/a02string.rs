#[test]
fn test1() {
    let mut text = String::from("Hello");
    println!("{}", text); // Hello
    text.push('!');
    println!("{}", text); // Hello!
    text.remove(3);
    println!("{}", text); // Helo!
}

#[test]
fn test2() {
    let mut text = String::with_capacity(3);
    println!("{}", text); // ""

    text.push_str("He"); // O(2)
    println!("{}", text); // "He"

    text.push_str("l"); // O(1)
    println!("{}", text); // "Hel"

    text.push_str("l"); // O(4)
    println!("{}", text); // "Hell"
}
