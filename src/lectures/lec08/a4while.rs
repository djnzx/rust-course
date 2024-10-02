#[test]
fn while_basic_syntax() {
    let mut n = 1;
    while n < 20 {
        if n % 15 == 0 {
            println!("{}: fizzbuzz", n);
        } else if n % 3 == 0 {
            println!("{}: fizz", n);
        } else if n % 5 == 0 {
            println!("{}: buzz", n);
        } else {
            println!("{}", n);
        }
        n += 1;
    }
}
