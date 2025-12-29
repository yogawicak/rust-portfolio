#[test]
fn test_string_manipulation() {
    let mut s = String::from("Hello");
    s.push_str(" World");
    println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = s1 + &s2; // s1 moved here
    println!("{}", s3);

    let s4 = format!("{}-{}", s2, s3);
    println!("{}", s4);
}
