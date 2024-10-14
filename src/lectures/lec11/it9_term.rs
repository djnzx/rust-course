// sum
// count
// all
// any
// ...

#[test]
fn count1() {
    let xs = vec![1, 2, 3, 3, 7, 3, 6];
    let x = xs.iter().count();
    println!("{:?}", x);
    // 7
}

#[test]
fn count2() {
    let xs = vec![1, 2, 3, 3, 7, 3, 6];
    let x = xs
        .iter()
        .sum::<i32>();
    println!("{:?}", x);
    // 7
}

#[test]
fn all1() {
    let xs = vec![1, 2, 3, 3, 7, 3, 6];
    let x = xs
        .iter()
        .all(|&x| x > 0);
    println!("{:?}", x);
    // true
    let x = xs
        .iter()
        .all(|&x| x < 7);
    println!("{:?}", x);
    // false
}

#[test]
fn any1() {
    let xs = vec![1, 2, 3, 3, 7, 3, 6];
    let x = xs
        .iter()
        .any(|&x| x < 2);
    println!("{:?}", x);
    // true
    let x = xs
        .iter()
        .all(|&x| x < 0);
    println!("{:?}", x);
    // false
}
