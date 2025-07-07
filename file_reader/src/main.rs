use std::env::{self, Args}; // for reading command line arguments
use std::fs::File; // to open file 
use std::io::{self, BufRead, BufReader};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("❌ Usage: cargo run <file_path> [--lines] [--search <keyword>]");
        return;
    }


    let file_path = &args[1];
    let show_lines = args.contains(&"--lines".to_string()); // check if --lines is in the arguments, otherwise false.

    let keyword = if let Some(pos) = args.iter().position(|x| x == "--search") {
        args.get(pos + 1)
    } else {
        None
    };

    let file = match File::open(file_path) {  // here we try to open the file 
        Ok(f) => f, 
        Err(e) =>  { eprintln!("❌ Failed to open file: {}", e);
        return;
    }
    };

    let reader = BufReader::new(file);

    for (i, line ) in reader.lines().enumerate() {  // read the file line by line 
        let line = line.unwrap();
   

  

    let matched = keyword.map_or(true, |k| line.contains(k));

    if matched {
        if show_lines {
            println!("{}: {}", i + 1, line);
        } else {
            println!("{}", line);
        }
    }

}



}



