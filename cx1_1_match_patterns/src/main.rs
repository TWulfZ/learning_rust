fn main() {
    let int = 42;
    println!("Match Patterns");

    // Wildcard
    println!("\n--- Wildcard `_` ---");
    match int {
        _ => println!("?"),
    }

    // Literals
    println!("\n--- Literals `value` ---");
    match int {
        0 => println!("Zero"),
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("?"),
    }

    // Variables
    println!("\n--- Variables `var` ---");
    match int {
        0 => println!("0"),
        1 => println!("0"),
        var => println!("{var}"),
    }

    let tuple = (4, 2);

    // Tuples
    println!("\n--- Tuples `(4, 2)` ---");
    match tuple {
        (0, 0) => println!("(0, 0)"),
        (_, 0) => println!("(_, 0)"),
        (4, n) => println!("(4, {n})"),
        _ => println!("?"),
    }

    struct Point {
        x: u8,
        y: u8,
        z: u8,
    }

    let point = Point { x: 0, y: 1, z: 2 };

    // Structs
    println!("\n--- Structs `Point {{ x: 0, y: 1, z: 2 }}` ---");
    match point {
        Point { x: 1, y: 0, z: 2 } => println!("Point: (1, 0, 2)"),
        Point { x: 1, y: _, z: 2 } => println!("Point: (1, ?, 2)"),
        Point { x: 0, z: 2, .. } => println!("Point: (0, ?, 2)"),
        Point { x, y, .. } => println!("Point: ({x}, {y}, ?)"),
    }

    let array = [1, 2, 3];

    // Arrays
    println!("\n--- Arrays ---");
    match array {
        [_, 0, _] => println!("[_, 0, _]"),
        [a, 3, c] => println!("[{a}, 3, {c}]"),
        [a, b, c] => println!("[{a}, {b}, {c}]"),
    }

    let slice = &[1, 2, 3, 4, 5][0..4];

    // Slices
    println!("\n--- Slices ---");
    match slice {
        [_, 1, _, _] => println!("[?, 1, ?, ?]"),
        [a, 0, c, d] => println!("[{a}, 0, {c}, {d}]"),
        [first, .., last] => println!("[{first}, ..., {last}]"),
        [_] => println!("[?]"),
        [] => println!("[]"),
    }

    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: "Wulf".to_string(),
        age: 42,
    };

    // References 
    println!("\n--- References ---");
    match person {
        Person { ref name, age } => println!("Name: {name}, age: {age}"),
    }

    println!("Name: {}, age: {}", person.name, person.age);

    // Match guard
    println!("\n--- Match Guard ---");
    match int {
        x if x > 40 => println!("{x} > 40"),
        x => println!("{x}"),
    }

    // Range
    println!("\n--- Range ---");
    match int  {
        30..=40 => println!("30 <= {int} <= 40"),
        50.. => println!("{int} > 50"),
        _ => println!("?"),
    }

    // Or
    println!("\n--- Or ---");
    match int {
        41 | 42 => println!("68 or 69"),
        _ => println!("?"),
    }

    // With var
    match int {
        x @ 41 | x @ 43 => println!("{x}"),
        x @ 30..=50 => println!("{x}"),
        _ => println!("?"),
    }

    #[allow(dead_code)]
    enum Token {
        Digit(u8),
        Char(char),
        Number { signed: bool, literal: &'static str },
    }

    let token = Token::Digit(9);
    match token {
        Token::Digit(n) => println!("Digit: {n}"),
        Token::Char(c) => println!("Char: {c}"),
        Token::Number { literal, .. } => println!("Number: {literal}"),
    }
}
