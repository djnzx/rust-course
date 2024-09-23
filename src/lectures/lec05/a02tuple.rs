#[test]
fn test1_tuple_creation() {
    let x = (1, 3.14, "Alex".to_string(), true);
}

#[test]
fn test1_tuple_access() {
    let x = (1, 3.14, "Alex".to_string(), true);

    let a = x.0;
    let b = x.1;
    let c = x.2;
    let d = x.3;
}

#[test]
fn test1_tuple_destructurization() {
    // we can create a tuple
    let x = (1, 3.14, "Alex".to_string(), true);

    let (a, b, c, d) = x;
}
