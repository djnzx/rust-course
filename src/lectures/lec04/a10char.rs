#[test]
fn test1() {
    let c1 = 'z';
    let c2 = 'я';
    let c3 = '你';
    let c4 = '😀';

    println!("{}", size_of_val(&c1)); // 4
    println!("{}", size_of_val(&c2)); // 4
    println!("{}", size_of_val(&c3)); // 4
    println!("{}", size_of_val(&c4)); // 4
}
