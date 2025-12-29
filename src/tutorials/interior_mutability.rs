use std::cell::RefCell;

#[test]
fn test_interior_mutability() {
    let x = RefCell::new(5);

    let y = x.borrow();
    println!("Value: {}", y);
    // drop(y); // Need to drop borrow before borrowing mutably if in same scope, but here it's fine if we don't use y later?
    // Actually, RefCell panics at runtime if rules are violated.

    drop(y);

    let mut z = x.borrow_mut();
    *z += 1;
    println!("Value: {}", z);
}
