use std::io::Read;
use std::str::{Bytes, Chars};

#[test]
fn iteration_chars_ascii() {
    let s5 = "hello".to_string();
    for c in s5.chars() {
        print!("{} ", c);
    }
    // h e l l o
    println!();
}

#[test]
fn iteration_chars_non_ascii() {
    let s5 = "Привіт".to_string();
    for c in s5.chars() {
        print!("{} ", c);
    }
    // П р и в і т
    println!();
}

#[test]
fn iteration_bytes_non_ascii() {
    let s5 = "Привіт".to_string();
    for c in s5.bytes() {
        print!("{} ", c);
    }
    // 208 159 209 128 208 184 208 178 209 150 209 130
    println!();
}

#[test]
fn length() {
    let s1 = "hello";
    println!("{}", s1.len()); // 5

    let s1 = "Привіт";
    println!("{}", s1.len()); // 12
}

#[test]
fn slicing_ascii() {
    let hello = "hello";
    dbg!(&hello[0..2]); // "he"
    dbg!(&hello[0..4]); // "hell"
}

#[ignore]
#[test]
fn slicing_utf8_panic() {
    let hello = "Привіт";
    dbg!(&hello[0..3]); // panic
}

#[test]
fn slicing_utf8_ok1() {
    let hello = "Привіт";
    dbg!(&hello[0..2]); // П

    let s1 = hello[0..4].bytes();
    for b in s1 {
        print!("{:08b} ", b);
    }
    // 11010000 10011111
    // ^^^      ^^
    //    10000    11111
    //
    //    1000011111
    //    ...
    //    1055
    //
    println!();
}

#[test]
fn slicing_utf8_ok2() {
    let hello = "Привіт";
    dbg!(&hello[0..4]); // Пр
}

/// Rust strings don’t support indexing
#[test]
fn utf8_1_byte() {
    let hello = String::from("Hello");
    println!("{}", hello);

    hello
        .chars()
        .for_each(|c| print!("{}, ", c));
    println!();

    hello
        .bytes()
        .for_each(|b| print!("{}, ", b));
    println!();

    hello
        .bytes()
        .for_each(|b| print!("{:#02x}, ", b));
    println!();
    // Hello
    // H, e, l, l, o,
    // 72, 101, 108, 108, 111,
    // 0x48, 0x65, 0x6c, 0x6c, 0x6f,
}

#[test]
fn utf8_2bytes() {
    let hello = String::from("Привіт");
    let chars: Chars = hello.chars();
    let bytes: Bytes = hello.bytes();

    dbg!(&hello); // "Hello"
    dbg!(&chars); // Chars(['П',     'р',     'и',     'в',     'і',     'т')
    dbg!(&bytes); // Bytes([208,159, 209,128, 208,184, 208,178, 209,150, 209,130)

    dbg!(hello.len()); // 12
}

#[test]
fn utf8mixed_len() {
    let hello = String::from("рa");
    let chars: Chars = hello.chars();
    let bytes: Bytes = hello.bytes();
    dbg!(&hello); // "рa"
    dbg!(chars.count()); // 2
    dbg!(bytes.count()); // 3
    dbg!(hello.len()); // 3
}

#[test]
fn utf8_3bytes() {
    let hello = "नमस्ते";
    // ['न', 'म', 'स', '्', 'त', 'े']
    // ["न", "म", "स्",      "ते"     ]

    let chars: Chars = hello.chars();
    let bytes: Bytes = hello.bytes();

    dbg!(hello); // "नमस\u{94d}त\u{947}"
    dbg!(&chars); // ['न', 'म', 'स', '\u{94d}', 'त', '\u{947}']
    dbg!(&chars); // ['न', 'म', 'स', '्',       'त', 'े'      ]

    dbg!(&chars);
    // ['न', 'म', 'स', '्', 'त', 'े' ]

    dbg!(&bytes);
    // [224, 164, 168,  224, 164, 174,  224, 164, 184,
    //  -------------   -------------   -------------
    //  224, 165, 141,  224, 164, 164,  224, 165, 135]
    //  -------------   -------------   -------------

    for b in bytes {
        print!("{:08b} ", b);
    }
    println!();
    // 11100000 10100100 10101000
    // ^^^^     ^^       ^^
    //     0000   100100   101000

    //       0000100100101000
}

#[test]
fn utf8_4bytes() {
    let hello = "😀🤪😐🙄";
    println!("{}", hello); // 😀🤪😐🙄

    hello
        .chars()
        .for_each(|c| print!("{}, ", c));
    println!(); // 😀, 🤪, 😐, 🙄,

    hello
        .bytes()
        .for_each(|b| print!("{}, ", b));
    println!();
    // bytes
    //
    // 240, 159, 152, 128,  240, 159, 164, 170,
    // ------------------   -------------------
    // 240, 159, 152, 144,  240, 159, 153, 132,
    // ------------------   -------------------
    println!("{}", hello.len()); // 16

    for b in hello.bytes() {
        print!("{:08b} ", b);
    }
    println!();
}

fn bytes(c: char) -> Vec<u8> {
    let x = c.to_string();
    x.into_bytes()
}

#[test]
fn char_suze() {
    let s = 'a';
    println!("{:?}", bytes(s)); // [97]
    let s = 'Щ';
    println!("{:?}", bytes(s)); // [208, 169]
    let s = '你';
    println!("{:?}", bytes(s)); // [228, 189, 160]
    let s = '💡';
    println!("{:?}", bytes(s)); // [240, 159, 146, 161]
}

#[test]
fn utf8() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Hola");
}
