#[test]
fn test_while_loop() {
    let mut counter = 0;
    let mut counter2 = 0;

    while counter < 10 {
        println!("Counter: {}", counter);
        counter += 1;
    }

    loop {
        counter2 += 1;
        if counter2 == 20 {
            break;
        }
    }
}

#[test]
fn test_loop_label() {
    let mut counter = 0;

    'outer: loop {
        counter += 1;
        if counter == 10 {
            break 'outer;
        }
    }
}
