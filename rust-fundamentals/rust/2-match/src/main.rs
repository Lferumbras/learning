use std::io;

fn main() {
    println!("Please enter a greeting:");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    // use of match expression to pattern match against variable "name"
    match name.trim() {
        "Good Bye" => println!("Sorry to see you go."),
        "Hello" => println!("Hi, nice to meet you!"),
        _ => println!("I can't find a greeting, good bye."),
    }
}

use std::io;

fn main() {
    println!("Please enter a greeting:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    // use of match expression to pattern match against variable "name"
    match name.to_lowercase().trim() {
        "good bye" => println!("Sorry to see you go."),
        "hello" => println!("Hi, nice to meet you!"),
        "how are you?" => println!("I'm doing well!"),
        _ => println!("I can't find a greeting, good bye."),
    }
}

