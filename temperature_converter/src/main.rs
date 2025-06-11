use std::io; // this provide input modal for users 

fn main() {
    println!(" Temperature Converter");
    println!(" 1: Celsius to Fahrenheit");
    println!(" 2: Fahrenheit to Celsius  ");
    println!(" 3: Kelvin to Celsius  ");
    println!(" 4: Celsius to Kelvin   ");
    println!(" please select an option (1 , 2 , 3, 4) :");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("failed to read input");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("invalid choice. pls enter 1 or 2.");
            return;
        }
    };

    if choice == 1 {
        celsuius_to_fahrenheit();
    } else if choice == 2 {
        fahrenheit_to_celsius();
    } else if choice == 3 {
        kelvin_to_celsius();
    } else if choice == 4 {
        celsius_to_kelvin();
    } else {
        println!("invalid choice. Please select 1 , 2 or 3.");
    }
}

fn celsuius_to_fahrenheit() {
    println!("Enter temperature in celcsuis:");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("invalid choice. pls enter 1 or 2.");
            return;
        }
    };

    let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
    println!("{:.2}°C is {:.2}°F", temp, fahrenheit);
}

fn fahrenheit_to_celsius() {
    println!("Enter temperature in fahrenheit:");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("invalid input. pls enter a valid number.");
            return;
        }
    };

    let celsius = (temp - 32.0) * 5.0 / 9.0;
    println!("{:.2}°C is {:.2}°F", temp, celsius);
}

fn kelvin_to_celsius() {
    println!("Enter the temperature in Kelvin");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("invalid input. pls enter a valid number.");
            return;
        }
    };

    let kelvin = temp - 273.15;
    println!("{:.2}°K is {:.2}°C", temp, kelvin);
}

fn celsius_to_kelvin() {
    println!("Enter the temperature in Celsius");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("invalid input. pls enter a valid number.");
            return;
        }
    };

    let celsius = temp + 273.15;
    println!("{:.2}°K is {:.2}°C", temp, celsius);
}
