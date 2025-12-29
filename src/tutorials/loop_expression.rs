#[test]
fn test_loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("Counter: {}", counter);

        if counter == 10 {
            break;
        }
    }

    let result = loop {
        counter += 1;
        if counter == 20 {
            break counter * 2;
        }
    };
    println!("Result: {}", result);
}
