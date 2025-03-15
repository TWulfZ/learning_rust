use rand::Rng;
use std::{
    cmp::Ordering,
    io::{self},
};

// fn main() {
//     println!("✨ Adivina el numero! ✨");

//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     let mut win: bool = false;

//     println!("Ingrese un numero.");

//     while !win {
//         let guess = input_number();

//         if guess == secret_number {
//             println!("Ganaste! ✨\nEl numero era: {}", secret_number);
//             win = true;
//         }

//         if guess < secret_number {
//             println!("Bajo!");
//         } else {
//             println!("Demasiado!");
//         }
//     }
// }

// fn input_number() -> u32 {
//     let mut input = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Lectura fallida");

//     return input.trim().parse().expect("Porfavor ingres un numero valido");
// }

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Please input your guess.");
    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
