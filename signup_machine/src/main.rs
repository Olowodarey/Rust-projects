use std::{
    io::{self, Write},
    os::linux::raw::stat,
};

fn main() {
    println!("🔄 State Machine: Signup Wizard");

    let mut state = State::Start;

    loop {
        match state {
            State::Start => {
                println!("Welcome! Let's begin your signup.");
                state = State::Entername;
            }

            State::Entername => {
                let name = input("Enter your name");
                if name.is_empty() {
                    println!("❌ Name cannot be empty.");
                } else {
                    state = State::EnterEmail(name);
                }
            }

            State::EnterEmail(ref name) => {
                let email = input("Enter your email");
                if email.contains("@") {
                    state = State::Confirm {
                        name: name.to_string(),
                        email,
                    };
                } else {
                    println!("❌ Invalid email format.");
                }
            }

            State::Confirm { name, email } => {
                println!("✅ Confirm your info:");
                println!("Name: {}", name);
                println!("Email: {}", email);

                let confirm = input("Is this correct? (yes/no): ");
                state = match confirm.as_str() {
                    "yes" => State::Complete,
                    "no" => State::Entername,
                    _ => {
                        println!("❌ Invalid choice.");
                        State::Confirm { name, email }
                    }
                }
            }

            State::Complete => {
                println!("🎉 Signup complete!");
                break;
            }
        }
    }
}

enum State {
    Start,
    Entername,
    EnterEmail(String),
    Confirm { name: String, email: String },
    Complete,
}

fn input(promot: &str) -> String {
    print!("{}", promot);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

// Controlled flow:
// Instead of using nested if/else blocks or function pointers, the state machine approach with match ensures all logic paths are centralized and   handled in one place.
