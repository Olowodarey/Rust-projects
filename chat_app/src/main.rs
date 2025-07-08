use std::{fmt::format, io::{self, Write}};

fn main() {
    println!("💬 ChatBot CLI - Type 'exit' to quit");

    let mut history: Vec<String> = Vec::new();

    loop {
        let input = prompt("you:");
        if input.to_lowercase() == "exit" {
            println!("👋 Goodbye!");
            break;
        }

        history.push(format!("you: {}", input));   // using this format here so that it can save the bot reply to the history vector so that we can see the bot reply 
    
    
        let response = bot_reply(&input);
        println!("Bot: {}", response);
        history.push(format!("Bot: {}", response));

    
    }

    println!("\n🗒️ Chat History:");
    for line in history {
        println!("{}", line);
    }
}

fn bot_reply(message: &str) -> String {
    let msg = message.to_lowercase();

    if msg.contains("hello") {
        "Hi there!".to_string()
    } else if msg.contains("how are you") {
        "I'm just code, but I'm doing fine!".to_string()
    } else if msg.contains("rust") {
        "Rust is memory-safe and fearless!".to_string()
    } else {
        "I don't understand that yet.".to_string()
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf ).unwrap();
    buf.trim().to_string()
}
