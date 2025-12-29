static HELLO: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

#[test]
fn test_static_variable() {
    println!("name is: {}", HELLO);

    add_to_counter(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
