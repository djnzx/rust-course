// https://www.hackerrank.com/challenges/apple-and-orange/problem

fn countApplesAndOranges1(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let mut apple_count = 0;
    let mut orange_count = 0;

    for da in apples {
        let c = a + da;
        if c >= s && c <= t {
            apple_count += 1;
        }
    }

    for db in oranges {
        let c = b + db;
        if c >= s && c <= t {
            orange_count += 1;
        }
    }

    println!("{}\n{}", apple_count, orange_count);
}

fn count(center: i32, deltas: &[i32], l: i32, r: i32) -> i32 {
    let mut count = 0;
    for da in deltas {
        let c = center + da;
        if c >= l && c <= r {
            count += 1;
        }
    }
    count
}

fn countApplesAndOranges2(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = count(a, apples, s, t);
    let orange_count = count(b, oranges, s, t);

    println!("{}\n{}", apple_count, orange_count);
}
