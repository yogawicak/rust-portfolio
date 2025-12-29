#[test]
fn test_while_loop() {
    let mut counter = 0;
    while counter < 10 {
        println!("Counter: {}", counter);
        counter += 1;
    }
}
