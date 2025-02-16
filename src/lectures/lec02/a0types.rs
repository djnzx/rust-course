#[test]
fn type_unit_sample1() {
    let x: () = println!("hello");
}

#[test]
fn type_unit_sample2() {
    let mut k = 1;
    let x: () = k += 1;
}

#[test]
fn type_unit_sample3() {
    let mut k = 10;

    let x: () = while k > 0 {
        k -= 1;
        println!(".")
    };
}

fn type_never() {
    let x = loop {
        print!(".")
    };
}
