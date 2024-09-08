#[test]
fn creation() {
    let s1 = "Hello!";
    let s2 = String::new();
    let s2 = String::from("Hello!");

    let mut s3 = String::from("Hello!");
    s3.push(',');
    s3.push_str(" world!");
    println!("{}", s3); // Hello!, world!
}
