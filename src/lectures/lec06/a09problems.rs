fn min1(xs: &[i32]) -> i32 {
    todo!()
}
fn min2(xs: &[i32]) -> Vec<i32> {
    if xs.is_empty() {
        vec![]
    } else {
        vec![*xs.iter().min().unwrap()]
    }
}

fn min(xs: &[i32]) -> (bool, i32) {
    if xs.is_empty() {
        (false, 0)
    } else {
        (true, *xs.iter().min().unwrap())
    }
}

#[test]
fn test2() {
    let found = min2(&[11, 2, 3]);
    println!("{:?}", found); // [2]

    let found = min2(&[]);
    println!("{:?}", found); // []
}

#[test]
fn test3() {
    let found = min(&[11, 2, 3]);
    println!("{:?}", found); // (true, 2)

    let found = min(&[]);
    println!("{:?}", found); // (false, 0)
}
