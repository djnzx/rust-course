use rand::Rng;

#[derive(PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn shift(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    fn copy(&self) -> Point {
        self.clone()
    }
}

#[test]
fn test1() {
    let mut p = Point { x: 1, y: 2 };
    let p2 = p.copy();
    p.shift(10, 20);
    assert_eq!(p, Point { x: 11, y: 22 });
    assert_eq!(p2, Point { x: 1, y: 2 });
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn random() -> Self {
        let mut rt = rand::thread_rng();
        let x = rt.gen_range(5..50);
        let y = rt.gen_range(3..100);
        Point::new(x, y)
    }
}

#[test]
fn test2() {
    let p = Point::new(3, 4);
    assert_eq!(p, Point { x: 3, y: 4 });

    let p = Point::random();
    assert!(p.x >= 5 && p.x < 50);
    assert!(p.y >= 3 && p.x < 100);
}

impl Point {
    const CENTER: Point = Point { x: 0, y: 0 };
}

#[test]
fn test3() {
    let p = Point::CENTER;
    assert_eq!(p, Point { x: 0, y: 0 });
}
