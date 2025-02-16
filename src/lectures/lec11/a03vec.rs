#[test]
fn test1() {
    let mut ns = Vec::with_capacity(3);
    ns.push(1); // O(1)
    println!("{:?}", ns); // [1]

    ns.push(2); // O(1)
    println!("{:?}", ns); // [1, 2]

    ns.push(3); // O(1)
    println!("{:?}", ns); // [1, 2, 3]

    ns.push(4); // O(4)
    println!("{:?}", ns); // [1, 2, 3, 4]

    let x = ns[3]; // O(1)

    ns[0] = 10; // O(1)
    println!("{:?}", ns); // [10, 2, 3, 4]
}
