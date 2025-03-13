fn main() {
    // array - se almacena en el stack
    println!("\n--- Array [i32; 3]---");
    let mut a = [1, 2, 3];
    println!("len: {}, ptr: {:p}", a.len(), a.as_ptr());
    println!("a[1] == {}", a[1]);
    a[1] = 0;
    println!("a[1] == {}", a[1]);
    println!("ptr: {:p}", a.as_ptr());

    // Vector - parte se almacena en el stack y parte en el heap
    // La lenght se almacena en el stack 🔋 y la capacidad se almacena en el heap 🚃 (memoria dinamica)
    println!("\n--- Vector Vec<i32>---");
    let mut v = Vec::new();
    println!("len: {}, capacity: {}, ptr: {:p}", v.len(), v.capacity(), v.as_ptr());

    v.push(1);
    v.push(2);
    v.push(3);

    println!("v[1] == {}", v[1]);
    v[1] = 0;
    println!("v[1] == {}", v[1]);
    println!("len: {}, capacity: {}, ptr: {:p}", v.len(), v.capacity(), v.as_ptr());
    v.push(4);
    v.push(5);
    v.push(6);
    println!("len: {}, capacity: {}, ptr: {:p}", v.len(), v.capacity(), v.as_ptr());
    v.pop();
    println!("len: {}, capacity: {}, ptr: {:p}", v.len(), v.capacity(), v.as_ptr());
    v.drain(0..);
    println!("len: {}, capacity: {}, ptr: {:p}", v.len(), v.capacity(), v.as_ptr());

    // &str - parte se almacena en el stack y parte en el binario
    println!("\n--- &str ---");
    let s = "Hola, 🦀";
    println!("len: {}, chars: {}, ptr: {:p}", s.len(), s.chars().count(), s.as_ptr());
    println!("Datatype of s: {}", std::any::type_name_of_val(&s));

    // s = "Hola, 🦀!!";
    // println!("len: {}, chars: {}, ptr: {:p}", s.len(), s.chars().count(), s.as_ptr());
    // println!("Datatype of s: {}", std::any::type_name_of_val(&s));

    // String - parte se almacena en el stack y parte en el heap
    println!("\n--- String ---");
    let mut s = String::from("Hola, 🦀");
    println!("len: {}, chars: {}, ptr: {:p}", s.len(), s.chars().count(), s.as_ptr());

    s.push('🦀');
    s.push_str("!!!");
    s.pop();
    println!("{s}");
    println!("len: {}, chars: {}, ptr: {:p}", s.len(), s.chars().count(), s.as_ptr());

    // Box - parte se almacena en el stack y parte en el heap
    println!("\n--- Box<[i32; 3]> ---");
    let mut b = Box::new([1, 2, 3]);
    println!("len: {}, ptr: {:p}", b.len(), b.as_ptr());
    println!("b[1] == {}", b[1]);
    b[1] = 0;
    println!("b[1] == {}", b[1]);
    println!("len: {}, ptr: {:p}", b.len(), b.as_ptr());
}
