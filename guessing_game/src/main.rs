use rand::Rng;
use std::cmp::{Ord, Ordering};
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Input your number below: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error reading input");

    let guess: u32 = guess.trim().parse().expect("Please type in a number!");

    println!("Your guessed: {}", guess);

    let secret = rand::thread_rng().gen_range(1, 101);

    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You won!")
    }

    println!("The number was {}", secret);
}
