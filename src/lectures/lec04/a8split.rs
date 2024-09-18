#[test]
fn split_at() {
    /// split by index
    let word = "programming";
    let (l, r) = word.split_at(7);
    assert_eq!("program", l);
    assert_eq!("ming", r);
}
#[test]
fn split_whitespaces() {
    /// split by whitespaces
    let sentence = "Раст це цікава мова програмування.";
    sentence.split_whitespace().for_each(|w| println!("{}", w));
    // Раст
    // це
    // цікава
    // мова
    // програмування
}
