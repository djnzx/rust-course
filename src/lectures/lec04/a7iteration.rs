#[test]
fn iteration_chars() {
    let s5 = "Hello".to_string();
    for c in s5.chars() {
        print!("{} ", c);
    }
    // H e l l o
    println!();
}

#[test]
fn iteration_bytes() {
    /// https://www.asciitable.com
    let s5 = "Hello".to_string();
    for c in s5.bytes() {
        print!("{} ", c);
    }
    // 72 101 108 108 111
    println!();
}
