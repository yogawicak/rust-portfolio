struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[test]
fn test_struct() {
    let user1 = User {
        email: String::from("yoga@example.com"),
        username: String::from("yoga"),
        active: true,
        sign_in_count: 1,
    };

    println!("User: {}", user1.username);

    // Update syntax
    let user2 = User {
        email: String::from("eko@example.com"),
        ..user1
    };
    println!("User2: {}", user2.username);
}
