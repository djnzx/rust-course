pub fn count_permutation(shipments: &Vec<u32>) -> i32 {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;
    
    if total % n != 0 {
        return -1;
    }
    
    let average = total / n;
    let mut moves = 0;
    let mut balance = 0;
    
    for &weight in shipments {
        balance += weight as i32 - average as i32;
        moves += balance.abs();
    }
    
    moves / 2
}

pub fn gen_shipments(n: usize) -> Vec<u32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let average = rng.gen_range(1..100);
    let mut shipments: Vec<u32> = (0..n).map(|_| rng.gen_range(average-10..average+10)).collect();
    
    let total: u32 = shipments.iter().sum();
    let remainder = total % n as u32;
    
    if remainder != 0 {
        let diff = n as u32 - remainder;
        shipments[0] += diff;
    }
    
    shipments
}

#[test]
fn test_count_permutation() {
    assert_eq!(count_permutation(&vec![1, 1, 1, 1, 6]), 4);
    assert_eq!(count_permutation(&vec![8, 2, 2, 4, 4]), 4);
    assert_eq!(count_permutation(&vec![9, 3, 7, 2, 9]), 7);
    assert_eq!(count_permutation(&vec![1, 2, 3]), -1);
    assert_eq!(count_permutation(&vec![5, 5, 5, 5]), 0);
}

#[test]
fn test_gen_shipments() {
    for _ in 0..10 {
        let n = rand::thread_rng().gen_range(1..20);
        let shipments = gen_shipments(n);
        let total: u32 = shipments.iter().sum();
        assert_eq!(total % n as u32, 0, "Generated shipments should be equally distributable");
    }
}

fn main() {
    let shipments = vec![1, 1, 1, 1, 6];
    println!("Moves needed: {}", count_permutation(&shipments));
    
    let valid_shipments = gen_shipments(5);
    println!("Generated shipments: {:?}", valid_shipments);
    println!("Moves needed: {}", count_permutation(&valid_shipments));
}
