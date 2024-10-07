use std::ops::Add;
use std::ops::Rem;

struct Electric;
struct Car;

#[derive(PartialEq, Debug)]
struct Tesla;

impl Add<Electric> for Car {
    type Output = Tesla;

    fn add(self, rhs: Electric) -> Self::Output {
        Tesla
    }
}

#[test]
fn code() {
    assert_eq!(Tesla, Car + Electric);
}
