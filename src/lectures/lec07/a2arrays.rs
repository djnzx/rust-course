fn arrays01(xs: &[i32]) {
    match xs {
        [] => println!("array is empty"),
        [x] => println!("array has one element: {x}"),
        [x, y] => println!("array has two elements: {x}, {y}"),
        [13, ..] => println!("array starts from number 13: {xs:?}"),
        [7, 13, ..] => println!("array starts from numbers 7, 13: {xs:?}"),
        [.., 11, 53] => println!("array ends with numbers 11, 53 {xs:?}"),
        [.., 53] => println!("array ends with number 53 {xs:?}"),
        [1, .., 11] => println!("array starts from 1 ends with 11 {xs:?}"),
        _ => println!("something different {xs:?}"),
    }
}

#[test]
fn test1() {
    arrays01(&[]);
    arrays01(&[34]);
    arrays01(&[34, 11]);
    arrays01(&[7, 13, 1, 2, 3, 4, 5]);
    arrays01(&[1, 2, 3, 4, 5, 53]);
    arrays01(&[1, 2, 3, 4, 5, 11, 53]);
    arrays01(&[1, 100, 101, 11]);
    arrays01(&[1, 1, 1]);
}
