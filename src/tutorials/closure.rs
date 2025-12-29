#[test]
fn test_closure() {
    let add_one = |x: i32| x + 1;
    let result = add_one(5);
    println!("Result: {}", result);

    let x = 10;
    let print_x = || println!("x: {}", x);
    print_x();
}
