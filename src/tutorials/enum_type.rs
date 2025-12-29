enum IpAddrKind {
    V4,
    V6,
}

#[test]
fn test_enum() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Just to use them
    match four {
        IpAddrKind::V4 => println!("V4"),
        IpAddrKind::V6 => println!("V6"),
    }
}
