fn car_parking_roof(cars: &[usize], k: usize) -> usize {
    let mut cars = cars.to_vec();
    cars.sort_unstable();
    (0..(cars.len() - k))
        .map(|idx| cars[idx + k - 1] - cars[idx] + 1)
        .min()
        .unwrap()
}

#[test]
fn car_parking_roof_test() {
    let cars = [6, 2, 12, 7];
    let k = 3;

    let roof_size = car_parking_roof(&cars, k);
    let expected = 6;
    assert_eq!(expected, roof_size);
}
