enum Solution {
    NoRoots,
    OneRoot(f32),
    TwoRoots(f32, f32),
}

fn solve_quadratic(a: f32, b: f32, c: f32) -> Solution {
    use Solution::*;

    let d = b.powi(2) - 4.0 * a * c;

    match d {
        d if d > 0. => {
            let dq = f32::sqrt(d);
            let a2 = a * 2.;
            TwoRoots((-b - dq) / a2, (-b + dq) / a2)
        }
        0. => OneRoot(-b / (2. * a)),
        _ => NoRoots,
    }
}

fn solve_and_report(a: f32, b: f32, c: f32) {
    use Solution::*;

    match solve_quadratic(a, b, c) {
        NoRoots => println!("quadratic equation has no roots"),
        OneRoot(x) => println!("quadratic equation has one root: {x}"),
        TwoRoots(x1, x2) => println!("quadratic equation has two roots: {x1}, {x2}"),
    }
}

#[test]
fn solve2() {
    solve_and_report(1., 2., 2.); // no roots
    solve_and_report(1., 2., 1.); // one root
    solve_and_report(1., 1., 0.); // two roots
}
