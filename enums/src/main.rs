use std::i32;

#[allow(unused)]

#[derive(Debug)]
struct Ip(u8, u8, u8, u8);

#[derive(Debug)]
enum IpAddrType { // Enums can have values associated with each option
    V4(Ip),
    V6(Ip),
}

enum Message { // Values store different data, but are all under the Message type
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum MatchResult {
    Loss(String),
    Draw(String),
    Win(String),
}

fn main() {
    let localhost = IpAddrType::V4(Ip(127, 0, 0, 1));
    println!("{:?}", localhost);
    
    //OPTION Enum:
    /* 
        Useful for dealing with optional values
    enum Option<T> {
        Some(T),
        None
    }
    */

    let mut some_url: Option<String> = Some(String::from("http://url.com"));
    println!("{:?}", some_url);

    some_url = None;
    println!("{:?}", some_url);

    let x = 10;
    let y = Option::Some(20);
    
    //let sum = x + y; -> unable to add value to 'raw' optional
    let forced_sum = x + y.unwrap(); // forces unwrap of optional value, panics if it's set to None, equivalent to Swift's '!'
    let default_sum = x + y.unwrap_or(0); // sets default value if None, equivalent to Swift's '??'

    get_points_for_match(MatchResult::Draw(String::from("Time A")));

    check_message_action(Message::Write(String::from("hello world!")));
    check_message_action(Message::Quit);

    let some_value = Some(10);
    
    match some_value {
        Some(10) => println!("Value is equal to 10"), // Use match to check if variable is equal to specific value
        _ => println!("Value is different from 10")
    }

    if let Some(10) = some_value { // Use if let to check if variable is equal to specific value
        println!("Value is equal to 10");
    }

    let none_value: Option<i8> = None;

    if let Some(i) = none_value { // Use if let to check if value exists
        println!("Value is {}", i);
    } else {
        println!("Value is None");
    }
}

fn get_points_for_match(result: MatchResult) -> u8 {
    let points: u8;
    
    match result { // Similar to 'switch'
        MatchResult::Loss(opponent) => {
            points = 0;
            println!("Lost to {}, won {} points.", opponent, points);
            points
        },
        MatchResult::Draw(opponent) => {
            points = 1;
            println!("Drew with {}, won {} point.", opponent, points);
            points
        },
        MatchResult::Win(opponent) => {
            points = 3;
            println!("Won againts {}, won {} points.", opponent, points);
            points
        },
    }
}

fn check_message_action(message: Message) {
    match message {
        Message::Write(body) => {
            println!("Sent message with body:\n{}\n---------------", body);
        }
        _ => { // Sets a default action in the match statement
            println!("Other action was called.");
        }
    }
}
