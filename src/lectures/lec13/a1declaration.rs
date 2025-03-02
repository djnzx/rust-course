fn test131() {
    struct Person {
        id: u32,
        name: String,
    }

    fn whatever() {
        // ...
        let p = Person {
            id: 33,
            name: "Jim".to_string(),
        };
        // ...
    }

    print!("hello");
}
