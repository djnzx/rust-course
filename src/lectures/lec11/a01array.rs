#[test]
fn test1() {
    let xs = [11, 13, 15];
    // length
    println!("len = {}", xs.len()); // 3
                                    // iterator
    for x in xs {
        println!("x = {}", x);
    }
}
