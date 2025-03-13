// https://www.hackerrank.com/challenges/apple-and-orange/problem

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
