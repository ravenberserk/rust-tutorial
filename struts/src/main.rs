struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("User1 - Email: {} - Name: {} - Active: {} - SingIn: {}.", user1.email, user1.username, user1.active, user1.sign_in_count);

    user1.email = String::from("change@example.com");
    println!("User1 email: {}.", user1.email);

    let user2 = build_user(String::from("test@example.com"), String::from("user2"));
    println! ("User2 name: {}.", user2.username);

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println! ("User3 email: {}.", user3.email);


}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
