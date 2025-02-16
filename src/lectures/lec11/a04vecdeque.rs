use std::collections::VecDeque;

#[test]
fn test1() {
    let mut q = VecDeque::new();

    q.push_front(1); // O(1)
    println!("{:?}", q); // [1]

    q.push_front(2); // O(1)
    println!("{:?}", q); // [2, 1]

    q.push_back(10); // O(1)
    println!("{:?}", q); // [2, 1, 10]

    q.push_back(20); // O(1)
    println!("{:?}", q); // [2, 1, 10, 20]

    let x = q.pop_front(); // O(1)
    println!("{:?}", q); // [1, 10, 20]

    let x = q.pop_back(); // O(1)
    println!("{:?}", q); // [1, 10]

    q.insert(1, 33); // O(N)
    println!("{:?}", q); // [1, 33, 10]
    q.remove(1); // O(N)
    println!("{:?}", q); // [1, 10]

    // let x = q[10]; // O(1)
}
