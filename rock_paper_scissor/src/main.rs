use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to Rock-paper-scissor");
    println!(
        "Instructions: Enter 'rock', 'paper', or 'scissors'. Type 'quit'. Type 'quit' to exit."
    );

    loop {
        println!("\n make your choice");

        let user_choice = get_user_choice();
        if user_choice == "quit" {
            println!("Thanks for playing! Goodbye");
            break;
        }

        let computer_choice = get_computer_choice();
        println!("Computer chose: {}", computer_choice);

        match determine_winner(&user_choice, &computer_choice) {
            GameResult::Win => println!("You win"),
            GameResult::Lose => println!("You lose"),
            GameResult::Draw => println!("its a draw"),
        }
    }
}

fn get_user_choice() -> String {
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read inpet ");

    let choice = choice.trim().to_ascii_lowercase();
    match choice.as_str() {
        "rock" | "paper" | "scissor" | "quit" => choice,
        _ => {
            println!("Invalid choice. pls enter 'rock', 'paper', 'sissors or quit");
            get_user_choice()
        }
    }
}

fn get_computer_choice() -> String {
    let choices = ["rock", "paper", "scissor"];
    let index = rand::thread_rng().gen_range(0..choices.len());
    choices[index].to_string()
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

fn determine_winner(user: &str, computer: &str) -> GameResult {
    match (user, computer) {
        ("rock", "scissor") => GameResult::Win,
        ("paper", "rock") => GameResult::Win,
        ("scissors", "paper") => GameResult::Win,
        (a, b) if a == b => GameResult::Draw,
        _ => GameResult::Lose,
    }
}



// try to track scores 
// add game mode 