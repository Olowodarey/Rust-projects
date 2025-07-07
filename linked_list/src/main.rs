use std::io::{self, Write};

fn main () {
    loop {
        println!("\n📝 Text File CRUD Menu:");
        println!("1. Create (overwrite)");
        println!("2. Read");
        println!("3. Update line");
        println!("4. Delete line");
        println!("5. Exit");
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}