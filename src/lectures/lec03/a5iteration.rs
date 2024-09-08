#[test]
fn iteration_basic() {
    let xs = [10, 13, 17, 25, 37, 51, 60];

    for x in xs {
        print!("{} ", x);
    }
    // 10 13 17 25 37 51 60
}

#[test]

fn iteration_advanced() {
    let xs = [10, 13, 17, 25, 37, 51, 60];

    let it = xs.iter();
    it.for_each(|x| print!("{} ", x));
    // 10 13 17 25 37 51 60
}

#[test]
fn iteration_indexed() {
    let xs = [10, 13, 17, 25, 37, 51, 60];

    for (idx, x) in xs.iter().enumerate() {
        print!("[{}]->{}, ", idx, x);
    }
    // [0]->10, [1]->13, [2]->17, [3]->25, [4]->37, [5]->51, [6]->60
}
