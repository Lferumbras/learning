use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file(filename: &str) -> Result<File, io::Error> {
    File::open(filename)
}

fn show_file_content(file: File) -> Result<(), io::Error> {
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso correto:");
        eprintln!("cargo run -- <nome_do_arquivo>");
        return;
    }

    let filename = &args[1];

    match read_file(filename) {
        Ok(file) => {
            if let Err(error) = show_file_content(file) {
                eprintln!("Erro ao ler o conteúdo do arquivo: {}", error);
            }
        }
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => {
                    eprintln!("Arquivo não encontrado: {}", filename);
                }
                io::ErrorKind::PermissionDenied => {
                    eprintln!("Sem permissão para abrir o arquivo: {}", filename);
                }
                _ => {
                    eprintln!("Erro ao abrir o arquivo '{}': {}", filename, error);
                }
            }
        }
    }
}