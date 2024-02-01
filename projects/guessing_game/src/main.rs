use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("GUESSING GAME\n=============");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut guess = String::new();

        println!("Enter your guess.");

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read user input from keyboard.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("You win! The secret number was {secret_number}.");
                break;
            }
        }
    }
}
