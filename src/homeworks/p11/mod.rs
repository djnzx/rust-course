use rand::Rng;

// Generate a random vector of length n with values in [10..99]
pub fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

// Find the pair of adjacent elements with the smallest sum
pub fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32) {
    data.windows(2)
        .enumerate()
        .map(|(i, pair)| (i, i + 1, pair[0] + pair[1]))
        .min_by_key(|&(_, _, sum)| sum)
        .unwrap_or((0, 0, 0))
}

// Print the vector and the min adjacent pair in a readable format
pub fn print_adjacent_sum(data: &[i32]) {
    // Print indexes
    print!("indexes:");
    for i in 0..data.len() {
        print!("{:>2}. ", i);
    }
    println!();

    // Print data
    println!("data:  {:?}", data);

    // Find min adjacent pair
    let (i1, i2, sum) = min_adjacent_sum(data);

    // Print arrows pointing to the pair
    print!("indexes:");
    for i in 0..data.len() {
        if i == i1 || i == i2 {
            print!(" \\__");
        } else {
            print!("    ");
        }
    }
    println!();
    
    print!("indexes:");
    for i in 0..data.len() {
        if i == i1 || i == i2 {
            print!(" __/");
        } else {
            print!("    ");
        }
    }
    println!();

    // Print the result
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[i1], data[i2], sum, i1, i2
    );
    println!();
}

#[test]
fn test_min_adjacent_sum() {
    let test_cases = vec![
        (
            vec![45, 87, 49, 64, 50, 37, 45, 72, 55, 64, 90, 86, 60, 54, 78, 72, 83, 44, 89, 22],
            (5, 6, 82),
        ),
        (
            vec![29, 92, 14, 65, 57, 98, 10, 45, 11, 48, 69, 21, 12, 75, 51, 69, 72, 36, 47, 45],
            (11, 12, 33),
        ),
        (
            vec![19, 86, 66, 95, 40, 24, 90, 74, 98, 37, 26, 44, 76, 86, 48, 63, 11, 38, 29, 40],
            (16, 17, 49),
        ),
        (
            vec![30, 18, 68, 87, 99, 81, 88, 45, 34, 79, 81, 79, 93, 55, 26, 24, 32, 55, 59, 97],
            (0, 1, 48),
        ),
    ];

    for (data, expected) in test_cases {
        assert_eq!(min_adjacent_sum(&data), expected);
    }
}

fn main() {
    // Generate and print 4 random vectors with their min adjacent sums
    for _ in 0..4 {
        let data = gen_random_vector(20);
        print_adjacent_sum(&data);
    }
}
