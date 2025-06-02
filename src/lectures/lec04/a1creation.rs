fn add(x: u32, y: u32) -> u32 {
    x + y
}

#[test]
fn test_add() {
    assert_eq!(add(1, 3), 4);
}

#[test]
fn creation_and_conversion() {
    /// creation
    let s1 = "Hello!";
    let s2 = String::from("Hello!");
    let mut s3 = String::new();

    /// conversion
    let s4 = s1.to_string();
    let s5 = s2.as_str();
}

#[test]
fn stack_heap() {
    /// stack
    let data = "initial contents";
    /// move to heap
    let s1 = data.to_string();
    let s2 = String::from(data);
    /// initially in heap
    let s3 = String::from("initial contents");
}
