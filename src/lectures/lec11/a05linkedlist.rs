use std::collections::LinkedList;

#[test]
fn test1() {
    // https://doc.rust-lang.org/std/collections/struct.LinkedList.html
    let mut ll = LinkedList::new();
    ll.push_back('a');
    ll.push_back('b');
    ll.push_front('x');
    ll.push_front('y');
    println!("{:?}", ll);
    // still unstable
    // https://github.com/rust-lang/rust/issues/58533
    // let cursor = ll.cursor_front();
}
