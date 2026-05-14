/*
Challenge Questions:

    Modify the code to add a new variant to the `FileSize` enum, representing a Terabyte size. Update the `format_size` function to handle the new variant 
    accordingly. Test the program by invoking the `format_size` function with a file size value representing a Terabyte.

    Extend the program by implementing a function that takes a file size in bytes as input and returns the file size representation in the largest possible unit. 
    For example, if the input is 2500 bytes, the function should return "2.44 KB". Invoke this function with different file size values and print the results 
    to ensure the correct conversion.

    Create a function parse_size(text: &str) -> Option<u64> that receives strings like "2 KB", "5 MB" or "1 GB" and returns the equivalent size in bytes.
*/

use std::collections::HashMap;


enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabytes(f64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
        _ => FileSize::Terabytes(size as f64 / 1_000_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
        FileSize::Terabytes(tb) => format!("{:.2} TB", tb),
    }
}

fn get_file_size(size: u64) -> String {
    let block : f64 = 1024.0;

    let filesize = match size {
        0..=1024 => FileSize::Bytes(size),
        1025..=1_048_576 => FileSize::Kilobytes(size as f64 / block.powf(1.0) ),
        1_048_577..=1_073_741_824 => FileSize::Megabytes(size as f64 / block.powf(2.0)),
        1_073_741_825..=1_099_511_627_776 => FileSize::Gigabytes(size as f64 / block.powf(3.0)),
        _ => FileSize::Terabytes(size as f64 / block.powf(4.0)),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
        FileSize::Terabytes(tb) => format!("{:.2} TB", tb),
    }
}

fn parse_size(size: &str) -> Option<f64>{
    let block : f64 = 1024.0;

    let unit_size = HashMap::from([
        ("KB", block.powf(1.0)),
        ("MB", block.powf(2.0)),
        ("GB", block.powf(3.0)),
        ("TB", block.powf(4.0)),
    ]);

    let unit = match size {
        s if s.contains("KB") => "KB",
        s if s.contains("MB") => "MB",
        s if s.contains("GB") => "GB",
        s if s.contains("TB") => "TB",
        _ => ""
    };

    let numeric_part: String = size
    .chars()
    .filter(|c| c.is_digit(10) || *c == '.')
    .collect();

    let n: f64 = numeric_part.parse().unwrap_or(0.0);

 
    Some(n * unit_size.get(unit).unwrap_or(&0.0) )
}

fn main() {
    let size : u64 = 6888008537399;
    let result = format_size(size);
    println!("{}", result);

    println!("{}",get_file_size(size));

    let size_str = String::from("6.7 MB");

    match parse_size(&size_str) {
        None => println!("Fail!"),
        Some(value) => println!("Success: {}", value),
    }
}
