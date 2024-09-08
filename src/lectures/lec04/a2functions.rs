#[test]
fn functions1() {
    let s = "Hello!";

    let x = s.contains("ll");
    println!("{}", x); // true
    let x = s.contains("ee");
    println!("{}", x); // false
    let x = s.is_empty();
    println!("{}", x); // false
    let x = s.starts_with("H");
    println!("{}", x); // true
    let x = s.ends_with("?");
    println!("{}", x); // false
}

#[test]
fn functions2() {
    // Створення нових рядків
    let mut s1 = String::new(); // Порожній рядок
    let s2 = "Привіт".to_string(); // Перетворення рядка з &str в String
    let s3 = String::from("Світ"); // Інший спосіб створення рядка

    // Додавання вмісту до рядка
    s1.push_str("Привіт");
    println!("s1 після push_str: {}", s1);

    s1.push('!'); // Додаємо окремий символ
    println!("s1 після push: {}", s1);

    // Конкатенація рядків
    let s4 = s2 + &s3; // s2 більше не доступний після цієї операції, бо вона забирає його значення
    println!("Результат конкатенації: {}", s4);

    // Використання макросу format! для конкатенації без зміни оригінальних рядків
    let s5 = format!("{} {}!", s4, s1); // Створюємо новий рядок
    println!("Результат format!: {}", s5);

    // Ітерування через символи та байти
    println!("Ітерування через символи:");
    for c in s5.chars() {
        print!("{} ", c);
    }
    println!();

    println!("Ітерування через байти:");
    for b in s5.bytes() {
        print!("{} ", b);
    }
    println!();

    // Витягування підрядків (важливо пам'ятати, що не можна індексувати String напряму)
    let substring = &s5[0..6]; // Індекси повинні співпадати з межами символів
    println!("Підрядок: {}", substring);

    // Заміна частини рядка
    let s6 = s5.replace("Привіт", "Hello");
    println!("Після заміни: {}", s6);

    // Перевірка порожності
    if s1.is_empty() {
        println!("s1 порожній");
    } else {
        println!("s1 не порожній");
    }

    // Довжина рядка (у байтах)
    println!("Довжина рядка s5: {} байтів", s5.len());

    // Конвертація String у &str
    let s7: &str = &s6; // Конвертація рядка у посилання на підрядок
    println!("Посилання на рядок: {}", s7);

    // Приведення до верхнього/нижнього регістру
    let upper = s7.to_uppercase();
    let lower = s7.to_lowercase();
    println!("Верхній регістр: {}", upper);
    println!("Нижній регістр: {}", lower);

    // Розбиття рядка
    let sentence = "Руст — це цікава мова програмування.";
    for word in sentence.split_whitespace() {
        println!("Слово: {}", word);
    }

    // Видалення пробілів
    let with_spaces = "   Руст    ";
    let trimmed = with_spaces.trim();
    println!("Після trim: '{}'", trimmed);

    // Вставка символів
    let mut s8 = String::from("Програмування");
    // s8.insert(11, '-'); // fail
    s8.insert(12, '-'); // fail
    println!("Після insert: {}", s8);

    // Видалення символів
    s8.pop(); // Видаляє останній символ
    println!("Після pop: {}", s8);

    // Очищення рядка
    s8.clear();
    println!("Після clear: '{}'", s8);
}

#[test]
fn iteration_chars() {
    let s5 = "Hello".to_string();
    for c in s5.chars() {
        print!("{} ", c);
    }
    // H e l l o
    println!();
}

#[test]
fn iteration_bytes_ascii() {
    let s5 = "Hello".to_string();
    for c in s5.bytes() {
        print!("{} ", c);
    }
    // 72 101 108 108 111
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
