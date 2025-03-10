// Functions
// Entry point

fn hello_world() {
    println!("Hello, world!");
}

fn tell_height(height: u32) {
    println!("Height: {}", height);
}

fn human_id(name: &str, age: i32, height: f32) {
    print!("Name: {}\nAge: {}\nHeight: {}", name, age, height);
}

fn sum(a: i32, b: i32) -> i32 {
    a+b
}

fn main() {
    hello_world();
    tell_height(45);
    human_id("Alice", 21, 1.65);
    let x = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("Total: {}", x);
    println!("Sum: {}", sum(1, 2));
}

fn _hello() {

}
