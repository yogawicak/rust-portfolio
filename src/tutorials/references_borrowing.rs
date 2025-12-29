#[test]
fn test_references_borrowing() {
    let mut s1 = String::from("Hello");
    
    // Immutable Reference
    let r1 = &s1;
    let r2 = &s1;
    println!("{}, {}", r1, r2);

    // Mutable Reference
    // let r3 = &mut s1; // Error: cannot borrow as mutable because it is also borrowed as immutable
    
    change_value(&mut s1);
    println!("{}", s1);
}

fn change_value(s: &mut String) {
    s.push_str(", World");
}
