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
