use colored::Colorize;

fn show_separator() {
    println!("{:->115}", "");
}

fn show_f32_header() {
    colored::control::set_override(true);
    println!("{:^30} | {:^47} | {:^32}", "scientific", "normal", "binary");
}

fn show_f32(n: &f32) {
    let bits = n.to_bits();
    let bits = format!("{:0>32b}", bits);
    let (sign, em) = bits.split_at(1);
    let (e, m) = em.split_at(8);

    let pretty = format!("{}{}{}", sign.red(), e.blue(), m);

    match n {
        0f32 => show_separator(),
        n => println!("{0:30e} | {0:47} | {1}", n, pretty),
    }
}

#[test]
fn float_representation() {
    let x = 0.3125_f32;
    println!("{:0>32b}", x.to_bits());
    // 00111110101000000000000000000000
    // ^\______/\_____________________/
    // |   |        |
    // |   |        |
    // |   |        |                f32 | f64
    // |   |        |      ------------------------
    // |   |        -- M - mantissa   23 | 52  bits
    // |   ----------- E - exponent    8 | 11  bits
    // --------------- S - sign        1 |  1  bit
    //
    // (-1)^s * 1.M * 2^(E-127)
    // IEE 754 https://uk.wikipedia.org/wiki/IEEE_754
    // https://en.wikibooks.org/wiki/A-level_Computing/AQA/Paper_2/Fundamentals_of_data_representation/Floating_point_numbers#:~:text=the%20mantissa%20holds%20the%20main,remaining%206%20for%20the%20exponent.
    // https://habr.com/ru/articles/337260/
    //
    // 1 = 0 01111111 00000000000000000000000
    // S = 0, therefore (-1)^0 = 1
    // M = 0, therefore we deal with 1.0
    // E = 01111111 = 127
    // exponent is stored with bias = 127, therefore
    //    E real | E stored
    //    ---------------------
    //     -127  |     0
    //     -126  |     1
    //     -125  |     2
    //      ...  |   ...
    //       -3  |   124
    //       -2  |   125
    //       -1  |   126
    //        0  |   127
    //        1  |   128
    //        2  |   129
    //        3  |   130
    //      ...  |   ...
    //      125  |   252
    //      126  |   253
    //      127  |   254
    //      inf  |   255
    //
}

#[test]
fn test_3_float_is_imprecise_1() {
    //          123456790000000000
    let x = 123456789123456789_f32;
    println!("{}", x);
    println!("{:e}", x);
}

#[test]
fn test_3_float_is_imprecise_2() {
    let x = 16777216_f32;
    println!("{}", x);
    let x = 16777217_f32;
    println!("{}", x);
    let x = 1 << 24;
    println!("{}", x);
}

#[test]
fn test_3_float_is_imprecise_3() {
    let x = 0.2 + 0.1;
    println!("{}", x);
    // 0.30000000000000004
}

// #[rustfmt::skip]
#[test]
fn look_for_representations() {
    const SEP: f32 = 0f32;
    show_f32_header();
    [
        SEP,
        1_f32,
        -1_f32,
        2_f32,
        3_f32,
        SEP, //
        1_f32 / 2_f32,
        0.3125_f32,
        2.0_f32.powi(-1),
        SEP,              //
        16777216_f32,     // max
        2.0_f32.powi(24), // max
        (1 << 24) as f32,
        SEP,          //
        16777216_f32, // max
        16777217_f32, // max
        16777218_f32, // max
        SEP,          //
        f32::MAX,
        2.0_f32.powi(127),
        SEP, //
        2.0_f32.powi(128),
        -2.0_f32.powi(128),
        SEP,                //
        2.0_f32.powi(-127), // min
        2.0_f32.powi(-127) * 2_f32,
        2.0_f32.powi(-127) * 4_f32,
        SEP,    //
        0.1f32, //
        0.2f32, //
    ]
    .iter()
    .for_each(show_f32);
}

fn show_u32_f32(nu: &u32) {
    let n = *nu as f32;
    let bits = n.to_bits();
    let bits = format!("{:0>32b}", bits);
    let (sign, em) = bits.split_at(1);
    let (e, m) = em.split_at(8);

    let pretty = format!("{}{}{}", sign.red(), e.blue(), m);

    println!("{0} | {1} | {2}", nu, n, pretty)
}

#[test]
fn precision() {
    println!("decimal  | float    | float binary");
    [16777216, 16777217, 16777218].iter().for_each(show_u32_f32);
}
