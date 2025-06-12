use std::io;

fn main() {
    println!("Palindrome checker");
    println!("Enter a string to check if its a palindrome");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");


    let cleaned_input = clean_string(&input);

    if cleaned_input.is_empty() {
        println!("pls enter a valid non empty string");
        return;
    }

    if is_palindrome(&cleaned_input) {
        println!("'{}' is a palindrome!", input.trim());

    }else {
        println!("'{}' is not a palindrome", input.trim());
    }




}

// cleans a strings: removes non-alphanumeric characters and convert to lowercase 
fn clean_string(input: &str) -> String {
    input.chars() // iterate over each character
    .filter(|c|  c.is_alphanumeric()) // convert to lowercase
    .collect::<String>() // collect into a new string
}

// checks if a cleaned string is a palindrome.
fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}