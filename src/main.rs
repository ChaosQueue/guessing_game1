use std::io;

fn main() {
    println!("Welcome to the Guessing Game!");
    println!("Enter your guess (1-100):");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Read Line failed?");

    println!("Your guess was {}", guess);
}
