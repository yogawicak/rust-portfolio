# Rust Concept
## 1. Ownership

### The 3 Rules of Ownership
1. Each value in Rust has a variable that’s called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

### Term Of Ownership and Borrowing
Think of "Borrowing" like lending a book to a friend.
- Ownership ( T ): You give the book to your friend as a gift. It's theirs now. You can't read it anymore. If they lose it, it's gone.
- Borrowing / Referencing ( &T ): You let your friend read the book, but you still own it. They must give it back to you. While they have it, they can look at it, but they can't destroy it or give it away to someone else forever.
- Borrowing = Referencing

### Ownership Function
- Passing ownership: When you pass a value to a function, the ownership moves to the function.
- Returning ownership: When a function returns a value, the ownership moves back to the caller.
- Borrowing: You can borrow a value from a function without taking ownership.
- If data type is stack-allocated, it will be copied when passed to a function. 
- If data type is heap-allocated, it will be moved when passed to a function.

### Term Of Reference
- Reference: A pointer to a value that is stored in memory.
- Borrowing: When you borrow a value, you are creating a reference to that value.
- Mutable borrowing: When you borrow a value mutably, you are creating a reference to that value that allows you to modify it.

### Problem Struct Update Syntax
- When you update a struct, you can use the update syntax to copy the values from one struct to another.
- But, if you want to update only some fields, you need to use the update syntax with the .. operator.
- If field is heap-allocated, you need to use the update syntax with the .. operator to copy the value.

### Stack and Heap
- Stack: Stores data with a known, fixed size at compile time.
- Heap: Stores data with an unknown size at compile time.

### Stack Data Types
- Primitive types: i32, u64, f64, bool, char
- Arrays with fixed size: [i32; 5]
- Tuples: (i32, bool, f64)
- Structs (if all fields are stack-allocated)
- Enums (sized to fit their largest variant)
- References: &T, &mut T (the pointer itself, not what it points to)

### Heap Data Types
String: growable text (actual text data on heap)
Vec<T>: growable array/list
Box<T>: heap-allocated single value
HashMap, HashSet: collections
Rc<T>, Arc<T>: reference-counted pointers
Trait objects: Box<dyn Trait>


