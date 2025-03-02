use std::io;

// will wait for the input, therefore - disabled
#[ignore]
#[test]
fn test_kbd() {
    print!("Enter your name:");

    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

    println!("You entered: {}", buffer);
}
