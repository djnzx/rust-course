fn print_user(name: String, age: i32) {
    todo!()
}

fn test1() {
    let name = "Jim";
    let age = 33;

    print_user(name.to_string(), age);
}

fn deliver_pizza(name: String, size: i32) {
    todo!()
}

fn test2() {
    let pizza = "Margarita";
    let size = 60;

    deliver_pizza(pizza.to_string(), size);
}

fn test3() {
    let name = "Jim";
    let age = 33;

    let pizza = "Margarita";
    let size = 60;

    print_user(pizza.to_string(), age);
    deliver_pizza(name.to_string(), size);

    print_user(name.to_string(), size);
    deliver_pizza(pizza.to_string(), age);
}

fn print_user2(person: (String, i32)) {
    todo!()
}

fn deliver_pizza2(pizza: (String, i32)) {
    todo!()
}

fn test4() {
    let person = ("Jim".to_string(), 33);
    let pizza = ("Margarita".to_string(), 60);

    print_user2(person);
    deliver_pizza2(pizza);

    let person = ("Jim".to_string(), 33);
    let pizza = ("Margarita".to_string(), 60);

    print_user2(pizza);
    deliver_pizza2(person);
}
