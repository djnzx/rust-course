use std::fmt::Display;

trait Rep<A: Display> {
    fn rep(&self) -> String;
}

impl<A: Display + std::fmt::Binary> Rep<A> for A {
    fn rep(&self) -> String {
        format!("{0:4} {:0>8b}", self)
    }
}

#[test]
fn some_numbers1() {
    [
        -3i8, -2, -1, 0, 1, 2, 3, 4, 124, 125, 126, 127, -128, -127, -126,
    ]
    .iter()
    .for_each(|x| println!("{}", x.rep()));
}

#[test]
fn some_numbers2() {
    [
        0u8, 1, 2, 3, 4, 5, 6, 7, 8, 16, 32, 64, 128, 252, 253, 254, 255,
    ]
    .iter()
    .for_each(|x| println!("{}", x.rep()));
}
