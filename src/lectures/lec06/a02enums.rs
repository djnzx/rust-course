enum Color {
    Red,
    Yellow,
    Green,
}

fn whatever1(color: Color) {}

#[test]
fn test() {
    whatever1(Color::Green);
    // whatever1(Color::Blue);
}

enum Switch {
    On,
    Off,
}

enum Option1 {
    Enabled,
    Disabled,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// also valid
// TODO: how to construct an element of this enum ?
enum Stub {}
