fn problem01(x: i32) {
    if x == 1 {
        println!("one")
    } else if x == 2 {
        println!("two")
    } else if x == -2 {
        println!("minus two")
    }
}

fn solution01(x: i32) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        -2 => println!("minus two"),
        _ => println!("Something else!"),
    }
}

fn solution02(x: bool) {
    match x {
        true => println!("t"),
        false => println!("f"),
    }
}

fn solution03(number: i32) {
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime number!"),
        _ => println!("Something else!"),
    }
}

fn solution04(number: i32) {
    match number {
        1 => println!("One!"),
        10..=19 => println!("A 10+ number!"),
        _ => println!("Something else!"),
    }
}

fn solution05(number: i32) {
    match number {
        1 => println!("One!"),
        _ => println!("Something else!"),
    }
}
