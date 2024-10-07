#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn move_point(p: Point, dx: i32, dy: i32) -> Point {
    Point {
        x: p.x + dx,
        y: p.y + dy,
    }
}

#[test]
fn test1() {
    let p = Point { x: 1, y: 2 };
    let p = move_point(p, 10, 20);
    assert_eq!(p, Point { x: 11, y: 22 });
}
