#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    fn contains(&self, point: Point) -> bool {
        point.x >= self.a.x && point.x < self.b.x &&
        point.y > self.b.y && point.y <= self.a.y
    }
}

fn area_occupied(rects: &Vec<Rectangle>) -> i32 {
    if rects.is_empty() {
        return 0;
    }

    let min_x = rects.iter().map(|r| r.a.x).min().unwrap();
    let max_x = rects.iter().map(|r| r.b.x).max().unwrap();
    let min_y = rects.iter().map(|r| r.b.y).min().unwrap();
    let max_y = rects.iter().map(|r| r.a.y).max().unwrap();

    let mut total_area = 0;

    for x in min_x..max_x {
        for y in min_y..max_y {
            let point = Point { x, y };
            if rects.iter().any(|r| r.contains(point)) {
                total_area += 1;
            }
        }
    }

    total_area
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
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

#[test]
fn test_single_rectangle() {
    let rects = vec![
        Rectangle {
            a: Point { x: 0, y: 4 },
            b: Point { x: 4, y: 0 },
        }
    ];
    assert_eq!(area_occupied(&rects), 16);
}

#[test]
fn test_overlapping_rectangles() {
    let rects = vec![
        Rectangle {
            a: Point { x: 0, y: 4 },
            b: Point { x: 4, y: 0 },
        },
        Rectangle {
            a: Point { x: 2, y: 6 },
            b: Point { x: 6, y: 2 },
        },
    ];
    assert_eq!(area_occupied(&rects), 28);
}

fn main() {
    let data = test_data();
    println!("Occupied area: {}", area_occupied(&data));
}
