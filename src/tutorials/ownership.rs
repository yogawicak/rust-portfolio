#[test]
fn test_ownership() {
    // Ownership Rules:
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    let s1 = String::from("Hello");
    let s2 = s1; // Ownership moved from s1 to s2

    // println!("{}", s1); // Error: value borrowed here after move
    println!("{}", s2);

    // Clone to copy data
    let s3 = s2.clone();
    println!("s2: {}, s3: {}", s2, s3);
}
