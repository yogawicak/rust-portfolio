// Crate is a compilation unit in Rust.
// We are currently in a binary crate (rust-portfolio).
// This file is part of the binary crate.

#[test]
fn test_crate() {
    // Using a function from the standard library crate (std)
    let x = std::cmp::max(1, 2);
    println!("Max: {}", x);
}
