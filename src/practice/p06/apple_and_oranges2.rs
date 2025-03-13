// https://www.hackerrank.com/challenges/apple-and-orange/problem

fn countApplesAndOranges2(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = count(a, apples, s, t);
    let orange_count = count(b, oranges, s, t);

    println!("{}\n{}", apple_count, orange_count);
}

fn countApplesAndOranges1(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let mut apple_count = 0;
    for delta in apples {
        let absolute = a + delta;
        if absolute >= s && absolute <= t {
            apple_count += 1;
        }
    }

    let mut orange_count = 0;
    for delta in oranges {
        let absolute = b + delta;
        if absolute >= s && absolute <= t {
            orange_count += 1;
        }
    }

    println!("{}\n{}", apple_count, orange_count)
}

fn count2(center: i32, deltas: &[i32], l: i32, r: i32) -> usize {
    let mut count: usize = 0;
    for delta in deltas {
        let absolute = center + delta;
        if absolute >= l && absolute <= r {
            count += 1;
        }
    }
    count
}

fn adjust(center: i32, delta: i32) -> i32 {
    center + delta
}

fn count(center: i32, deltas: &[i32], l: i32, r: i32) -> usize {
    deltas
        .iter()
        .map(|d| center + d)
        // .map(|&d| adjust(center, d))
        .filter(|&c| c >= l && c <= r)
        .count()
}

fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = count(a, apples, s, t);
    let orange_count = count(b, oranges, s, t);

    println!("{}\n{}", apple_count, orange_count)
}
