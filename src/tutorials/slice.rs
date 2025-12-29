#[test]
fn test_slice() {
    let array = [1, 2, 3, 4, 5];
    let slice = &array[1..3]; // [2, 3]

    println!("Slice: {:?}", slice);

    let slice2 = &array[..3]; // [1, 2, 3]
    println!("Slice2: {:?}", slice2);

    let slice3 = &array[2..]; // [3, 4, 5]
    println!("Slice3: {:?}", slice3);
}
