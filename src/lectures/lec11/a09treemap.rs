use std::collections::BTreeMap;

#[test]
fn test1() {
    let xs = "learning rust is an interesting process"
        .chars()
        .fold(BTreeMap::new(), |mut m, c| {
            m.entry(c)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            return m;
        });
    println!("{xs:?}")
    // {' ': 5, 'a': 2, 'c': 1, 'e': 4, 'g': 2, 'i': 4, 'l': 1,
    //  'n': 5, 'o': 1, 'p': 1, 'r': 4, 's': 5, 't': 3, 'u': 1}
}
