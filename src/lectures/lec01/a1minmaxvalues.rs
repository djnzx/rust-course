use std::{f32, f64};

#[test]
fn min_max() {
    println!("        {:40} | {:40}", "MIN", "MAX");
    println!("{:-<91}", "");
    println!("i8:     {:40} | {:40}", i8::MIN, i8::MAX);
    println!("i16:    {:40} | {:40}", i16::MIN, i16::MAX);
    println!("i32:    {:40} | {:40}", i32::MIN, i32::MAX);
    println!("i64:    {:40} | {:40}", i64::MIN, i64::MAX);
    println!("i128:   {:40} | {:40}", i128::MIN, i128::MAX);
    println!("isize:  {:40} | {:40}", isize::MIN, isize::MAX);
    println!("{:-<91}", "");
    println!("u8:     {:40} | {:40}", u8::MIN, u8::MAX);
    println!("u16:    {:40} | {:40}", u16::MIN, u16::MAX);
    println!("u32:    {:40} | {:40}", u32::MIN, u32::MAX);
    println!("u64:    {:40} | {:40}", u64::MIN, u64::MAX);
    println!("u128:   {:40} | {:40}", u128::MIN, u128::MAX);
    println!("usize:  {:40} | {:40}", usize::MIN, usize::MAX);
    println!("{:-<91}", "");
    println!("f32:    {:40e} | {:40e}", f32::MIN, f32::MAX); // ~7-8 digits
    println!("f64:    {:40e} | {:40e}", f64::MIN, f64::MAX); // ~16 digits
    println!("{:-<91}", "");
}
