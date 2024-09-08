use std::str::{Bytes, Chars};

#[test]
fn slicing() {
    let s = String::from("hello world");
    /// slicing doesn't take a memory
    let hello = &s[0..5]; // hello
    println!("{}", hello);

    let hello2 = &s[..5]; // hello
    println!("{}", hello2);

    let world = &s[6..11]; // world
    println!("{}", world);

    let world2 = &s[6..]; // world
    println!("{}", world2);

    let whole = &s[..]; // hello world
    println!("{}", whole);
}
#[test]
fn range_syntax() {
    let r1 = 1..5;
    let r2 = 1..=5;
    let r3 = 3..;
    let r4 = ..5;
    let r5 = ..=5;

    r1.for_each(|i| print!("{} ", i)); // 1 2 3 4
                                       // r3.for_each(|i| print!("{} ", i)); // will hang
}

fn parsing() {
    let r = "5".parse::<u32>();
    /// handle explicitly (getOrThrow)
    let int: u32 = r.clone().expect("should be a number");
    /// allow to panic with std message
    let r: u32 = r.unwrap();
}

fn stack_heap() {
    /// stack
    let data = "initial contents";
    /// move to heap
    let s1 = data.to_string();
    let s2 = String::from(data);
    /// initially in heap
    let s3 = String::from("initial contents");
}

fn concatenation() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    /// deref
    let s2a = &s2[..];
    /// fn +(self, s: &str) -> String
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");
}

#[test]
fn string_builder() {
    let mut s = String::new();
    /// grow
    s.push_str("abc");
    s.push_str("def");
    dbg!(s);
}

#[test]
fn utf8slicing() {
    let hello = "Здравствуйте";
    dbg!(&hello[0..4]); // byte slice: "Зд"
    dbg!(&hello[0..6]); // byte slice  "Здр"
}

// #[test]
fn utf8slicing_panic() {
    let hello = "Здравствуйте";
    dbg!(&hello[0..5]); // panic ! half-char
}

/// Rust strings don’t support indexing
#[test]
fn utf8_1byte() {
    let hello = String::from("Hello");
    let slice = &hello[..];
    let chars: Chars = hello.chars();
    let bytes: Bytes = hello.bytes();

    dbg!(&hello); // "Hello"
    dbg!(slice); // "Hello"
    dbg!(&chars); // Chars(['H', 'e', 'l', 'l', 'o' ])
    dbg!(&bytes); // Bytes([72,  101, 108, 108, 111 ])

    // for c in chars {
    //     dbg!(c);
    // }
    // for c in bytes {
    //     dbg!(c);
    // }
    // the type `str` cannot be indexed by `usize`
    // let x = &hello[0 as usize];
    // dbg!(x);
}

#[test]
fn utf8_2bytes() {
    let hello = String::from("Привет");
    let chars: Chars = hello.chars();
    let bytes: Bytes = hello.bytes();

    dbg!(&hello); // "Hello"
    dbg!(&chars); // Chars(['П',     'р',     'и',     'в',     'е',     'т')
    dbg!(&bytes); // Bytes([208,159, 209,128, 208,184, 208,178, 208,181, 209,130)

    dbg!(hello.len()); // 12
}

#[test]
fn utf8mixed_len() {
    let hello = String::from("рa");
    let chars: Chars = hello.chars();
    let bytes: Bytes = hello.bytes();
    dbg!(&hello); // "рa"
    dbg!(chars); // 2
    dbg!(bytes); // 3
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
                  // ['न', 'म', 'स', '्',       'त', 'े'      ]
    dbg!(&bytes); // [224, 164, 168,    224, 164, 174,    224, 164, 184,    224, 165, 141,    224, 164, 164,    224, 165, 135]
                  //  -------------     -------------     -------------     -------------     -------------     -------------
}

fn bytes(c: char) -> Vec<u8> {
    let x = c.to_string();
    x.into_bytes()
}

#[test]
fn char_suze() {
    let s = 'a';
    println!("{:?}", bytes(s));
    let s = 'Щ';
    println!("{:?}", bytes(s));
    let s = '你';
    println!("{:?}", bytes(s));
    let s = '💡';
    println!("{:?}", bytes(s));
}

fn utf8() {
    /// UTF-8
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Hola");

    let chars = &hello[..];
    // string access is not O(1) because we need to traverse it from start
    let xs = "नमस्ते"; // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
                     //                       ['न', 'म', 'स', '्', 'त', 'े']
                     //                       ["न", "म", "स्",      "ते"     ]

    /// A String is a wrapper over a Vec<u8>
    let s1 = String::from("hello");
    // let s1 = "hello";
    // let h = s1[0]; // `String` cannot be indexed by `{integer}`
    println!("{}", s1.len()); // 5
    let hello = "Здравствуйте";
    println!("{}", hello.len()); // 24
    /// char is always 4 bytes
    let chars: Chars = hello.chars();
    for c in chars {
        dbg!(c);
    }
    let chars: Bytes = hello.bytes();
    for c in chars {
        dbg!(c);
    }
    //dbg!("{}", chars.count());
    // chars.for_each(|x|println!("{}", x));
}
