#[test]
fn test_for_loop() {
    let array = [1, 2, 3, 4, 5];

    for value in array {
        println!("Value: {}", value);
    }

    for number in 1..10 {
        println!("Number: {}", number);
    }

    for number in 1..=10 {
        println!("Number inclusive: {}", number);
    }
}
