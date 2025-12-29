#[test]
fn test_memory_management() {
    // Rust uses Ownership, Borrowing, and Lifetimes for memory management.
    // Stack vs Heap
    
    // Stack (Fixed size)
    let a = 10; 
    let b = 20;
    println!("Stack: {}, {}", a, b);

    // Heap (Dynamic size)
    let s = String::from("Hello");
    println!("Heap: {}", s);
}
