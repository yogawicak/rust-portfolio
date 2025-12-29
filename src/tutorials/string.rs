#[test]
fn test_string() {
    let mut s = String::from("Hello");
    println!("{}", s);

    s.push_str(", World!");
    println!("{}", s);

    let s2 = s.replace("World", "Rust");
    println!("{}", s2);
}
