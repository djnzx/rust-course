/// if/else
/// match
/// for/while/loop
/// continue/break

// 4h24min
#[test]
fn for_basic_not_including_syntax() {
    for n in 0..5 {
        println!("{}", n);
    }
}

#[test]
fn for_basic_including_syntax() {
    for n in 0..=5 {
        println!("{}", n);
    }
}

fn for_infinite() {
    for n in 0.. {
        println!("{}", n);
    }
}
