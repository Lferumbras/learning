fn main() {
    let mut i = 0;

    while i < 5 {
        println!("i = {}", i);

        i += 1;
    }
}



use std::io;

// This example is a useful application of `while` because it allows to continue
// asking for user input until the user types a specific word (in this case,
// "stop").
fn main() {
    let mut input = String::new();

    while input.trim() != "stop" {
        input.clear();

        println!("Please enter a word (type 'stop' to exit):");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        println!("You entered: {}", input);
    }

    println!("Goodbye!");
}



fn main() {
    // the for loop using a range. Note you can use also `(1..10)` or `(1..=10)`
    for i in 1..10 {
        println!("i = {}", i);
    }

    // for i in (1..=5).rev() {
    //     println!("{}", i);
    // }

    // let numbers = vec![1, 2, 3, 4, 5];
    // for n in numbers {
    //     println!("{}", n);
    // }
}


fn main() {
    for i in 1..=100 {
        if i % 2 == 0 {
            // Skip even numbers
            continue;
        }

        println!("i = {}", i);

        if i == 7 {
            // Exit loop when i is 7
            break;
        }
    }
}



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