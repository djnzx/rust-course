#[test]
fn nested_loops() {
    let mut x = 0;
    let mut y = 0;
    loop {
        loop {
            x += 1;
            y += 1;
            print!("{} ", x);
            if x == 5 {
                break;
            }
        }
        loop {
            x -= 1;
            y += 1;
            print!("{} ", x);
            if x == 0 {
                break;
            }
        }
        if y >= 20 {
            break;
        }
    }
}
