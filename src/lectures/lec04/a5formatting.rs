use colored::Colorize;

#[test]
fn test_1_basic() {
    let x = 123;
    // using Display trait
    let s = format!("{x}");

    assert_eq!("123", s);
}

#[test]
fn test_1_print_debug_information() {
    let x = 123;
    // using Debug trait
    let s = format!("{:?}", x);
    dbg!(s);
}

#[test]
fn test_2_refer_many_times() {
    let x = 123;
    let y = 456;
    let s = format!("{0} {1} {0}", x, y);

    assert_eq!("123 456 123", s);
}

#[test]
fn test_3_align_right() {
    let x = 123;
    let s = format!("{:10}", x);

    assert_eq!("       123", s);
}

#[test]
fn test_3_align_right_custom_fill() {
    let x = 123;
    let s = format!("{:_>10}", x);

    assert_eq!("_______123", s);
}

#[test]
fn test_3_align_left() {
    let x = 123;
    let s = format!("{:<10}", x);

    assert_eq!("123       ", s);
}

#[test]
fn test_3_align_left_custom_fill() {
    let x = 123;
    let s = format!("{:_<10}", x);

    assert_eq!("123_______", s);
}

#[test]
fn test_3_align_center() {
    let x = 123;
    let s = format!("{:^10}", x);

    assert_eq!("   123    ", s);
}

#[test]
fn test_3_align_center_custom_fill() {
    let x = 123;
    let s = format!("{:_^10}", x);

    assert_eq!("___123____", s);
}

#[test]
fn test_4_formatting_doubles_by_default() {
    let x = 1.234567891011121314e5;
    let s = format!("{}", x);

    assert_eq!("123456.78910111212", s);
}

#[test]
fn test_4_formatting_doubles_number_of_decimal_digits() {
    let x = 1.234567891011121314e5;
    let s = format!("{:.2}", x);

    assert_eq!("123456.79", s);
}

#[test]
fn test_4_formatting_doubles_scientific() {
    let x = 123456.7891011121314;
    let s = format!("{:e}", x);

    assert_eq!("1.2345678910111212e5", s);
}

#[test]
fn test_4_formatting_doubles_scientific_with_precision() {
    let x = 123456.7891011121314;
    let s = format!("{:.3e}", x);

    assert_eq!("1.235e5", s);
}

#[test]
fn test_4_formatting_doubles_scientific_with_precision_width() {
    let x = 123456.7891011121314;
    let s = format!("{:20.3e}", x);

    assert_eq!("             1.235e5", s);
}

#[test]
fn test_4_formatting_doubles_scientific_with_precision_width_custom_fill() {
    let x = 123456.7891011121314;
    let s = format!("{:_>20.3e}", x);

    assert_eq!("_____________1.235e5", s);
}

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
fn colored_output2() {
    use colored::*;
    control::set_override(true);

    println!("{} {}", "This is red".red(), "and this is blue".blue());
    println!(
        "{}",
        "This is bold and yellow"
            .yellow()
            .strikethrough()
    );
    println!(
        "{}",
        "This is green with underline"
            .green()
            .underline()
    );
}
