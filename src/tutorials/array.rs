#[test]
fn test_array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Array: {:?}", array);

    let a = array[0];
    let b = array[1];
    println!("a: {}, b: {}", a, b);

    // Mutable array
    let mut mut_array = [1, 2, 3];
    mut_array[0] = 10;
    println!("Mut Array: {:?}", mut_array);

    // Length
    println!("Length: {}", array.len());
}
