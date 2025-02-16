trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

enum Option<A> {
    Some(A),
    None,
}

// fn a() {
//     let x = null;
// }
