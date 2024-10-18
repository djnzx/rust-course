#[test]
fn chessboard1() {
    const SIZE: u8 = 8;

    for y in 0..SIZE {
        for x in 0..SIZE {
            if y % 2 == 0 {
                if x % 2 == 0 {
                    print!("**");
                } else {
                    print!("  ");
                }
            } else {
                if x % 2 != 0 {
                    print!("**");
                } else {
                    print!("  ");
                }
            }
        }
        println!();
    }
}

#[test]
fn chessboard2() {
    const SIZE: u8 = 8;

    for y in 0..SIZE {
        for x in 0..SIZE {
            let c = if y % 2 == 0 {
                if x % 2 == 0 {
                    "**"
                } else {
                    "  "
                }
            } else {
                if x % 2 != 0 {
                    "**"
                } else {
                    "  "
                }
            };
            print!("{c}");
        }
        println!();
    }
}

#[test]
fn chessboard3() {
    const SIZE: u8 = 8;

    for y in 0..SIZE {
        for x in 0..SIZE {
            let c = if (x + y) % 2 == 0 { "**" } else { "  " };
            print!("{c}");
        }
        println!();
    }
}

#[test]
fn chessboard4() {
    const SIZE: u8 = 8;

    for y in 0..SIZE {
        for x in 0..SIZE {
            let s = match x + y {
                sum if sum % 2 == 0 => "**",
                _ => "  ",
            };
            print!("{s}");
        }
        println!();
    }
}
