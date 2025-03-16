fn main() {
    // --- Simple loop ---
    // loop {
    //     println!("again!");
    // }

    // Return value from loop
    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {}", result);

    // --- Loop labels ---
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {}", count);
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {}", remaining);
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {}", count);

    // --- While loop ---
    // let a = [10,20,30,40,50,60,70,80,90,100];

    // let mut index=0;

    // while index < a.len() {
    //     println!("The value is: {}", a[index]);
    //     index += 1;
    // }

    // // --- For loop ---
    // let a = [10, 20, 30, 40, 50,80];

    // println!("a: {:?}", a.iter().enumerate());

    // for (index, element) in a.iter().enumerate() {
    //     println!("the index is: {}", index);
    //     println!("the value is: {}", element);
    // }

    // --- For loop with range ---
    for number in 1..5 {
        println!("{number}");
    }


    // .rev() reverses the iteration direction
    println!("--- Reverse ---");
    for number in (1..5).rev() {
        println!("{number}");
    }
    println!("liftoff");

    //
}
