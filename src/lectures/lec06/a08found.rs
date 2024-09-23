enum SearchResult {
    Found(usize),
    NotFound,
}

fn index_of(x: i32, xs: &i32) -> usize {
    todo!()
}

fn index_of2(needle: i32, xs: &[i32]) -> SearchResult {
    match xs.iter().enumerate().find(|(idx, x)| **x == needle) {
        Some((at, _)) => SearchResult::Found(at),
        None => SearchResult::NotFound,
    }
}

fn find_and_report(needle: i32, xs: &[i32]) {
    let x = index_of2(needle, &xs);
    match x {
        SearchResult::Found(at) => println!("element {needle} fount at index {at}"),
        SearchResult::NotFound => println!("element {needle} not found"),
    }
}

#[test]
fn test1() {
    //                             0  1  2
    find_and_report(1, &[3, 2, 1, 5, 6]); // Found(2)
    find_and_report(7, &[3, 2, 1, 5, 6]); // NotFound
}
