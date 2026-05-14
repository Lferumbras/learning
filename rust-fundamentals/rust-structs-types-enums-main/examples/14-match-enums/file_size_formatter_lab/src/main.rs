/*
Challenge Questions:

    You are tasked with extending the application to allow a user to pass in a String representing size and unit, and then returning a debug representation of a 
    struct that shows all the different representations in KB, MB, and GB . 
    
    This is an example that takes an input and provides the output required:
    $ cargo run "24 mb"
    Sizes { bytes: "24000000 bytes", kilobytes: "24000 kilobytes", megabytes: "24 megabytes", gigabytes: "0 gigabytes" }

    Here are some things to consider if you are getting stuck:
    You'll need to split the input string to capture the number and the size. You can use the size (e.g. "kb") to match on how to process that number. 
    The struct will need to have the derive debug attribute to print it out
    Use impl to extend the struct to do the work on the struct for you
    Use the example code to assist you with some of the match statements and transformations needed

    Concepts Covered:
        Introduction to Rust: You will familiarize themselves with the provided source code and its components, including enums and structs.
        Enum usage: You will learn how to define and use an enum (FileSize) to represent different file size units.
        Struct usage: Explore the Sizes struct, its initialization, and how to compute and format the file sizes.
        Error handling: Modify the code to handle invalid input formats, such as missing or unrecognized size identifiers.
        String parsing and formatting: Gain experience in parsing string input to extract the size and identifier, and formatting strings to display the file 
        sizes in a human-readable format.
        Implementing the Debug trait: Learn how to derive and use the Debug trait to print debug information of the Sizes struct.
*/

use std::collections::HashMap;
use std::env;

static unit : f64 = 1024.0;

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String
}

impl Sizes {
    fn new() -> Self {
        Self {
            bytes: "0 bytes".to_string(),
            kilobytes: "0 kilobytes".to_string(),
            megabytes: "0 megabytes".to_string(),
            gigabytes: "0 gigabytes".to_string()
        }
    }

    fn parse_size(size: &str) -> Result<(f64,f64), String>{
        let size_up = size.to_uppercase();

        let first_letter_idx = size_up.find(|c: char| c.is_alphabetic())
        .ok_or("Nenhuma unidade de medida encontrada")?;

        println!("{}",first_letter_idx);

        let (numeric_part, unit_part) = size_up.split_at(first_letter_idx);

        let n = match unit_part.trim() {
            "GB" => 3.0,
            "MB" => 2.0,
            "KB" => 1.0,
            "B"  => 0.0,
            _    => return Err(format!("Unidade '{}' é inválida", unit_part)),
        };


        let value: f64 = numeric_part.trim().parse()
        .map_err(|_| "Número inválido na string")?;

        Ok((unit.powf(n) * value ,n))
        
    }

    fn set_formated_sizes(&mut self, size : &str){
        let  bites_size: f64;
        let  n : f64;

        match Self::parse_size(&size) {
            Ok(value) => (bites_size, n) = (value.0, value.1),
            Err(error) => panic!("Erro: {}", error),
        }

        if n >= 0.0 {
            self.bytes =  format!("{:.2} bytes", bites_size*unit.powf(0.0) ) ;
        }

        if n >= 1.0{
            self.kilobytes =  format!("{:.2} kilobytes", bites_size*unit.powf(1.0) ) ;
        }

        if n >= 2.0{
            self.megabytes =  format!("{:.2} megabytes", bites_size*unit.powf(2.0) ) ;
        }
        
        if n >= 3.0{
            self.gigabytes =  format!("{:.2} gigabytes", bites_size*unit.powf(3.0) ) ;
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Size is: {}.", args[1]);

    let mut size = Sizes::new();

    size.set_formated_sizes(&args[1]);

    println!("{:?}", size);
    
}
