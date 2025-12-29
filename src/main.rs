mod tutorials;
mod user_system; // Tell Rust to look for user_system.rs

use user_system::{create_default_user, UserManager};

fn main() {
    println!("--- User System Demo ---");

    // 1. OWNERSHIP: Create the manager
    let mut manager = UserManager::new();

    // 2. OWNERSHIP: Create users (ownership moves from factory -> here)
    let user1 = create_default_user(1, "alice");
    let user2 = create_default_user(2, "bob");

    // 3. MOVE: Add users to manager (ownership moves from here -> manager)
    manager.add_user(user1);
    manager.add_user(user2);

    // println!("{:?}", user1); // ERROR! user1 is gone (moved into manager)

    // 4. IMMUTABLE BORROW: Look up a user
    if let Some(u) = manager.get_user_by_id(1) {
        println!("Found user: {} (Logins: {})", u.username, u.login_count);
    }

    // 5. MUTABLE BORROW: Update a user
    manager.increment_login_count(1);
    manager.increment_login_count(1);
    println!("Updated login stats.");

    // Check changes with another borrow
    if let Some(u) = manager.get_user_by_id(1) {
        println!("User {} now has {} logins.", u.username, u.login_count);
    }

    // 6. CONSUME: Close the system
    // Ownership of 'manager' moves into close_system and is dropped.
    manager.close_system();

    // manager.get_user_by_id(1); // ERROR! manager is gone.
}

#[test]
fn test_user_creation() {
    let user = create_default_user(100, "testuser");
    assert_eq!(user.id, 100);
    assert_eq!(user.username, "testuser");
    assert_eq!(user.email, "testuser@example.com");
    assert_eq!(user.login_count, 0);
}

#[test]
fn var_mutable_immutable_test() {
    let mut x = 10;
    let y = 20;
    println!("x before addition: {}, y: {}", x, y);
    x = x + y;

    // can't change variable type
    // x = "hello";

    println!("x after addition: {}", x);
}

#[test]
fn shadowing_test() {
    let x = 10;
    println!("x before shadowing: {}", x);

    let x = "hello";
    println!("x after shadowing: {}", x);
}

#[test]
fn data_type_test() {
    /**
     * Scalar Types:
     *  - integers
     *  - floats
     *  - booleans
     *  - characters
     */
    let x = 10;
    let y = 20.5;
    let z = true;
    let c = 'a';

    println!("x: {}, y: {}, z: {}, c: {}", x, y, z, c);

    /**
     * Compound Types:
     *  - tuples
     *  - arrays
     */
    let tuple = (10, 20.5, true, 'a');
    let array = [1, 2, 3, 4, 5];

    println!("tuple: {:?}, array: {:?}", tuple, array);
}

#[test]
fn number_conversion_test() {
    let x = 10;
    let y = 20.5;

    // Explicit conversion (narrowing)
    let w: i32 = y as i32;
    print!("w: {}", w);
}

#[test]
fn augmented_assignment_test() {
    let mut x = 10;
    println!("x before addition: {}", x);

    x += 5;
    println!("x after addition: {}", x);
}

#[test]
fn numeric_operation_test() {
    let mut x = 10;
    let y = 20;

    x += y;

    println!("x after addition: {}", x);
}

#[test]
fn comparison_test() {
    let x = 10;
    let y = 20;

    print!("{} > {}: {}", x, y, x > y);
    println!("{} < {}: {}", x, y, x < y);
    println!("{} >= {}: {}", x, y, x >= y);
    println!("{} <= {}: {}", x, y, x <= y);
    println!("{} == {}: {}", x, y, x == y);
    println!("{} != {}: {}", x, y, x != y);
}

#[test]
fn boolean_operation_test() {
    let x = true;
    let y = false;

    println!("{} && {}: {}", x, y, x && y);
    println!("{} || {}: {}", x, y, x || y);
    println!("!{}: {}", x, !x);
}
