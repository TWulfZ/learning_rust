// If Else [If expression] [ Else expression ]
fn main() {
    let age: u16 = 18;
    if age >= 18 {
        println!("You can drive a car");
    } else {
        println!("You can't drive a car");
    }

    // Multiple conditions
    let number= 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement (Rust does not have a ternary operator)
    let condition = true;
    // let number = if condition {5} else {6};
    let number = if condition {
        5
    } else {
       // "six"  Error: incompatible types: expected integer, found `&str`
       6
    };
    println!("The value of number is: {}", number);

}
