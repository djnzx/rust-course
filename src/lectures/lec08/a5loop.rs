/// infinite
#[ignore]
#[test]
fn loop_syntax() {
    let mut n = 0u32;
    loop {
        n += 1;
        println!("{}", n);
    }
}

#[test]
fn loop_syntax_with_break() {
    let mut n = 0u32;
    loop {
        n += 1;
        println!("{}", n);
        if n == 10 {
            break;
        }
    }
}

#[ignore]
#[test]
fn loop_syntax_with_break_and_continue() {
    let mut n = 0u32;
    loop {
        // ...
        // ...
        n += 1;
        println!("{}", n);
        if n % 2 == 0 {
            continue;
        }
        // ...
        // ...
        if n == 100 {
            break;
        }
        // ...
        // ...
    }
}

#[test]
fn loop_syntax_with_labels() {
    let mut n = 0u32;

    'a: loop {
        n += 1;

        println!("b");

        if n == 100 {
            break;
        };
        continue 'a;
    }
}

#[test]
fn loop_old_usage() {
    let mut total = 1;

    loop {
        total *= 2;
        if total > 1000 {
            break;
        }
    }
    assert_eq!(1024, total);
}

#[test]
fn loop_returns() {
    let mut total = 1;

    let outcome = loop {
        total *= 2;
        if total > 1000 {
            break total;
        }
    };
    assert_eq!(1024, outcome);
}
