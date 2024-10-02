struct Point(i32, i32);

fn test(xy: Point) {
    let s = match xy {
        Point(0, y) => format!("on the axis X, y={y}"),
        Point(x, 0) => format!("on the axis Y, x={x}"),
        Point(1, 1) => "at x=1, y=0".to_owned(),
        Point(x, y) => format!("x={x}, y={y}"),
    };
}
