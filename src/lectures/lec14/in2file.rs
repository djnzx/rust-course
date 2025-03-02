use std::fs::File;
use std::io;
use std::io::BufRead;

#[test]
fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("Read: {}", line);
    }

    Ok(())
}
