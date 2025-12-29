type Kilometers = i32;

#[test]
fn test_type_alias() {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}
