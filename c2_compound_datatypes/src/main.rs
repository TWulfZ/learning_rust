// Compound data types
// arrays, tuples, slices, and strings (slice string)

fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number array: {:?}", numbers);
    //let mix = [1,2, "apple", true];
    //println!("Mixed array: {:?}", mix);
    let fruits: [&str; 3] = ["apple", "banana", "cherry"];
    println!("Fruits array: {:?}", fruits);
    println!("First fruit 1st element[0]: {}", fruits[0]);
    println!("Second fruit 2nd element[1]: {}", fruits[1]);
    println!("Third fruit 3rd element[2]: {}", fruits[2]);

    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human: {:?}", human);
    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("Mix tuple: {:?}", my_mix_tuple);

    // Slices: [1,2,3,4,5]
    let number_slice: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number slice: {:?}", number_slice);

    let animal_slices: &[&str] = &["Lion", "Tiger", "Elephant"];
    println!("Animal slice: {:?}", animal_slices);

    let book_slices: &[&String] = &[
        &"IT".to_string(),
        &"Harry Potter".to_string(),
        &"ZEN".to_string(),
    ];
    println!("Book slice: {:?}", book_slices);

    // Strings Vs String Slices (&str)
    // Strings [ growable, mutable, owned string type]
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah");
    println!("Cold Stones Say: {}", stone_cold);

    // B- &str (String Slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..8];
    println!("String Slice: {}", slice);
}
