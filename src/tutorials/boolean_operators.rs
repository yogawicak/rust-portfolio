#[test]
fn test_boolean_operators() {
    let a = true;
    let b = false;

    println!("{} && {} = {}", a, b, a && b);
    println!("{} || {} = {}", a, b, a || b);
    println!("!{} = {}", a, !a);
}
