#[test]
fn edit() {
    /// trim
    let raw = "    Rust    ";
    let trimmed = raw.trim();
    println!("{}", trimmed); // "Rust"

    /// replace
    let mut edited = trimmed.replace("us", "uuuussss");
    println!("{}", edited); // "Ruuuusssst"

    /// insert
    edited.insert(5, '-');
    println!("{}", edited); // "Ruuuu-sssst"
}
