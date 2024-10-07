use std::ops::Add;

#[derive(PartialEq, Debug)]
struct Cart {
    numbers: u8,
    total: f32,
}

impl Add for Cart {
    type Output = Cart;

    fn add(self, rhs: Cart) -> Self::Output {
        Cart {
            numbers: self.numbers + rhs.numbers,
            total: self.total + rhs.total,
        }
    }
}

#[test]
fn code() {
    let c1 = Cart {
        numbers: 3,
        total: 14.99,
    };
    let c2 = Cart {
        numbers: 5,
        total: 24.99,
    };
    assert_eq!(
        c1 + c2,
        Cart {
            numbers: 8,
            total: 39.98
        }
    );
}
