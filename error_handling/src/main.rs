use std::io::{self, Write};
use std::num::ParseFloatError;

fn main() {
    println!("Hello, world!");

    loop {
        println!("\n1. Add | 2. Divide | 3. Exit");
        let choice = input("choose an option:");

        match choice.as_str() {
            "1" => match parse_two_numbers() {
                Ok((a, b)) => println!("✅ Result: {} + {} = {}", a, b, a + b),
                Err(e) => eprintln!("❌ Error: {}", e),
            },

            "2" => match parse_two_numbers() {
                Ok((a, b)) => match divide(a, b) {
                    Ok(result) => println!("✅ Result: {} / {} = {}", a, b, result),
                    Err(e) => eprintln!("❌ Error: {}", e),
                },
                Err(e) => eprintln!("❌ Error: {}", e),
            },
            "3" => {
                println!("👋 Exiting.");
                break;
            }
            _ => println!("❌ Invalid choice."),
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

//parse two numbers with error handling

fn parse_two_numbers() -> Result<(f64, f64), ParseFloatError> {
    let a = input("Enter first number").parse::<f64>()?; //  tries to convert that String into a 64-bit floating-point number (f64).
    let b = input("enter second number").parse::<f64>()?; // The ? means:    If parsing succeeds, store it in a.    If parsing fails, immediately return the error from the function.

    Ok((a, b))
}

// divsion with custom error handling

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("cannot divde by zero".to_string())
    } else {
        Ok(a / b)
    }
}
