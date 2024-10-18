#[test]
fn change_case() {
    let s = "Привіт";
    let upper = s.to_uppercase();
    let lower = s.to_lowercase();
    println!("{}", upper); // ПРИВІТ
    println!("{}", lower); // привіт
}

#[test]
fn test1() {
    // in some cases when letter becomes lowercase
    // it has more than 1 symbol
    let s = "ΣΣ";
    let c = 'Æ';

    let cl = c.to_lowercase();
    let c2 = cl.collect::<String>();
    println!("{}", s.to_lowercase());
    println!("{}", c2);
}
