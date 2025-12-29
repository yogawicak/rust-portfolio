mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use self::front_of_house::hosting;

#[test]
fn test_module() {
    hosting::add_to_waitlist();
    println!("Module test passed");
}
