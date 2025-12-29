#[test]
fn test_number() {
    let a: i32 = 10;
    let b: f64 = 10.5;
    let c: i8 = 10;
    
    println!("a: {}, b: {}, c: {}", a, b, c);

    // Number conversion
    let d = a as i64;
    println!("d: {}", d);
}
