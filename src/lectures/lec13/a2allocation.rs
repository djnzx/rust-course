#[test]
fn test131() {
    let x = 5;
    // x = x + 1;

    let mut k = 5;
    k = k + 1;
}

#[test]
fn test_shadowing() {
    let x = 5;

    let x = 6;

    let x = "trick";
}

#[test]
fn test_shadowing_bad() {
    let x = 5;
    let x = 6;
}

#[test]
fn test_shadowing_good() {
    let x = "5";
    let x = x
        .parse::<u32>()
        .unwrap();
}

#[test]
fn test_shadowing_good2() {
    let spaces = "   ";
    let spaces = spaces.len();
}

fn test_shadowing_doesnt_compile() {
    let mut spaces = "   ";
    // spaces = spaces.len();
}
