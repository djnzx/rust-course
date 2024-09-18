#[test]
fn slicing() {
    let s = String::from("hello world");
    //                             012345678910
    /// slicing doesn't take a memory
    let hello = &s[0..5]; // "hello"

    let hello2 = &s[..5]; // "hello"

    let hello3 = &s[..=6]; // "hello w"

    let world = &s[6..11]; // "world"

    let world2 = &s[6..]; // "world"

    let whole = &s[..]; // "hello world"

    println!("{}", hello);
    println!("{}", hello2);
    println!("{}", hello3);
    println!("{}", world);
    println!("{}", world2);
    println!("{}", whole);
}
