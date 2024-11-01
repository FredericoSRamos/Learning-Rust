use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the number guessing game!\n");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {    
        println!("Guess a number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read the number");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("\nYou guessed the number {guess}!");

        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("This number is too small!\n"),
            Ordering::Greater => println!("This number is too large!\n"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
        }
    }
}