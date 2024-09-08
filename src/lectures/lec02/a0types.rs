#[test]
fn type_unit() {
    let x: () = println!("hello");
}

fn type_never() {
    let x = loop {
        print!(".")
    };
}
