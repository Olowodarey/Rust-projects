// BMI = weight(kgs)/height^2(m^2)
// BMI < 18.5UW
// BMI 18.5 - 24.9 is nutral
// BMI 25 - 29.9 is over weight
// BMI >= 30 Obesity

use std::io;

fn main() {
    println!("BMI calculator");
    println!("Pls enter your weight in kilogram (kg):");

    let weight = match get_input_as_f64() {
        Some(value) => value,
        None => {
            println!("invalid input for weight. pls enter a valid input.");
            return;
        }
    };

    println!("pls enter your height in meters (m):");

    let height = match get_input_as_f64() {
        Some(value) => value,
        None => {
            println!("invalid input for height. pls enter a valid input.");
            return;
        }
    };

    if height == 0.0 {
        println!("Height cannot be zero");
        return;
    }

    let bmi = calculate_bmi(weight, height);
    println!("your BMI is: {:.2}", bmi);

    let category = classify_bmi(bmi);
    println!("BMI Category: {}", category);
}

fn get_input_as_f64() -> Option<f64> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim().parse::<f64>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}

fn classify_bmi(bmi: f64) -> &'static str {
    if bmi < 18.5 {
        "underweight"
    } else if bmi >= 18.5 && bmi <= 24.9 {
        "normal weight"
    } else if bmi >= 25.0 && bmi <= 29.9 {
        "overweight"
    } else {
        "Obesity"
    }
}
