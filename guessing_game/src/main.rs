use rand::Rng;
use std::cmp::{Ord, Ordering};
use std::io;
use colored::*;

fn main() {
    println!("Guess the number!");
    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Input your number below: ");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading input");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            },
        };
    
        println!("Your guessed: {}", guess);
    
        match guess.cmp(&secret) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You won!".green());
                break;
            }
        };
    }

    println!("The number was {}", secret);
}
