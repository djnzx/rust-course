#[test]
fn test20() {
    const W: u32 = 10;
    const H: u32 = 10;

    for y in 0..H {
        for x in 0..W {
            let c = if y == 0 || y == H - 1 || x == 0 || x == W - 1 || x == y || x == H - 1 - y {
                '*'
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!();
    }
}

#[test]
fn test21() {
    const W: u32 = 10;
    const H: u32 = 10;

    for y in 0..H {
        for x in 0..W {
            let is_horizontal = y == 0 || y == H - 1;
            let is_vertical = x == 0 || x == W - 1;
            let id_diagonal = x == y;
            let id_co_diagonal = x == W - 1 - y;

            let c = if is_horizontal || is_vertical || id_diagonal || id_co_diagonal {
                '*'
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!();
    }
}

#[test]
fn test22() {
    const W: u32 = 25;
    const H: u32 = 10;
    let k = W / H; // 2

    for y in 0..H {
        for x in 0..W {
            let is_horizontal = y == 0 || y == H - 1;
            let is_vertical = x == 0 || x == W - 1;
            let id_diagonal = x == y * k;
            let id_co_diagonal = x == W - 1 - y * k;

            let c = if is_horizontal || is_vertical || id_diagonal || id_co_diagonal {
                '*'
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!();
    }
}

#[test]
fn test23() {
    const W: u32 = 25;
    const H: u32 = 10;
    let k = W as f32 / H as f32; // 2.5

    for y in 0..H {
        for x in 0..W {
            let is_horizontal = y == 0 || y == H - 1;
            let is_vertical = x == 0 || x == W - 1;
            let id_diagonal = x == (y as f32 * k) as u32;
            let id_co_diagonal = x == W - 1 - (y as f32 * k) as u32;

            let c = if is_horizontal || is_vertical || id_diagonal || id_co_diagonal {
                '*'
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!();
    }
}

#[test]
fn test24() {
    const W: u32 = 25;
    const H: u32 = 10;
    let k = W as f32 / H as f32; // 2.5

    fn mul(a: u32, b: f32) -> u32 {
        (a as f32 * b) as u32
    }

    for y in 1..=H {
        for x in 1..=W {
            let c: char = match (x, y) {
                (_, 1) => '-',
                (_, H) => '-',
                (1, _) => '|',
                (W, _) => '|',
                _ if x == mul(y, k) => '\\',
                _ if x == W - mul(y, k) => '/',
                _ => ' ',
            };
            print!("{}", c);
        }
        println!();
    }
}

#[test]
fn test25() {
    const W: u32 = 25;
    const H: u32 = 10;
    let k = W as f32 / H as f32; // 2.5

    fn mul(a: u32, b: f32) -> u32 {
        (a as f32 * b) as u32
    }

    for y in 1..=H {
        for x in 1..=W {
            let c: char = match (x, y) {
                (_, 1 | H) => '-',
                (1 | W, _) => '|',
                _ if x == mul(y, k) => '\\',
                _ if x == W - mul(y, k) => '/',
                _ => ' ',
            };
            print!("{}", c);
        }
        println!();
    }
}
