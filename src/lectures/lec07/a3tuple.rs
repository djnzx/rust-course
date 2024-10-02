fn test(xy: (i32, i32)) {
    let s = match xy {
        (0, y) => format!("on the axis X, y={y}"),
        (x, 0) => format!("on the axis Y, x={x}"),
        (1, 1) => "at x=1, y=0".to_owned(),
        (x, y) => format!("x={x}, y={y}"),
    };
}
