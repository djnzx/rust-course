// homeworks/hw06.rs
pub fn draw_christmas_tree(sections: usize) {
    let max_width = 2 * sections + 1;
    
    (0..sections).flat_map(|section| {
        (0..=section).map(move |triangle| {
            (0..=triangle).map(move |row| {
                let stars = 2 * row + 1;
                let spaces = (max_width - stars) / 2;
                format!("{:^1$}", "*".repeat(stars), max_width)
            })
        })
    })
    .flatten()
    .for_each(|line| println!("{}", line));
    
    // Малюємо стовбур
    let trunk_spaces = (max_width - 1) / 2;
    println!("{:^1$}", "*", max_width);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        draw_christmas_tree(3);
    }
}
