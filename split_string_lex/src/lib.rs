//crates and traits zone
use std::io::{BufReader, BufRead};
use clap::Parser;

//atributtes zone:
#[derive(Parser)]
#[command(name = "resplit", version = "0.0.1", author = "Alejandro Chavez Barrera, Alfredo Deza", about = "Split strings by one or more dilimeters and return a field like cut" long_about = None)]
//This struct is an attribute too, that uses the clap crate to parse the command line arguments, and also uses the attributes above this comment (i know it uses parser but i'm not sure in what way: ¿as entry?, ¿as function that passes the entry?)
pub struct Cli {
    #[arg(short('f'))]
    field: usize,
    #[arg(short('d'))]
    delimeter: String,
    #[arg(short('s'))]
    debug: bool,
}

pub fn read_stdin() -> String { //esta funcion: lee una entrada directamente del teclado y la devuelve como un string.
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read input line");
    line.trim().to_string()
}


pub fn split(s: String, cli: &Cli) -> String { // esta funcion: lee una entrada proveniente de la linea de comandos (¿imagino que el resultado de la operacion read_stdin?) y provee como salida: el resultado de separar la string de entrada, este resultado se encuentra en forma de: ¿vector, arreglo de objetos, lista (debido a los indices que puedo ver)?
    let parts: Vec<&str> = s.split(&cli.delimeter).collect();
    if cli.debug {
        println!("Parts: {:?}", parts);
        println!("Indexes available starting at 0: {}", parts.len());
    }
    parts.get(cli.field).unwrap_or(&"").to_string()
}