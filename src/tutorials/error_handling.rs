use std::fs::File;
use std::io::ErrorKind;

#[test]
fn test_error_handling() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    
    // Result
    let result: Result<i32, String> = Ok(10);
    if let Ok(val) = result {
        println!("Value: {}", val);
    }
}
