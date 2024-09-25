enum Solution {
    NoRoots,
    OneRoot(f32),
    TwoRoots(f32, f32),
}

fn solve_quadratic(a: f32, b: f32, c: f32) -> Solution {
    use Solution::*;

    let d = b.powi(2) - 4.0 * a * c;

    if d < 0. {
        NoRoots
    } else if d == 0. {
        OneRoot(-b / (2. * a))
    } else {
        let dq = f32::sqrt(d);
        let a2 = a * 2.;
        TwoRoots((-b - dq) / a2, (-b + dq) / a2)
    }
}
