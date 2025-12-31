#[test]
fn test_for_loop() {
    let array = [1, 2, 3, 4, 5];
    let array2 = ['a', 'b', 'c', 'd', 'e'];

    for value in array {
        println!("Value: {}", value);
    }

    for number in 1..10 {
        println!("Number: {}", number);
    }

    for number in 1..=10 {
        println!("Number inclusive: {}", number);
    }

    for char in array2 {
        println!("Char: {}", char);
    }
}

#[test]
fn test_range_test() {
    let range = 1..10;
    for number in range {
        println!("Number: {}", number);
    }
}
