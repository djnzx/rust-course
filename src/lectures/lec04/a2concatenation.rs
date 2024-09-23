#[test]
fn concatenation_and_length() {
    let mut s3 = String::from("Hello!");
    let l1 = s3.len();
    println!("{}", l1);

    s3.push(',');
    s3.push_str(" world!");
    let l1 = s3.len();

    println!("{}", s3); // Hello!, world!
    println!("{}", l1);
}

#[test]
fn concatenation() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    /// deref
    let s2a = &s2[..];
    /// fn +(self, s: &str) -> String
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");
}
