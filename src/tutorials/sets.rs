use std::collections::HashSet;

#[test]
fn test_sets() {
    let mut books = HashSet::new();

    books.insert("Harry Potter");
    books.insert("Lord of the Rings");
    books.insert("Harry Potter"); // Duplicate

    println!("Books: {:?}", books);
}
