#[test]
fn main1() {
    let s1 = String::from("hello");
    let len = calc_length(&s1);
    println!("The length of '{s1}' is {len}.");
}

fn calc_length(s: &String) -> usize {
    s.len()
}

fn main2() {
    let s = String::from("hello");
    change(&s);
}

fn change(s: &String) {
    // s.push_str(", world");
}

fn main3() {
    let mut s = String::from("hello");
    change2(&mut s);
}

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

#[test]
fn two_mut_refs() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;
    //
    // println!("{}, {}", r1, r2);
}

#[test]
fn two_mut_refs_good() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here,
      // so we can make a new reference with no problems.

    let r2 = &mut s;
}

#[test]
fn mut_immut_simultaneously() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM
                 //
                 // println!("{}, {}, and {}", r1, r2, r3);
}

// fn main4() {
//     let nothing = dangle();
// }
//
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

fn main() {
    let owns = makes();
}

fn makes() -> String {
    let s = String::from("hello");
    s
}
