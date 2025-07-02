use std::{ io};

fn main () {
   loop {
    println!("Fibonacci Sequence Generator");
    println!("Enter the number of terms you want to generate:   or enter 'end' to quit");

    let mut input = String::new();
    io::stdin().read_line(&mut input ).expect("failed to read input");

    let input = input.trim();

    if input.eq_ignore_ascii_case("end") {
        println!("🥰🥰🥰Goodbye🥰🥰 ");
        break;
    }

    let num_terms = match  input.parse() {
       Ok(n) => n,
       Err(_)  => {
        println!("Invalid input. Please enter a number or 'end'.");
        continue;
       }      
    };

    if num_terms == 0 {
        println!("Number of terms must be greater than ")
    }

    let sequence = generate_fibonacci(num_terms);
    println!("Fibonacci Sequence ({} terms): {:?}", num_terms, sequence);
}
   }




// generate a fibonacci sequence up to n terms 
fn generate_fibonacci(n: u32) -> Vec<u64> {
    let mut sequence = Vec::new();

    if n >= 1 {
        sequence.push(0);  // first term here is pushing 0 
    }
    if n >= 2 {
        sequence.push(1); // second term here is pushing 1
    }

    for i in 2..n {
        let next = sequence[i as usize - 1] + sequence[i as usize -2];
        sequence.push(next);
    }

    sequence
}