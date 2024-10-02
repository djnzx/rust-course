#[test]
fn break_for() {
    for n in 0.. {
        println!("{}", n);
        if n == 999 {
            break;
        }
    }
}

#[test]
fn break_while() {
    let mut n: i32 = 0;
    while true {
        n += 1;
        if n == 999 {
            break;
        }
    }
}

#[test]
fn break_loop() {
    let mut n: i32 = 0;
    loop {
        n += 1;
        if n == 999 {
            break;
        }
    }
}
