use std::fs::File;
use std::io::{BufWriter, Error, Write};

#[test]
fn test1_write_text_file1() -> std::io::Result<()> {
    let mut file = File::create("output.txt");
    match file {
        Ok(mut file) => {
            let x = writeln!(file, "Це запис у текстовий файл!");
            match x {
                Ok(_) => {
                    println!("file wrote good");
                    Ok(())
                }
                Err(e) => {
                    println!("error during file write {e}");
                    Err(e)
                }
            }
        }
        Err(e) => {
            println!("error during file create {e}");
            Err(e)
        }
    }
}

#[test]
fn test1_write_text_file() -> std::io::Result<()> {
    let mut file = File::create("output.txt")?;
    writeln!(file, "Це запис у текстовий файл!")?;
    Ok(())
}

#[test]
fn test2_write_bin_file() -> std::io::Result<()> {
    let file = File::create("output.bin")?;
    let mut buf = BufWriter::new(file);
    buf.write_all(&[0x42, 0x55, 0x53, 0x59])?;
    Ok(())
}

#[test]
fn test2_write_bin_file1() {
    let file_r = File::create("output.bin");
    match file_r {
        Err(e) => println!("error during file create {e}"),
        Ok(file) => {
            let mut buf = BufWriter::new(file);
            let r = buf.write_all(&[0x42, 0x55, 0x53, 0x59]);
            match r {
                Err(e) => println!("error during file write {e}"),
                Ok(_) => println!("file wrote without errors"),
            }
        }
    }
}
