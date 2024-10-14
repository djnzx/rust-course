#[test]
fn flat_map1() {
    let xs = 1..;
    let ys = xs
        .filter(|x| x % 2 == 0)
        .skip_while(|&x| x < 100)
        .take(10)
        .flat_map(|x| vec![-x, x])
        .filter(|&x| x % 10 == 0)
        .collect::<Vec<_>>();
    println!("{:?}", ys);
    // [-100, 100, -110, 110]
}
