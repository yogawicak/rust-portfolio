#[test]
fn test_comparison_operators() {
    let a = 10;
    let b = 20;

    println!("{} > {} = {}", a, b, a > b);
    println!("{} < {} = {}", a, b, a < b);
    println!("{} >= {} = {}", a, b, a >= b);
    println!("{} <= {} = {}", a, b, a <= b);
    println!("{} == {} = {}", a, b, a == b);
    println!("{} != {} = {}", a, b, a != b);
}
