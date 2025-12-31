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

#[test]
fn test_immutable_owner_mutable_borrow() {
    // 1. Immutable owner
    let s = String::from("Hello");

    // 2. Attempting to create a mutable reference
    // This line causes a compile-time error: "cannot borrow `s` as mutable, as it is not declared as mutable"
    // let r = &mut s;

    // println!("Reference: {}", r);

    // 3. Correct way: The owner must be mutable
    let mut s_mut = String::from("Hello");
    let r_mut = &mut s_mut;
    r_mut.push_str(", world");
    println!("Success: {}", r_mut);

    println!("Success2: {}", s_mut);
}

#[test]
fn test_mutable_reference_limits() {
    let mut s = String::from("Hello");

    // Scenario 1: Multiple mutable references (Not Allowed)
    // let r1 = &mut s;
    // let r2 = &mut s; // Error: cannot borrow `s` as mutable more than once at a time
    // println!("{}, {}", r1, r2);

    // Scenario 2: Mutable reference while immutable reference exists (Not Allowed)
    // let r1 = &s;
    // let r2 = &mut s; // Error: cannot borrow `s` as mutable because it is also borrowed as immutable
    // println!("{}, {}", r1, r2);

    // Scenario 3: Correct usage with scopes
    {
        let r1 = &mut s;
        r1.push_str(" World");
        // r1 goes out of scope here
    } // r1 is no longer valid

    let r2 = &mut s; // Allowed because r1 is gone
    r2.push_str("!");
    println!("{}", r2);
}
