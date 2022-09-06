struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
fn main() {
    let user1 = User {
        email: String::from("r@gmail.com"),
        active: false,
        username: String::from("rman"),
        sign_in_count: 20
    };

    let user2 = build_user(String::from("email"), String::from("username"));

    let user3 = User{
        ..user1
    };

    // cannot use the strings (email, username) in user1 any more since ownership of those strings have been moved to user3

    println!("{}", user1.active);
}
