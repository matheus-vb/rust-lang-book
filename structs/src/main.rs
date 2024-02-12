#[allow(unused)]

#[derive(Debug)] //Allow User struct to be displayed in a print statement
struct User { // Simple struct
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

struct Point(f32, f32, f32); // Tuple struct

// Struct with method implementations
#[derive(Debug)]
struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn calculate_area(&self) -> f32 {
        self.height * self.width
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.height > other_rect.height && self.width > other_rect.width
    }

}

impl Rectangle {
    fn build_square(h: f32) -> Rectangle { // Associated function -> similar to static functions in OOP, can be called upon the type instead of 
        Rectangle { width: h, height: h } // an instance, and do not access specific instace related attributes
    }
}

fn main() {

    // STRUCTS
    let u1 = User {
        username: String::from("matheus"),
        email: String::from("matheus@email.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:#?}", u1);

    let u2 = build_user(String::from("joao"), String::from("joao@email.com"));
    println!("{:#?}", u2);

    let u3 = User {
        username: String::from("pedro"),
        email: String::from("pedro@email.com"),
        ..u1 // Build rest of structs attributes from another instance
    };
    println!("{:#?}", u3);

    // TUPLE STRUCTS
    let point_a = Point(0.3, 10.0, 45.8);
    let point_b = Point(1.0, 0.5, 52.0);

    let distance = distance_between_points(&point_a, &point_b);
    println!("Distance bewteen points: {}", distance);
    

    // STRUCT WITH METHODS
    let rect = Rectangle {
        width: 23.0,
        height: 18.0,
    };

    let area = rect.calculate_area();
    println!("Rect area: {}", area);

    let rect2 = Rectangle {
        width: 30.0,
        height: 50.0,
    };
    println!("Rect can hold rect2: {}", rect.can_hold(&rect2));

    // ASSOCIATED FUNCTIONS
    let square = Rectangle::build_square(11.8);
    println!("Square: {:#?}", square);

}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn distance_between_points(p1: &Point, p2: &Point) -> f64 {
    let d = (p2.0 - p1.0).powf(2.0) + (p2.1 - p1.1).powf(2.0) + (p2.2 - p1.2).powf(2.0);
    (d as f64).sqrt()
}
