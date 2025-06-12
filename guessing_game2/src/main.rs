use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Welcome to the guessing game ");

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        let mut attempts = 0;

        println!("\nI'm thinking of a number between 1 and 100, can you guess it?");

        loop {
            println!("\nPlease input your guess:");

            let mut guess = String::new();
            io::stdin().read_line(&mut guess).expect("Failed to read input");


            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("⚠️ Please enter a valid number!");
                    continue;
                }
            };

            attempts += 1;

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("🎉 Congratulations! You guessed the number in {} tries.", attempts);
                    break;
                }
            }
        }

            // Ask if the user wants to play again
            println!("\nWould you like to play again? (y/n):");

            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read input");
    
            if choice.trim().to_lowercase() != "y" {
                println!("👋 Thanks for playing! Goodbye.");
                break;
            }
    }
}