use std::cmp::min;

fn car_parking_roof1(cars: &[u32], k: usize) -> u32 {
    let mut cars = cars.to_vec();
    cars.sort();

    let roof_size = |i1: usize, i2: usize| -> u32 { cars[i2] - cars[i1] + 1 };

    let mut m = roof_size(0, k - 1);

    println!("{:?}", cars);

    for idx in 0..cars.len() - k + 1 {
        let s = roof_size(idx, idx + (k - 1));
        m = min(m, s);
        println!("i: {idx}, size: {s}, min:{m}");
    }

    m
}

fn car_parking_roof2(cars: &[usize], k: usize) -> usize {
    let mut cars = cars.to_vec();
    cars.sort_unstable();
    let roof_size = |idx: usize| cars[idx + k - 1] - cars[idx] + 1;
    (0..(cars.len() - k + 1))
        .map(roof_size)
        .min()
        .unwrap()
}

fn car_parking_roof3(cars: &[u32], k: usize) -> u32 {
    let mut cars = cars.to_vec();
    cars.sort();
    let roof_size = |i: (usize, usize)| cars[i.0].abs_diff(cars[i.1]) + 1;
    let it1 = 0..;
    let it2 = (k - 1)..cars.len();
    it1.zip(it2)
        .map(roof_size)
        .min()
        .unwrap()
}

#[test]
fn car_parking_roof_test1() {
    let cars = [6, 2, 12, 7];
    let k = 3;
    let roof_expected = 6;
    let roof_real = car_parking_roof2(&cars, k);
    assert_eq!(roof_expected, roof_real);
}

#[test]
fn car_parking_roof_test2() {
    let cars = [6, 2, 12, 7, 20, 25, 26, 28];
    let k = 3;
    let roof_expected = 4;
    let roof_real = car_parking_roof3(&cars, k);
    assert_eq!(roof_expected, roof_real);
}
