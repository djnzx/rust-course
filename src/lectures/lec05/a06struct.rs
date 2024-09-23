// declaration

struct Person {
    name: String,
    age: i32,
}

struct Pizza {
    name: String,
    size: i32,
}

#[test]
fn test3_struct_creation_accessing() {
    // creation
    let person: Person = Person {
        name: "Jim".to_string(),
        age: 33,
    };

    let pizza: Pizza = Pizza {
        name: "Margarita".to_string(),
        size: 60,
    };

    // accessing, easy
    let name1 = person.name;
    let age1 = person.age;
}

#[test]
fn test3_destructurization() {
    // we can create a named tuple
    // declaration

    struct Person {
        name: String,
        age: i32,
    }

    // creation
    let person: Person = Person {
        name: "Jim".to_string(),
        age: 33,
    };

    let Person { name, age } = person;
    // let Person { surname, age } = person;
}

fn profit_harder_to_mix() {
    struct Point2d {
        x: i32,
        y: i32,
    }

    let p1 = Point2d { x: 2, y: 3 };
    let p2 = Point2d { x: 3, y: 2 };
}
