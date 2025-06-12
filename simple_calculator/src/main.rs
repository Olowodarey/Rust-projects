use std::io;

fn main() {
    println!("Simple Calculator");
    println!("Available operations: +, -, *, /, &");
    println!("Enter your expression (e.g., 5 + 3):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let tokens: Vec<&str> = input.trim().split_whitespace().collect();

    if tokens.len() != 3 {
        println!("Invalid ionput. Pls follow the format: number operator number"); // this func we checking to make sure that the user input is equal yto 3 the number and the sign operator and number 
        return;
    }

    let num1: f64 = match tokens[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("invalid first number.");
            return;
        }
    };

    let operator = tokens[1];

    let num2: f64 = match tokens[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("invalid first number.");
            return;
        }
    };

    let result = match operator {
        "+" => add(num1, num2),
        "-" => subtract(num1, num2),
        "*" => multiply(num1, num2),
        "/" => divide(num1, num2),
        "%" => mod_num(num1, num2),
        _ => {
            println!(" Invalid operator. use +, -, *, or /.");
            return;
        }
    };

    println!("REsult: {:.2}", result)
}

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("cannot divde by zero");
        std::process::exit(1);
    }
    a / b
}

fn mod_num(a: f64, b: f64) -> f64 {
    a % b
}



// this project breaks after each operator make it to always work till it get stop using loop 