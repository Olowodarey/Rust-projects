use std::io;
use rand::Rng;
use std::cmp::Ordering;     // use to check numbers greater than and less than 


fn main() {
    println!("Welcome to the Guessing Game");
    println!("i am thinking of a number between 1 and 100. can you guess it?");

    // generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);


    loop {
        println!("Pls input your guess");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Pls enter a valid number");
                continue;
            }
        };


        println!("you guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try again"),
            Ordering::Greater => println!("Too big! Try again"),
            Ordering::Equal => {
                println!("Congratulations! you have guessed the number");
                break;
            }
        }
    }
}