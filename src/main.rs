use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the Guessing Game!");
    let my_secret_number = rand::thread_rng().gen_range(1,101);
    play_game(my_secret_number);
}

fn play_game(secret_number: u32) {
    let mut lo_num = 1;
    let mut hi_num = 100;

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
                println!(", You win!");
                break;
            }
        }
    }


}
