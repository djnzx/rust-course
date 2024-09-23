#[test]
fn test1_tuple() {
    let x = (1, 3.14, "Alex".to_string(), true);
    let (a, b, c, d) = x.clone();

    let a = x.0;
    let b = x.1;
    let c = x.2;
    let d = x.3;
}

#[test]
fn test2_named_tuple() {
    #[derive(Clone)]
    struct Person(String, i32);

    let person: Person = Person("Jim".to_string(), 33);

    let Person(name, age) = person.clone();

    let n = person.0;
    let a = person.1;
}

#[test]
fn test3_struct() {
    #[derive(Clone)]
    struct Pizza {
        name: String,
        size: i32,
    }

    let pizza: Pizza = Pizza {
        name: "Margarita".to_string(),
        size: 60,
    };

    let Pizza { name, size } = pizza.clone();

    let n = pizza.name;
    let s = pizza.size;
}
