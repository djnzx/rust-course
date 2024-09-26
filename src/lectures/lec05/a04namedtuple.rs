struct User(String, i32);
struct Pizza(String, i32);

fn print_user(user: User) {
    todo!()
}

fn deliver_pizza(pizza: Pizza) {
    todo!()
}

#[test]
fn test2_named_tuple_creation() {
    let person: User = User("Jim".to_string(), 33);
    let pizza: Pizza = Pizza("Margarita".to_string(), 60);
}

#[test]
fn test2_named_tuple_access() {
    let person: User = User("Jim".to_string(), 33);
    let pizza: Pizza = Pizza("Margarita".to_string(), 60);

    let x = person.0;
    let y = person.1;
}

#[test]
fn test2_named_tuple_destructutrization() {
    let person: User = User("Jim".to_string(), 33);

    let User(name, age) = person;
}

#[ignore]
#[test]
fn test4_named_tuple_usage() {
    let person: User = User("Jim".to_string(), 33);
    let pizza: Pizza = Pizza("Margarita".to_string(), 60);

    print_user(person);
    deliver_pizza(pizza);

    // will not compile
    // print_user(pizza);
    // deliver_pizza(person);
}
