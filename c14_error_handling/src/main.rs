// Disable warnings
#![allow(dead_code)]

use std::fs::File;
use std::io::{self, Read};
// Approach 1
// enum Option<T> {
//     Some(T),
//     None,
// }

// Aproach 2
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// Option<T>
fn divideOption(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 { None } else { Some(numerator / denominator) }
}

// Result<T, E>
fn divideResult(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("Cannot divide by 0"))
    } else {
        Ok(numerator / denominator)
    }
}

fn read_file_contents(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?; // Si falla, retorna Err(io::Error)
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // Si falla, retorna Err(io::Error)
    Ok(contents)
}

fn main() {
    let result = divideOption(36.0, 0.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by zero"),
    }

    match divideResult(60.0, 0.0) {
        Ok(x) => println!("Result: {x}"),
        Err(err) => println!("{err}"),
    }

    let result = divideResult(60.0, 0.0);
    println!("Result: {result:?}");

    // Using `?` operator to propagate errors
    match read_file_contents("helloworld.exe") {
        Ok(content) => println!("File contents: \"{content}\""),
        Err(err) => eprintln!("Error reading file: {err}"),
    }
}
