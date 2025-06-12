use std::env; // use for command line argument
use std::fs::File; // to open file
use std::io::Read; //to read file  

fn main() {
    // collect command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!(" Usage: cargo run <file_path>");
        return;
    }

    let file_path = &args[1];
    println!("Reading file: {}", file_path);

    // read the file contents
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening file: {}", err);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        println!("Error reading file: {}", err);
        return;
    }

    let world_count = count_words(&contents);
    println!("World Count: {}", world_count);

    let char_count = count_char(&contents);
    println!("characters  Count: {}", char_count);

    let char_lines = count_line(&contents);
    println!("lines  Count: {}", char_lines);
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_char(text: &str) -> usize {
    text.chars().count()
}

fn count_line(text: &str) -> usize {
    text.lines().count()
}

// add the charater count, and also line count
