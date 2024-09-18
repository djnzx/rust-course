#[test]
fn change_case() {
    let s = "Привіт";
    let upper = s.to_uppercase();
    let lower = s.to_lowercase();
    println!("{}", upper); // ПРИВІТ
    println!("{}", lower); // привіт
}
