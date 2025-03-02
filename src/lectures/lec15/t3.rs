fn f1(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_11a() {
    let a = 3;
    let b = 5;
    let real = f1(a, b);
    let expected = 8;

    assert_eq!(real, expected);
}

#[ignore]
#[test]
fn test_11b() {
    let a = 3;
    let b = 5;
    let real = f1(a, b);
    let expected = 9;

    assert_eq!(real, expected);
}

fn div(a: i32, b: i32) -> i32 {
    a / b
}

#[test]
#[should_panic]
fn test_div() {
    let x = div(3, 0);
}

#[ignore]
#[test]
fn test_3() {}

#[test]
fn test_2_addition() {
    let sum = 2 + 2;
    assert_eq!(sum, 4);
}

trait Q<A, B, C, D, E> {
    fn f1(a: A) -> B {
        todo!()
    }
    fn f2(a: B) -> C {
        todo!()
    }
    fn f3(a: C) -> D {
        todo!()
    }
    fn f4(a: D) -> E {
        todo!()
    }

    fn app(a: A) -> E {
        todo!();
        // let b: B = f1(a);
        // let c: B = f2(b);
        // let d: B = f3(c);
        // let e: B = f4(d);
        // e
    }
}
