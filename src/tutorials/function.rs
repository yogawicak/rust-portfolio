fn say_hello() {
    println!("Hello");
}

fn say_goodbye(name: &str) {
    println!("Goodbye {}", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_function() {
    say_hello();
    say_goodbye("Yoga");
    let result = add(10, 20);
    println!("Result: {}", result);
}
