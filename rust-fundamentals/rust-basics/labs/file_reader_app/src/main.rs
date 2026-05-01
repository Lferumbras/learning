use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, path};
use std::io;

fn print_root_dir(){
    match env::current_dir() {
        Ok(path) => println!("O diretório atual é: {:?}", path.display()),
        Err(e) => println!("Erro ao obter o diretório: {:?}", e),
    }
}

fn get_root_path() -> String{
    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    //printa o diretorio mas do binario dentro de target
    println!("My path is {}.", args[0]);

    args[0].clone()
}

fn read_file(filename : &String) -> File{
    let path = env::current_dir().unwrap().join(filename.trim());
    println!("{:?}",path);

    let file = File::open(path);
    
    match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                }   
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    }
}

fn show_file_content(file : File){

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }

}

fn main() {

    //print_root_dir();

    let mut input = String::new();

    while input.trim() != "stop" {
        
        input.clear();

        println!("Please enter a word (type 'stop' to exit):");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        println!("You entered: {}", input);

        //let path = get_root_path();
        
        let file: File = read_file( &input);
        
        show_file_content(file);

        
    }

    /* 
    let reader = BufReader::new(file);
    for line in reader.lines(){
        println!(":{}",line.unwrap());
    }
    */

    /* */
}
