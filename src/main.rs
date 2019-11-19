use std::io;
use std::cmp::Ordering;
use rand::Rng;

const LO_VAL_INIT: u32 = 1;
const HI_VAL_INIT: u32 = 100;

fn main() {
    println!("Welcome to the Guessing Game!\nA guess the number game implemented in Rust");
    let my_secret_number = rand::thread_rng().gen_range(LO_VAL_INIT,HI_VAL_INIT+1); // +1 for bounds
    play_game(my_secret_number);
}

fn play_game(secret_number: u32) {
    let mut guess_count = 0;
    let mut lo_num = LO_VAL_INIT;
    let mut hi_num = HI_VAL_INIT;

    loop {
        println!("Enter your guess ({} to {}):", lo_num, hi_num);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Read Line failed?");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("Your guess was {}", guess);
        // Keep track of the number of guesses taken
        guess_count = guess_count + 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!(", too small!");
                if guess > lo_num {
                    lo_num = guess;
                }
            },
            Ordering::Greater => {
                println!(", too large!");
                if guess < hi_num {
                    hi_num = guess;
                }
            },
            Ordering::Equal => {
                println!(", You win!\n(It took you {} guesses)", guess_count);
                break;
            }
        }
    }
}
