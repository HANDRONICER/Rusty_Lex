//Llamada a nuestras "librerias externas" ==> crates
    //crates son: resplit,clap
//Llamada de: "librerias de apoyo que expanden las funcionalidades de una crates/ librerias simbioticas" == "traits"
    //traits son: Cli, Parser

use resplit::Cli;
use clap::Parser;

fn main() { //una funcion que se llama main es el punto de entrada de nuestra aplicacion, lo que entiendo que hace en este momento es:
    let cli = Cli::parse(); //Realiza un parse de los argumentos de la linea de comandos.
    println!("{}", cli.input); //Imprime el argumento de la linea de comandos actual.
    println!("{}", cli.output); //Imprime el argumento de la linea de comandos actual ///en caso de que exista algun argumento de salida (en este caso el argumento de salida es: ¿resultado?)
    let buffer = resplit::read_stdin(); //lee el contenido del archivo de entrada

    let result = resplit::split(buffer, &cli); //realiza el split del contenido del archivo de entrada: "&cli", tomando en cuenta el "caracter o palabra que indica cuando debe realizarse un split de inmediato: buffer. 
    //Pero mas alla del nombre buffer cu ¿cual es esa palabra,caracter,delimitante?"
    println!("el resultado es: {}",result); // imprime el resultado del split realizado al leer el contenido del archivo de entrada.
}
