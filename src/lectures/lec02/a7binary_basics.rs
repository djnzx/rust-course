#[test]
fn test_2_binary_rep() {
    let x = 5_u8;
    println!("{0:2} {0:0>8b}", x);
    let y = 7_u8;
    println!("{0:2} {0:0>8b}", y);
    let z = x + y;
    println!("{0:2} {0:0>8b}", z);
}

#[test]
fn test_2_hex_rep() {
    let x = 63_u8;
    println!("0x{:0>2x}", x);
    let x = 63_u8;
    println!("0x{:0>2X}", x);
}

fn show_detailed(x: i8, y: i8) {
    println!("{0:3} {0:0>8b}", x);
    println!("{0:3} {0:0>8b}", y);
    let z: i8 = x + y;
    println!("{:-<12}", "");
    println!("{0:3} {0:0>8b}", z);
    println!();
}

#[test]
fn test_2_binary_rep_negative() {
    show_detailed(5, 7);
    show_detailed(12, -5);
    show_detailed(-5, -7);
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
#[test]
fn test_4_string_type_name() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
}
