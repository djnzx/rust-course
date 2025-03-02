#[test]
fn test131() {
    let x = 5;
    let x = x + 1; // shadow
    {
        let x = x * 2; // inner scope
        println!("inner: {x}");
    }
    println!("outer: {x}");
}

#[test]
fn test132() {
    let x = 5; // x = 5
    let x = x + 1; // x = 6, shadow
    {
        let x = x * 2; // x = 12, temporary shadow
        println!("inner: {x}");
    }
    println!("outer: {x}"); // x = 6
}

#[test]
fn test133() {
    {
        // s is not valid here
        let s = "hello"; // s is valid
                         // ...
    } // this scope is now over,
      // and s is no longer valid
}
