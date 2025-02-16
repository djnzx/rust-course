use std::iter::{zip, Map};
use std::slice::Iter;

#[test]
fn basics1() {
    let v1 = vec![1, 2, 3];
    for x in &v1 {
        print!("{} ", x)
    }
}

#[test]
fn basics2() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for x in v1_iter {
        print!("{} ", x)
    }
}

#[test]
fn iterator_is_not_reusable() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for x in v1_iter {
        print!("{} ", x)
    }
    // for x in v1_iter {}
}

#[test]
fn iterator_next() {
    let v1 = vec![1, 2, 3];
    // let x: Iterator
    // to use .next manually it must be mutable
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    // We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on
    assert_eq!(total, 6);
}

#[test]
fn another_iterator() {
    let v1 = vec![1, 2, 3];
    // looking on the signature, we can say, it just holds initial iterator + functions applied
    let it2 = v1
        .iter()
        .map(|x| x + 1);

    for x in it2 {
        println!("{}", x);
    }
}
#[test]
fn consuming_explicitly() {
    let v1 = vec![1, 2, 3];
    let it2 = v1
        .iter()
        .map(|x| x + 1)
        .collect::<Vec<i32>>();
    // syntax to infer initial type
    let it3 = v1
        .iter()
        .map(|x| x + 1)
        .collect::<Vec<_>>();

    for x in it2 {
        println!("{}", x);
    }
}

#[test]
fn iterator2() {
    let v1 = vec![1, 2, 3, 4, 5];

    let mut v1_iter = v1.iter();

    let mut cnt = 0;

    loop {
        match v1_iter.next() {
            Some(_) => {
                cnt += 1;
            }
            None => {
                break;
            }
        }
    }

    assert_eq!(cnt, v1.len());
}

#[test]
fn iterator3() {
    let v1 = vec![1, 2, 3, 4, 5];

    let mut v1_iter = v1.iter();
    let sum: i32 = v1_iter.sum();

    dbg!(sum);
    assert_eq!(sum, 15);
}

#[test]
fn iterator4() {
    let v1 = vec![1, 2, 3, 4, 5];
    let it2: Map<Iter<i32>, fn(&i32) -> i32> = v1
        .iter()
        .map(|x| x + 1);
    let collected: Vec<i32> = it2.collect();
    // dbg!(collected);
    assert_eq!(collected, vec![2, 3, 4, 5, 6]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// predicate takes reference
fn predicate(s: &Shoe, size: u32) -> bool {
    s.size == size
}

fn shoes_filtered_by_size_2(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // method reference
    shoes
        .into_iter()
        .filter(|s| predicate(s, shoe_size)) // no currying, explicit lambda here
        .collect()
}

fn shoes_filtered_by_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // TODO: .iter() vs .into_iter()
    shoes
        // .iter()
        .into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_filtered_by_size(shoes, 10);

    assert!(in_my_size
        .iter()
        .all(|s| s.size == 10));
}

#[test]
fn test1() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let all = shoes
        .iter()
        .all(|s| s.size >= 10);

    let any = shoes // exists
        .iter()
        .any(|s| s.size == 10);
}

#[test]
fn test_iter_vs_into_iter() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    // borrow and return: works with references &T
    let xs = shoes.iter();
    // borrow and return
    let xs = shoes.iter();
    // consume and does'nt return: works with data T
    let xs = shoes.into_iter();
    // will not compile
    // let xs = shoes.iter();
}

#[test]
fn test10() {
    let ns = 1..10;
    let ls = 'a'..;

    let xs = ns.zip(ls);
    // let qs = xs.unzip();
    // xs.for_each(|(x, l)| println!("{}: {}", x, l));

    // let xs = ns.into_iter().collect::<Vec<i32>>();
}
