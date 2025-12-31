fn print_number(number: i32) {
    println!("Number: {}", number);
}

fn print_string(name: String) {
    println!("Name: {}", name);
}

fn print_string_borrow(name: &String) {
    println!("Name: {}", name);
}

fn full_name(first_name: &str, last_name: &str) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_ownership_function() {
    let number = 10;
    print_number(number); // Copy trait
    println!("Number after: {}", number);

    let name = String::from("Yoga");
    print_string(name);
    // println!("Name after: {}", name); // Error: value moved

    let name2: String = String::from("Eko");
    print_string_borrow(&name2);
    println!("Name2 after: {}", name2);
}

#[test]
fn test_return_ownership() {
    let name = String::from("Yoga");
    let full_name = full_name(&name, "Punya");

    println!("Full name: {}", full_name);
}
