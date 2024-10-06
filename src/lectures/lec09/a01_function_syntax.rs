use std::ops::Add;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

trait SpaceX<A: Add, B> {
    fn launch(a: A) -> B;
    fn add1(a: i32, b: f64) -> f64;
    fn add2(ab: (i32, f64)) -> f64;
    fn div(a: i32, b: i32) -> (i32, i32);
    fn print(s: String);
    fn get_a_number1(_x: ()) -> i32;
    fn get_a_number2() -> i32;
    fn destroy_universe1() -> ();
    fn destroy_universe2();
}

fn solve(x: i32, y: i32) -> i32 {
    fn solve1(a: i32, b: i32) -> i32 {
        todo!()
    }

    fn solve2(a: i32, b: i32) -> i32 {
        todo!()
    }

    solve1(x, y) + solve2(x, y)
}

fn solve2(x: i32, y: i32) -> i32 {
    fn solve_part(a: i32, b: i32) -> i32 {
        // a + b + x
        todo!()
    }

    todo!()
}

fn returns_unit1(x: i32) -> () {
    println!("hello");
}

fn returns_unit2(x: i32) {
    println!("hello");
}

fn returns_unit3(mut x: i32) {
    x += 1;
}

fn returns_unit4(mut x: i32) {
    let x = for i in 0..10 {
        // whatever
    };
}

fn inc1(x: i32) -> () {
    x + 1;
}

fn inc2(x: i32) {
    x + 1;
}

fn inc(x: i32) -> i32 {
    x + 1
}

fn scope() {
    //
    let x = {
        let a = 1 + 2;
        let b = 3 + 4;
        a + b
    };
    //let c = a + b;
    //
}

fn align_all(data: &[&str], width: u8) -> Vec<String> {
    fn align(s: &str, width: u8) -> String {
        s.to_string() + &*" ".repeat(width as usize - s.len())
    }

    data.iter()
        .map(|x| align(x, width))
        .collect::<Vec<String>>()
}

fn align_all_v2(data: &[&str], width: u8) -> Vec<String> {
    let align = |s: &str| s.to_string() + &*" ".repeat(width as usize - s.len());

    data.iter().map(|x| align(x)).collect::<Vec<String>>()
}

#[test]
fn test2() {
    align_all_v2(&["abc", "ab", "a", "abcdef"], 10)
        .iter()
        .for_each(|x| println!(">{}<", x));
}
// >abc       <
// >ab        <
// >a         <
// >abcdef    <

fn align(s: &str, width: u8) -> String {
    s.to_string() + &*" ".repeat(width as usize - s.len())
}

#[test]
fn test1() {
    let x = align("abc", 10);
    println!("`{}`", x);
}

fn closures_syntax(delta: i32) {
    let with_delta = |x: i32| x + delta;

    let total = with_delta(100);
    println!("delta was: {delta}");
    println!("total (100+{delta}): {total}");
}

#[test]
fn test3() {
    closures_syntax(1);
}
// delta was: 1
// total (100+1): 101
