use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {   
        let mut buf = String::new();

        println!("Guess a number between 1 and 100: ");

        if let Err(e) = std::io::stdin().read_line(&mut buf) {
            println!("An error occurred: {}", e);
            continue;
        }

        let guess: u32 = match buf.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        let guess = match guessing_game_v2::validator::check_guess(guess) {
            Ok(value) => value,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You guessed the correct number!");
                break;
            }
        }
    }
}
