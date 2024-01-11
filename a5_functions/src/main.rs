fn main() {//La funcion "main" es la funcion que se ejecuta automaticamente al iniciar el programa por primera vez.

    println!("Hello, world!"); //Imprimimos una frase en consola
    another_function(); // Llamada de la funcion
    another_function_with_parameters(5, 6); // Llamada de la funcion, se proporcionan los "valores concretos"="argumentos"==x:5,y:6, que seran asignados a "los contenedores de valores propios de la funcion que declaramos"=="parametros"== especificados dentro de la declaracion de nuestra funcion. 
}

fn another_function() { //Para declarar una funcion, se utiliza la palabra reservada "fn" seguido del nombre de la funcion.//Declaracion de la funcion
    println!("Another function."); //Imprimimos una frase en consola
}

fn another_function_with_parameters(x:i32,y:i32) { //Declarando la funcion, declaramos dentro de los parametros de la funcion: "x:i32,y:i32", los cuales son declaraciones de "contenedores de valores propios de la funcion", 
//notese que se "annotan los tipos de datos" en la "declaracion de cada contenedor en particular", 
//Estos tipos de datos annotados solo seran asignados en el momento en el que se realize una "instanciacion de los contenedores de valores".
//Es decir, cuando se realize  una llamada de la "funcion", como consecuencia se desencadena que se realize correctamente la instanciacion de los contenedores de valores automaticamente.
  
    println!("Another function with parameters."); //Imprimimos una frase en consola
    println!("The value of x is: {}, The value of y is {}", x,y); //Imprimimos una frase en consola e imprimimos los valores de x y y.
}


//Las funciones son un "conjunto de 1)instrucciones y/o 2)evaluaciones" que se ejecutan cuando se llama a la funcion.
//Las funciones pueden tener parametros, y tambien pueden no tener parametros.


/*
Dentro de Rust: Existe algo denominado:

    STATEMENT/SENTENCIAS: Son objetos de codigo que al ejecutarse/llamarse NO RETORNAN NINGUN VALOR.
    EXPRESSION/EXPRESIONES: Son objetos de codigo que al ejecutarse/llamarse RETORNAN UN VALOR.

    Recordar: Un Objeto de codigo es una existencia de codigo a la cual se le realizo un NOMBRAMIENTO con el cual identificarle solo dentro del codigo de nuestro programa;
                Existen 2 objetos de codigo: Contenedor de "valores" ó Contenedor de "Funciones==Funcion/Acciones==Bloque de codigo"

*/

//Nota el codigo a continuacion nunca se ejecuta, por tanto el programa emitira una alerta acerca de: "valores sin utilizar" y "funciones sin utilizar",ya que el siguiente codigo es didactico, nunca es usado dentro de la funcion MAIN. haga caso omiso de la notificacion por favor.

//STATEMENT/SENTENCIAS: ejemplo de sentencia/instruccion:
const CONTENEDORDEVALOR_CONSTANTE_A_: i32 = 5; //Despues de ejecutar este codigo, vemos que el codigo no-retorna ningun valor

//Expression/expresion: ejemplo de expresion/evaluacion:
fn funcion_que_retorna_un_valor() -> i32 { //Despues de ejecutar este codigo, vemos que el codigo si retorna un valor.
    let y=5+1; // se realiza una "operacion matematica"=="evaluacion entre valores" y se guarda el resultado en la variable "y"
    return y; //notese que la palabra reservada "return" es una palabra reservada de Rust, y que se utiliza para retornar un valor de una funcion. en este caso el resultado final de la ejecucion de nuestra funcion fue almacenado en contenedor de valores: variable "Y", por lo que retornamos esa "variable". 
}

//Expression/expresion: ejemplo de expresion/evaluacion:
fn funcion_que_retorna_un_valor2() -> i32 { //notese el uso de "->" seguido de un datatype: i32, esto significa que se busca que la funcion retorne un valor de tipo "i32", esta accion de "definir el tipo de retorno deseado" NO SE NOMBRA.
    let y=5+1; // se realiza una "operacion matematica"=="evaluacion entre valores" y se guarda el resultado en la variable "y"
    y //notese que se puede omitir la palabra reservada "return" que se utiliza para retornar un valor de una funcion,aun asi se retorna el valor de la variable "Y" como el resultado final de la ejecucion de nuestra funcion, gracias a que Rust implicitamente la ejecucion o lectura de la ultima linea como valor, (siempre que esta ultima linea no tenga un "punto y coma")
}




