#[test]
fn access_by_index() {
    let xs = [10, 20, 30, 40, 50];

    let k = xs[0];
    dbg!(k); // 10

    // let k = xs[5]; // will not compile
}

#[test]
fn access_by_name() {
    let xs = [10, 20, 30, 40, 50];
    let x = xs.first();
    let y = xs.last();
    println!("{:?}", x);
    println!("{:?}", y);

    let xs: [u32; 0] = [];
    let x = xs.first();
    let y = xs.last();
    println!("{:?}", x);
    println!("{:?}", y);
}

#[test]
fn split_at() {
    let xs = [10, 13, 17, 25, 37, 51, 60];
    let (l, r) = xs.split_at(3);

    println!("{:?}", l); // [10, 13, 17]
    println!("{:?}", r); // [25, 37, 51, 60]
}

#[test]
fn split_by_chunks() {
    let xs = [10, 13, 17, 25, 37, 51, 60];

    let k = xs.chunks(2);
    k.for_each(|xx| println!("{:?}", xx));
}

#[test]
fn access_to_chunk() {
    let xs = [10, 20, 30, 40, 50];
    println!("{:?}", xs.len()); // 5

    let z = xs.last_chunk::<3>();
    println!("{:?}", z); // Some([30, 40, 50])

    let z = xs.last_chunk::<6>();
    println!("{:?}", z); // None
}

#[test]
fn contains() {
    let qs = ["a", "b", "c", "d"];
    let c = qs.contains(&"a");
    println!("{:?}", c); // true
    let c = qs.contains(&"z");
    println!("{:?}", c); // false
}

#[test]
fn functions() {
    let xs = [10, 13, 17, 25, 37, 51, 60];

    let qs = ["a", "b", "c", "d"];
    let z = qs.join("_");
    println!("{:?}", z); // "a_b_c_d"
}
