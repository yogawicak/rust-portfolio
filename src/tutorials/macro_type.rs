macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

#[test]
fn test_macro() {
    say_hello!();
}
