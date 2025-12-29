#[test]
fn test_collection() {
    // Vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("Vector: {:?}", v);

    let v2 = vec![1, 2, 3];
    println!("Vector 2: {:?}", v2);
}
