use std::rc::Rc;

#[test]
fn test_multiple_ownership() {
    let a = Rc::new(String::from("Hello"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
