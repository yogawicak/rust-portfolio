#[test]
#[allow(dead_code)]
fn test_attributes() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);
}
