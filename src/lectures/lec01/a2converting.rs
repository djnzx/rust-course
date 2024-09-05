#[test]
fn converting() {
    let x: u8 = 5;
    let y: i16 = 10;
    let z = x + y as u8;
}

#[test]
fn converting1() {
    let x = 1234i32;
    let y = 10u8;
    let z = x as u8 + y;

    println!("z = {}", z);
}

#[test]
fn converting2() {
    let x: i32 = -5;
    let y = x as u32;

    println!("x = {:b}", x);
}
