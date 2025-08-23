use itertools::Itertools;

fn main() {
    let solutions = find_solutions();
    println!("Кількість рішень: {}", solutions.len());
}

fn find_solutions() -> Vec<[u8; 8]> {
    let mut solutions = Vec::new();
    let digits: Vec<u8> = (1..=8).collect();

    // перебір унікальних комбінацій для 8 літер
    for perm in digits.iter().permutations(8) {
        let (m, u, x, a, s, l, o, n) = (
            *perm[0], *perm[1], *perm[2], *perm[3], 
            *perm[4], *perm[5], *perm[6], *perm[7]
        );

        // перевірка рівняння
        let muxa = m * 1000 + u * 100 + x * 10 + a;
        let xa = x * 10 + a;
        let slon = s * 1000 + l * 100 + o * 10 + n;

        if muxa + xa == slon {
            solutions.push([m, u, x, a, s, l, o, n]);
        }
    }

    solutions
}

#[test]
fn test_solutions() {
    let solutions = find_solutions();
    assert_eq!(solutions.len(), 1);
    let sol = &solutions[0];
    println!("  {}{}{}{}", sol[0], sol[1], sol[2], sol[3]);
    println!("    {}  {}", sol[2], sol[3]);
    println!("  -----");
    println!("  {}{}{}{}", sol[4], sol[5], sol[6], sol[7]);
}
