use colored::*;

#[test]
fn test1() {
    control::set_override(true);

    println!("x = 5");
    println!("{}", "error".red());
}

#[test]
fn test2() {
    /// reuse indexes
    ///                                               0       1
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
}
