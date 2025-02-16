use std::collections::HashSet;

// for_each / collect
#[test]
fn test1() {
    let xs = [1, 2, 3, 3, 2, 1];
    let ys = xs
        .iter()
        .collect::<HashSet<_>>();

    println!("{:?}", ys);
    // {1, 2, 3}
}

#[test]
fn test2() {
    [1, 2, 3, 3, 2, 1]
        .iter()
        .collect::<HashSet<_>>()
        .iter()
        .for_each(|x| println!("{:?}", x));
}
