#[test]
fn manual_reference_dereference() {
    let x = 5;

    let px = &x;

    let x2 = *px;
}

#[test]
fn automatic_reference_dereference() {
    let x: i32 = 5;

    let px = &x;

    fn go(px: &i32) -> i32 {
        px.abs()
    }

    let x_abs = px.abs();
}
