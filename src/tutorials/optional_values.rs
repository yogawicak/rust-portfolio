#[test]
fn test_optional_values() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    if let Some(i) = some_number {
        println!("Value: {}", i);
    }

    if absent_number.is_none() {
        println!("No value");
    }
}
