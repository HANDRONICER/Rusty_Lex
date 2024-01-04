//Code from: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html
//Don't make assumtions, verify and corroborate by yourself before do, doing and twice after done.



fn main() {
    let mut x = 5; //Declaracion e invocacion de una variable mutable de nombramiento: "x"
    println!("The value of x is: {x}");
    x = 6; //Re-asignacion de valor de nuestra variable mutable "x"
    println!("The value of x is: {x}");
        
    //Importante: Shadowing permite re-declarar una variable con el mismo nombre, pero con un nuevo tipo y/o valor.
    
    { //Esta corchea delimita el inicio de un bloque anidado
    
        let x = x * 2; //Re-Declaracion de la variable x ==> "SHADOWING" aplicado a la variable x), Esta variable shadow solo sera valida dentro de este bloque anidado.
        println!("El valor de x dentro de este nuevo bloque anidado es:  {x}"); //El valor actual de la variable x con shadowing aplicado es: 12
    
    } //Esta corchea delimita el final de un bloque anidado
    
    
    
    //En esta zona ya nos encontramos fuera de nuestro bloque anidado, por lo que la validez del shadowning anterior termino, por tanto el valor de x fuera del bloque anidado es 6 de acuerdo a la variable mutable original: "x".
    //Mientras que valor dentro de la variable x shadow, siempre y cuando se encuentre dentro del bloque anidado para ser valida, siempre sera: 12
    println!("El valor de x fuera del bloque anidado es: {x}");
    
    }