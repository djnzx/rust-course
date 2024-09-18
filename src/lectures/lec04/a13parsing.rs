#[test]
fn parsing() {
    let r = "5".parse::<u32>();
    /// handle explicitly (getOrThrow)
    let int: u32 = r.clone().expect("should be a number");
    /// allow to panic with std message
    let r: u32 = r.unwrap();
}
