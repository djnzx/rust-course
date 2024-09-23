#[ignore]
#[test]
fn solve1() {
    let x: f32 = solve_quadratic1(1.0, -1.0, 0.0);
    let (x1, x22): (f32, f32) = solve_quadratic2(1.0, -1.0, 0.0);
    // what to write and how to write
}

fn solve_quadratic1(a: f32, b: f32, c: f32) -> f32 {
    todo!()
}

fn solve_quadratic2(a: f32, b: f32, c: f32) -> (f32, f32) {
    todo!()
}

enum Solution {
    NoRoots,
    OneRoot(f32),
    TwoRoots(f32, f32),
}

fn solve_quadratic(a: f32, b: f32, c: f32) -> Solution {
    use Solution::*;

    let d = b.powi(2) - 4.0 * a * c;
    match d {
        0. => OneRoot(-b / (2. * a)),
        d if d < 0. => NoRoots,
        d => {
            let dq = f32::sqrt(d);
            let a2 = a * 2.;
            TwoRoots((-b - dq) / a2, (-b + dq) / a2)
        }
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
