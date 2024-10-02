enum Solution {
    NoRoots,
    OneRoot(f32),
    TwoRoots(f32, f32),
}

fn solve_quadratic(a: f32, b: f32, c: f32) -> Solution {
    todo!()
}

fn solve_and_report(a: f32, b: f32, c: f32) {
    use Solution::*;

    // will not compile
    // match solve_quadratic(a, b, c) {
    //     OneRoot(x) => println!("quadratic equation has one root: {x}"),
    //     TwoRoots(x1, x2) => println!("quadratic equation has two roots: {x1}, {x2}"),
    // }
}

fn test(xy: (i32, i32)) {
    let s = match xy {
        (0, y) => format!("on the axis X, y={y}"),
        (x, 0) => format!("on the axis Y, x={x}"),
        (1, 1) => "at x=1, y=0".to_owned(),
        (x, y) => format!("x={x}, y={y}"),
    };
}
