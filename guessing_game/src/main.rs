extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    /*
    Guess should be of type u32 (unsigned 32-bit integer
    The match statement is used to handle different cases of parsing input.
    guess.trim().parse() trims any whitespace from the user input, then parse it into an int.
    If parsing succeeds, Ok(num) is returned, where num is the parsed integer.
    If parsing fails (for example, if the input is not a valid number), Err(_) is returned.
    without needing to repeat the comment symbol.
    */

    let guess: u32 = match guess.trim().parse() { 
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    println!("You guessed: {}", guess);
    

    match guess.cmp(&secret_number) {
        Ordering::Less  => println!("Too small!"),
        Ordering::Greater   => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}

}