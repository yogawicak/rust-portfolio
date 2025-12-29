#[test]
fn test_variable_scope() {
    let yoga = 1;

    {
        println!("Inner yoga: {}", yoga);
        let eko = 2;
        println!("Inner eko: {}", eko);
    }

    println!("Outer yoga: {}", yoga);
    // println!("Outer eko: {}", eko); // Error because eko is out of scope
}
