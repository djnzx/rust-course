// map
// filter
// flat_map
// take
// take_while
// skip
// skip_while

#[test]
fn test1() {
    let xs = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        .iter()
        .filter(|&x| x % 3 == 0)
        .collect::<Vec<_>>();

    println!("{:?}", xs);
    // [3, 6, 9]
}

#[test]
fn test11() {
    let xs = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        .iter()
        .filter(|&x| *x < 5)
        .collect::<Vec<_>>();

    println!("{:?}", xs);
    // [1, 2, 3, 4]
}

#[test]
fn test2() {
    let xs = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        .iter()
        .map(|&x| x * 10)
        .collect::<Vec<_>>();

    println!("{:?}", xs);
    // [10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
}

#[test]
fn test3() {
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        .iter()
        .map(|&x| x * 10)
        .for_each(|x| print!("{} ", x));
}

#[test]
fn test4() {
    [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        .iter()
        .skip(3)
        .take(2)
        .for_each(|x| print!("{} ", x));
    // 4 5
}

#[test]
fn test5() {
    [1, 1, 2, 1, 3, 4, 5, 6, 7, 8, 9, 10]
        .iter()
        .skip_while(|&x| *x < 5)
        .for_each(|x| print!("{} ", x));
    // 5 6 7 8 9 10
}

#[test]
fn test6() {
    [1, 1, 2, 1, 3, 4, 5, 6, 7, 8, 9, 10]
        .iter()
        .skip_while(|&x| *x < 5)
        .take_while(|&x| *x < 10)
        .for_each(|x| print!("{} ", x));
    // 5 6 7 8 9
}
