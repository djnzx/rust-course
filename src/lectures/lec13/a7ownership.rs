fn main() {
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // println!("{}", s); // ... and so is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // because i32 implements the Copy trait,
                   // x does NOT move into the function,

    println!("{}", x); // so it's okay to use x afterward
} // Here, x goes out of scope, then s.
  // But because s's value was moved, nothing special happens.

fn takes_ownership(s1: String) {
    // s1 comes into scope
    println!("{s1}");
} // Here, some_string goes out of scope and `drop` is called.
  // The backing  memory is freed.

fn makes_copy(i1: i32) {
    // i1 comes into scope
    println!("{i1}");
} // Here, i1 goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    let s = String::from("yours");
    // s comes into scope
    s // s is returned and moves out to the calling function
}

fn takes_and_gives_back(s: String) -> String {
    // s comes into scope
    s // s is returned and moves out to the calling function
}
