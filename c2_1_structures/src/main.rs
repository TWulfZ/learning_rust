struct Person {
    name: String,
    age: u8,
}

struct Pair(i32, f32);

struct Point {
    x: i32,
    y: i32,
}

struct Reactangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let peter = Person {
        age: 27,
        name: String::from("Peter"),
    };
    
    let point = Point{ x: 10, y: 20 };
    let point2 = Point{ x: 30, y: 40 };

    let rect = Reactangle {
        top_left: point,
        bottom_right: point2,
    };

    let pair = Pair(1,1.0);
    

    println!("Name: {}, age: {}", peter.name, peter.age);
    println!("Rect: ({}, {}) to ({}, {})", rect.top_left.x, rect.top_left.y, rect.bottom_right.x, rect.bottom_right.y);

    println!("Pair contains {:?} and {:?}", pair.0, pair.1);

    println!("Name: {}", peter.name);
}
