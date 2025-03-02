#[test]
fn test131() {
    let x = 5;
    let y = x;
    println!("{}", y); // 5
    println!("{}", x); // 5
}

#[test]
fn test132_move() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);
    // println!("{}", s1);
}

#[test]
fn test132_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s2);
    println!("{}", s1);
}

#[test]
fn test132_drop() {
    let mut s = String::from("hello");
    s = String::from("ahoy");
    println!("{s}, world!");
}
