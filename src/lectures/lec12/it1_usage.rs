use std::slice::Iter;

#[test]
fn test1() {
    let xs = [1, 2, 3];

    for x in xs {
        println!("{}", x);
    }
}

#[test]
fn test() {
    let xs = [1, 2, 3];
    let it: Iter<i32> = xs.iter();

    for x in it {
        println!("{}", x);
    }
}
