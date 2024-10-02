/// if/else
/// match
/// for/while/loop
/// continue/break

fn basic_flow() {
    fn f(a: u32) -> f32 {
        todo!()
    }
    fn g(a: f32) -> bool {
        todo!()
    }
    fn h(a: bool) -> String {
        todo!()
    }

    fn app(a: u32) -> String {
        let b = f(a);
        let c = g(b);
        let d = h(c);
        d
    }

    let n: i32 = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else {
        println!("{} is non-negative", n);
    }
}

#[test]
fn if_basic_syntax() {
    let n: i32 = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else {
        println!("{} is non-negative", n);
    }
}

#[test]
fn if_stacked_syntax() {
    let n: i32 = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}

#[test]
fn if_returns_syntax() {
    let n: i32 = 5;

    let s = if n < 0 {
        "negative"
    } else if n > 0 {
        "positive"
    } else {
        "zero"
    };
}
