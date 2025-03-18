#![allow(warnings)]

fn main() {
    // tuple
    let rect = (200, 500);

    // Struct
    struct Book {
        title: String,
        author: String,
        pages: u32,
        aviable: bool,
    }

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("twulfz"),
        email: String::from("twulfz@gmail.com"),
        sign_in_count: 1,
    };

    // Print struc
    println!(
        "User: {{ \n\tactive: {:?}, \n\tusername: {:?}, \n\temail: {:?}, \n\tsign_in_count: {:?}\n\t}}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    user1.email = String::from("twulfzqpro@twulfz");
    
    let user2 = User {
        email: String::from("twulfzqpro@twulfz"),
        ..user1
    };

    // Tuples Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let white = Color(255, 255, 255);
    let end = Point(100, 100, 100);
    
    // Unit-Like structs
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
