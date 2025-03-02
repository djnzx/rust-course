use std::io;

#[test]
fn test_kbd() {
    print!("Enter your name:");

    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    println!("You entered: {}", buffer);
}
