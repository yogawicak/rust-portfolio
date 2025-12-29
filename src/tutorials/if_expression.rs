#[test]
fn test_if_expression() {
    let value = 9;

    if value >= 8 {
        println!("Good");
    } else if value >= 5 {
        println!("Not Bad");
    } else {
        println!("Bad");
    }

    let result = if value >= 8 {
        "Good"
    } else if value >= 5 {
        "Not Bad"
    } else {
        "Bad"
    };

    println!("Result: {}", result);
}
