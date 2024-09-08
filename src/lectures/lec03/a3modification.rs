#[test]
fn modification() {
    let xs = [10, 20, 30, 40, 50];

    //xs[0] = 5; // will not compile

    let mut xs = [10, 20, 30, 40, 50];

    xs[0] = 5;
    println!("{:?}", xs);
}
