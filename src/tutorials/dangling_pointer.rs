#[test]
fn test_dangling_pointer() {
    // Attempting to create a dangling pointer
    // let reference_to_nothing = dangle(); // This will cause a compile-time error

    // The compiler error would be:
    // error[E0106]: missing lifetime specifier
    // or if we try to force it: "this function's return type contains a borrowed value, but there is no value for it to be borrowed from"

    // Correct way: Return ownership of the data
    let string_value = no_dangle();
    println!("The string is: {}", string_value);
}

// This function would fail to compile because it tries to return a reference to a value 
// that is dropped when the function ends.
/*
fn dangle() -> &String {
    let s = String::from("hello");
    &s // s is dropped here, so the reference would point to invalid memory
}
*/

// Correct implementation: Return the String directly (move ownership out)
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
