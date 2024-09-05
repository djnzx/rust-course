#[test]
fn literals() {
    // decimal
    let x = 123;
    println!("{}", x); // 123

    // hexadecimal
    let x = 0xF0; // 240
    println!("{}", x);

    // octal
    let x = 0o71; // 57
    println!("{}", x);

    // binary
    let x = 0b101011;
    println!("{}", x) // 43
}

#[test]
fn extend_narrow() {
    let x = 1234u16;
    let y = x as u32;
    let z = x as u8;

    assert_eq!(210, z); // 1234-256-256-256-256

    println!("{} {} {}", x, y, z);
}
