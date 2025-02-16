#[test]
fn test1() {
    let xs = 'a'..;
    let ys = 1..10;

    let zs = xs.zip(ys);

    zs.for_each(|t| println!("t:{:?}", t));
}
