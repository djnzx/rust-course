fn problem_still_easy_to_mix() {
    struct Point2d(i32, i32);

    let p1 = Point2d(2, 3);
    let p2 = Point2d(3, 2);

    let Point2d(x, y) = p1;
    let Point2d(y, x) = p2;
}
