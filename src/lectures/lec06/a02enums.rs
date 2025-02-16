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

enum Result {
    Found,
    NotFound { l: Option<i32>, r: Option<i32> },
}

enum Solution {
    NoRoots,
    OneRoot { x: f32 },
    TwoRoots { x1: f32, x2: f32 },
}

fn whatever(sol: Solution) {
    match sol {
        Solution::NoRoots => {
            print!("NoRoots");
        }
        Solution::OneRoot { x } => {
            print!("One root ${x}")
        }
        Solution::TwoRoots { x1, x2 } => {
            print!("TwoRoots {x1}, {x2}")
        }
    }
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
