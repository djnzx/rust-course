#[test]
fn test1of4() {
    const SIZE: u8 = 11;
    for y in 0..SIZE {
        for x in 0..SIZE {
            let c = if x + y < SIZE { ' ' } else { '*' };
            print!("{c}");
        }
        println!();
    }
}
//           *
//          **
//         ***
//        ****
//       *****
//      ******
//     *******
//    ********
//   *********
//  **********

#[test]
fn test2of4() {
    const SIZE: u8 = 11;
    for y in 0..SIZE {
        for x in 0..SIZE {
            let c = if (SIZE - x) + y < SIZE { ' ' } else { '*' };
            print!("{c}");
        }
        println!();
    }
}
// *
// **
// ***
// ****
// *****
// ******
// *******
// ********
// *********
// **********
// ***********

#[test]
fn test3of4() {
    const SIZE: u8 = 11;
    for y in 0..SIZE {
        for x in 0..SIZE {
            let c = if x + (SIZE - y) < SIZE { ' ' } else { '*' };
            print!("{c}");
        }
        println!();
    }
}
// ***********
//  **********
//   *********
//    ********
//     *******
//      ******
//       *****
//        ****
//         ***
//          **
//           *

#[test]
fn test4of4() {
    const SIZE: u8 = 11;
    for y in 0..SIZE {
        for x in 0..SIZE {
            let c = if (SIZE - x) + (SIZE - y) < SIZE {
                ' '
            } else {
                '*'
            };
            print!("{c}");
        }
        println!();
    }
}
// ***********
// ***********
// **********
// *********
// ********
// *******
// ******
// *****
// ****
// ***
// **

#[test]
fn test0() {
    const SIZE: u8 = 11;
    let half = SIZE / 2;

    fn m(a: u8) -> u8 {
        SIZE - 1 - a
    }

    for y in 0..SIZE {
        for x in 0..SIZE {
            let q1 = x + y < half;
            let q2 = m(x) + y < half;
            let q3 = x + m(y) < half;
            let q4 = m(x) + m(y) < half;
            let c = if q1 || q2 || q3 || q4 { ' ' } else { '*' };
            print!("{c}");
        }
        println!();
    }
}
//      *
//     ***
//    *****
//   *******
//  *********
// ***********
//  *********
//   *******
//    *****
//     ***
//      *
