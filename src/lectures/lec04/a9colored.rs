use colored::Colorize;

// https://en.wikipedia.org/wiki/ANSI_escape_code
#[test]
fn colored_output() {
    println!("\x1b[31mThis is red text\x1b[0m");

    // Green text
    println!("\x1b[32mThis is green text\x1b[0m");

    // Blue text
    println!("\x1b[34mThis is blue text\x1b[0m");

    // Bold yellow text
    println!("\x1b[1;33mThis is bold yellow text\x1b[0m");

    // Reset text formatting
    println!("This is normal text");
}

#[test]
fn colored_output_dsl() {
    use colored::*;
    control::set_override(true);

    println!("{}", "This is red".red());
    println!("{}", "And this is blue".blue());
    println!("{}", "This is yellow and striked".yellow().strikethrough());
    println!("{}", "This is green with underline".green().underline());
}
