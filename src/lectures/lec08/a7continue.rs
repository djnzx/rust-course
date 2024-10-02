#[test]
fn continue_syntax() {
    let mut n: i32 = 0;
    for i in 0..100 {
        n = n + 1; // 100 times
        if i % 2 == 0 {
            continue;
        }
        n = n + 1; // 50 times
    }
    assert_eq!(150, n);
}
