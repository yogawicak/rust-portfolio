#[test]
fn test_formatting() {
    let x = 5;
    let y = 10;

    println!("x = {}, y = {}", x, y);
    println!("{0}, {1}, {0}", x, y);
    println!("{name} is {age} years old", name = "Yoga", age = 20);
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);
    println!("Debug: {:?}", [1, 2, 3]);
}
