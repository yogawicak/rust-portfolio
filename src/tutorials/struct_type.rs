struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple struct
struct Color(i32, i32, i32);

fn print_user_email(user: &User) {
    println!("User Email: {}", user.email);
}

#[test]
fn test_struct() {
    let user1 = User {
        email: String::from("yoga@example.com"),
        username: String::from("yoga"),
        active: true,
        sign_in_count: 1,
    };

    println!("User: {:?}", user1.email);

    // Update syntax
    let user2 = User {
        email: String::from("eko@example.com"),
        ..user1
    };
    println!("User2: {}", user2.username);
}

#[test]
fn test_struct_in_function() {
    let user1 = User {
        email: String::from("yoga@example.com"),
        username: String::from("yoga"),
        active: true,
        sign_in_count: 1,
    };
    print_user_email(&user1);
}

#[test]
fn test_init_shorthand() {
    let email = String::from("yoga@example.com");
    let username = String::from("yoga");

    let user1 = User {
        email,    // email: email
        username, // username: username
        active: true,
        sign_in_count: 1,
    };

    println!("User: {:?}", user1.email);
}

#[test]
fn test_struct_update_syntax() {
    let user1 = User {
        email: String::from("yoga@example.com"),
        username: String::from("yoga"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("eko@example.com"),
        ..user1
    };

    println!("User2: {:?}", user2.email);
}

#[test]
fn test_tuple_struct() {
    let black = Color(0, 0, 0);
    println!("Color: {}", black.0);
}
