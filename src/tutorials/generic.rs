struct Point<T> {
    x: T,
    y: T,
}

#[test]
fn test_generic() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("Integer: {}, {}", integer.x, integer.y);
    println!("Float: {}, {}", float.x, float.y);
}
