// 1- Each value in Rust has a variable that's its owner.
// 2- There can be only one owner at a time.
// 3- When the owner goes out of scope, the value will be dropped.

// Examples: Each value in Rust has a variable tha'ts its owner.
// fn main() {
//     let s1 = String::from("Rust");
//     let s2 = s1;
//     let len = calculate_lenght(&s2);
//     println!("Lenght of \"{}\" is {}", s2, len);
// }

fn calculate_lenght(s: &String) -> usize {
    s.len()
}

// 2- Therew can be only one owner at a time.
// fn main() {
//     let s1 = String::from("RUST");
//     let s2 = s1;

//     println!("{}", s2);
// }

// 3- When the owner goes out of scope, the value will be dropped.
fn main() {
    let s1 = String::from("RUST");
    let len = calculate_lenght(&s1);
    println!("Lenght of \"{}\" is {}", s1, len);
    printLost(&s1);
}

fn printLost(s: &String) {
    //println!("{}", s1);
}