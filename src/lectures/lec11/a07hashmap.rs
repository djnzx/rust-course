use std::collections::HashMap;

#[test]
fn test1() {
    let mut m = HashMap::new();
    m.insert("hello", 1);
    m.insert("to", 20);
    println!("{m:?}"); // {"to": 20, "hello": 1}

    assert_eq!(m.get(&"hello"), Some(&1));
    assert_eq!(m.contains_key("hello"), true);
    assert_eq!(m.len(), 2);
}

#[test]
fn test2() {
    let mut m = HashMap::new();

    m.entry("to")
        .and_modify(|v| *v += 1)
        .or_insert(1);
}

#[test]
fn test3() {
    let xs = "learning rust is an interesting process"
        .chars()
        .fold(HashMap::new(), |mut m, c| {
            m.entry(c)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            return m;
        });
    println!("{xs:?}")
    // {'c': 1, 'a': 2, 'p': 1, 'i': 4, 'n': 5, 'r': 4, 'e': 4,
    //  'l': 1, ' ': 5, 's': 5, 't': 3, 'o': 1, 'u': 1, 'g': 2}
}
