#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

#[test]
fn flatmap_playground1() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let xs = data
        .iter()
        .flat_map(|x| if x % 2 == 0 { Some(x) } else { None })
        .collect::<Vec<_>>();
    println!("{:?}", data);
    println!("{:?}", xs);
}

#[test]
fn flatmap_playground2() {
    let data = [Point { x: 1, y: 2 }, Point { x: 3, y: 4 }];
    let xs = data
        .iter()
        .flat_map(|x| vec![x.x, x.y])
        .collect::<Vec<_>>();
    println!("{:?}", data);
    println!("{:?}", xs);
}

fn to_points(r: &Rectangle) -> impl Iterator<Item = Point> + '_ {
    let xs = r.a.x.min(r.b.x)..r.a.x.max(r.b.x);
    let ys = r.a.y.min(r.b.y)..r.a.y.max(r.b.y);

    xs.flat_map(move |x| {
        ys.clone()
            .map(move |y| Point { x, y })
    })
}

#[test]
fn flatmap_playground3() {
    let data = vec![Rectangle {
        a: Point { x: 1, y: 2 },
        b: Point { x: 3, y: 4 },
    }];
    let xs = data
        .iter()
        .flat_map(to_points)
        .collect::<Vec<_>>();
    println!("{:?}", data);
    println!("{:?}", xs);
}

#[test]
fn flatten1() {
    let xs = vec![vec![1, 2, 3], vec![4, 5], vec![7, 8, 9, 10]];
    let ys = xs
        .iter()
        .flatten()
        .collect::<Vec<_>>();
    println!("{:?}", ys);
    // [1, 2, 3, 4, 5, 7, 8, 9, 10]
}

#[test]
fn flat_map1() {
    let xs = vec![1, 2, 3];
    let ys = xs
        .iter()
        .flat_map(|&x| vec![-x, x])
        .collect::<Vec<_>>();
    println!("{:?}", ys);
    // [-1, 1, -2, 2, -3, 3]
}
