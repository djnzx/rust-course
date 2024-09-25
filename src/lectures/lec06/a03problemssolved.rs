enum Option<A> {
    None,
    Some(A),
}

fn min(xs: &i32) -> Option<i32> {
    todo!()
}

enum FoundResult<A> {
    NotFound,
    Found(A),
}

fn index_of(x: i32, xs: &[i32]) -> FoundResult<usize> {
    todo!()
}

enum IOResult<E, A> {
    Error(E),
    Result(A),
}

fn read_file(name: String) -> IOResult<String, String> {
    todo!()
}

enum Color {
    Red,
    Yellow,
    Green,
}

fn whatever(color: Color) {
    todo!()
}

fn test() {
    use Color::*;

    whatever(Red);
    whatever(Green);
    // whatever(Blue);
}

enum AuthenticationResult {
    Token(String),
    InvalidUsername,
    InvalidPassword,
    PasswordExpired,
    UserIsLocked,
}

fn auth(username: String, password: String) -> AuthenticationResult {
    todo!()
}
