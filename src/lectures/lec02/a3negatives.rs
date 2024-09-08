trait Rep {
    fn rep(&self) -> String;
}

impl Rep for i8 {
    fn rep(&self) -> String {
        format!("{0:4} {:0>8b}", self)
    }
}

#[test]
fn some_numbers() {
    println!("  -3 {:0>8b}", -3i8);
    println!("  -2 {:0>8b}", -2i8);
    println!("  -1 {:0>8b}", -1i8);
    println!("   0 {:0>8b}", 0i8);
    println!("   1 {:0>8b}", 1i8);
    println!("   2 {:0>8b}", 2i8);
    println!("   3 {:0>8b}", 3i8);
    println!("   4 {:0>8b}", 4i8);
    println!(" ...........");
    println!(" 124 {:0>8b}", 124i8);
    println!(" 125 {:0>8b}", 125i8);
    println!(" 126 {:0>8b}", 126i8);
    println!(" 127 {:0>8b}", 127i8);
    println!("-128 {:0>8b}", -128i8);
    println!("-127 {:0>8b}", -127i8);
    println!("-126 {:0>8b}", -126i8);
    println!("-125 {:0>8b}", -125i8);
}
#[test]

fn some_numbers2() {
    [-5i8, 7i8, 12i8]
        .iter()
        .for_each(|x| println!("{}", x.rep()));
}
