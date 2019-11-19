use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the Guessing Game!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("(The secret number is: {})", secret_number);

    println!("Enter your guess (1-100):");

    let mut guess = String::new();
    let len = io::stdin().read_line(&mut guess)
        .expect("Read Line failed?");

    println!("Your guess was {} (len:{})", guess, len);
}
