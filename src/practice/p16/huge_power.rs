// the task is:
// to calculate the last digit of: 1234567^1000000000000000000

fn last_digit_bruteforce1(power: u64) -> u8 {
    let x: u32 = 137;
    let mut ld = x;

    for _ in 1..power {
        ld = ld * x;
    }
    (ld % 10) as u8
}

fn last_digit_bruteforce2(power: u64) -> u8 {
    match power {
        0 => 1,
        power => {
            let x: u32 = 137;
            let mut ld = x;

            for _ in 1..power {
                ld = (ld * x) % 10;
            }
            (ld % 10) as u8
        }
    }
}

#[ignore]
#[test]
fn test_bruteforce() {
    /// never finishes ;)
    let ld = last_digit_bruteforce2(1000000000000000000u64);
    println!("last digit bruteforce way: {}", ld);
}

#[test]
fn playground_bruteforce() {
    (0..=12)
        .into_iter()
        .for_each(|x| println!("{:2} {}", x, last_digit_bruteforce2(x)));
    //  0 1
    //  1 7
    //  2 9
    //  3 3

    //  4 1
    //  5 7
    //  6 9
    //  7 3

    //  8 1
    //  9 7
    // 10 9
    // 11 3

    // ....
}

fn last_digit_smart(power: u64) -> i32 {
    match power % 4 {
        1 => 7,
        2 => 9,
        3 => 3,
        _ => 1,
    }
}

#[test]
fn last_digit_smart_test() {
    [(0, 1), (1, 7), (2, 9), (3, 3), (4, 1), (5, 7), (6, 9), (3, 3), (1000000000000000000u64, 1)]
        .iter()
        .for_each(|(n, last)| assert_eq!(last_digit_smart(*n), *last));
}

#[test]
fn playground_smart() {
    (0..=12)
        .into_iter()
        .for_each(|x| println!("{:2} {}", x, last_digit_smart(x)));
}
