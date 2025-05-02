// homeworks/hw04.rs
pub fn draw_diamond() {
    const WIDTH: usize = 11; 
    const HEIGHT: usize = 11; 
    
    let half_height = HEIGHT / 2;
    
    for y in 0..HEIGHT {
        let mut line = String::new();
        let distance = if y <= half_height {
            y
        } else {
            HEIGHT - y - 1
        };
        
        for x in 0..WIDTH {
            if x >= half_height - distance && x <= half_height + distance {
                line.push('*');
            } else {
                line.push(' ');
            }
        }
        println!("{}", line);
    }
}
