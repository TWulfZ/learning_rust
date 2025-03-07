// Primitive data types
// int, float, bool, char

// Integer
// Rust has signet (+ and -) and unsigned integer
// integer (only+) tpyes of different sizes
// i8, i16, i32, i64, i128: Signed integers
// u8, u16, u32, u64, u128: Unsigned integers
fn main() {
    let x: i32 = -42;
    let y: u32 = 32;

    println!("Signed Integer {}", x);
    println!("Unsigned Integer {}", y);

// diff bet i32 (32 bits) and i64(64 bits)
// range :
// i32 ± 2,147,483,647
// i64: ± 9,223,372,036,854,775,807
    
    let e: i32 = 2_147_483_647;
    let i: i64 = 9_223_372_036_854_775_807;
    println!("Maximum value of i32 {}", e);
    println!("Maximum value of i64 {}", i);

// Floats [Floating Point Types]
    let pi: f64 = 3.141592653589793;
    let e: f64 = 2.71828;
    println!("Float: {}", pi);
    println!("Float: {}", e);

// Boolean Values [true or false]
    let is_snowing: bool = true;
    let is_raining: bool = false;

    println!("Is it snowing? {}", is_snowing);
    println!("Is it raining? {}", is_raining);

// Character [char]
    let a: char = 'a';
    let b: char = 'b';
    let c: char = 'c';
    println!("Characters: {}, {}, {}", a, b, c);
}