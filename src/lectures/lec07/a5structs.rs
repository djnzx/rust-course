struct Pizza {
    name: String,
    size: u8,
}

fn test(p: Pizza) {
    match p {
        Pizza { name, size } if name == "Margarita" => println!("Margarita of size {}", size),
        Pizza { name, size: 30 } => println!("Pizza of size 30 with name: {name}",),
        Pizza { name, size } => println!("pizza name: {}, size: {}", name, size),
    };
}
