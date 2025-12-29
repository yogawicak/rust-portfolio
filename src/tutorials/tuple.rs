#[test]
fn test_tuple() {
    // data type fix
    let tuple: (i32, f64, bool) = (10, 10.5, true);

    println!("Tuple: {:?}", tuple);

    // Destructuring
    let (a, b, c) = tuple;
    println!("a: {}, b: {}, c: {}", a, b, c);

    // Index access
    println!("Index 0: {}", tuple.0);
    println!("Index 1: {}", tuple.1);
    println!("Index 2: {}", tuple.2);

    // mutable destructuring
    let mut tuple = (10, 10.5, true);
    let (a, b, c) = tuple;
    println!("a: {}, b: {}, c: {}", a, b, c);

    // mutable index access
    tuple.0 = 20;
    println!("Index 0: {}", tuple.0);
}
