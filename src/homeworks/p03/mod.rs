// hw03.rs
pub fn draw_envelope() {
    const W: usize = 30;
    const H: usize = 15;
    
    for y in 0..H {
        let mut line = String::new();
        for x in 0..W {
            let c = if x == 0 || x == W - 1 || y == 0 || y == H - 1 { 
                '*'
            } else if x == y || x == W - y - 1 {
                '*'
            } else if x == H - y - 1 || x == W - H + y { 
                '*'
            } else {
                ' '
            };
            line.push(c);
        }
        println!("{}", line);
    }
}
