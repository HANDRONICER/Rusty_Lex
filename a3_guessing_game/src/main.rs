
use std::io; //PRELUDIO == Se llama a la biblioteca que contiene dentro de si la libreria: IO (inpiut/output), la cual habilita la funcionalidad de recibir entradas y salidas en el teclado por parte del usuario. 
use rand::Rng; 
//Se llama a la biblioteca rand para que este disponible dentro del codigo que estamos haciendo en nuestro archivo main.rs
//al mismo tiempo se llama al paquete de codigo: "trait"== Rng, el cual define funciones y metodos que se encargan de modificar el comportamiento de una libreria encargada de generar numeros.
use std::cmp::Ordering; //sub-libreria "compare" el cual contiene dentro de si codigo capaz de habilitar funciones de comparacion de datos, sub-paquete de codigo: Ordering==> el cual define un DataType: "enumerador", con el nombre de: "Ordering", a su vez con 3 variantes, cada una de las cuales es capaz de instanciarse al encontrar si el valor es MAYOR QUE, MENOR QUE O IGUAL QUE.

fn main() {
    
    println!("Bienvenido al juego:::: ¡¡Adivina el numero!!"); //Esta linea imprime el texto que esta dentro de ella.
    println!("Por favor introduce solo un numero, este debe estar dentro del rango 1 al 10, ESTO ES SOLO PARA PROBAR QUE EL PROGRAMA ESTA EN FUNCIONAMIENTO");
    
    let mut guess: String = String::new(); //Se crea una variable para contener el "input del usuario"==(datos proporcionados por el usuario)

    io::stdin().read_line(&mut guess).expect("Failed to read line"); 
    //se usa la libreria: std, especificamente la sub_libreria: io la cual que contiene dentro de si las definiciones/declaraciones de los DataTypes cuya funcionalidad es ACEPTAR,PROCESAR Y CONTENER Datos RECIBIDOS desde el teclado, 
    //estos DataTypes contienen dentro de si mismos funciones propias para poder crear una instancia de si mismos: es decir poseen la capacidad de AUTO-INSTANCIARSE.
    //Dentro de la sub-libreria "io", se encuentra el codigo de la definicion del Dataype "stdin"
    //como se usa una funcion asociada, accedemos de manera independiente a la definicion del codigo del DataType de "stdin" (la cual si se verifica podemos ver que esta contenida dentro de la libreria "io" y por tanto es perfectamente compartible con "io").
    //y desde la "Definicion" del DataType "stdin", llamamos y ejecutamos directamente a la funcion asociada: "stdin()"  
    //esta funcion "stdin()", contenida dentro de la definicion de codigo del DataTyoe "stdin", crea una "instancia" del DataType "stdin" el cual se encarga de almacenar y manipular datos de entrada, luego invocamos la sub-funcion/funcion localizada dentro del DataType STDIN especializada en leer la entrada directamente del teclado:  .read_line()
    //esta ultima lee la entrada al teclado, y luego la pasa al Stdin, el cual almacena el valor de forma temporal y realiza el procedimiento necesario para convertir esa entrada binaria proveniente desde el teclado a su correspondiente equivalencia en valor string.

    println!("Acabas de introducir el numero: {}", guess ); //imprime el numero que acabamos de introducir para mostrarlo.

    /* */

    let x = 5;
    let y = 10;
    println!("valor_x = {x} and valor_y+2 = {}",y+2); //NOTESE: como al imprimir el valor de x DENTRO de los PLACEHOLDERS, se utiliza directamente el nombre de la variable x, en cambio para imprimir Y, pasamos el dato a los Placeholders (actualmente vacios) empleando para ello una coma seguida de una expresion que define la variable que sera incluida detro de ese place holder.

    /*
    Añadir en .toml, la dependencia externa:
    
    [dependencies]
    rand = "0.8.5"

    */

    println!("Ahora si: por favor trata de adivinar el numero!!!");

    loop{
        //esto hace que vuelva a preguntar!
        
        println!("INTRODUCE TU NUMERO");

        let numero_secreto = rand::thread_rng().gen_range(1..=100); //Genera un numero random del 1 al 100, otra forma de colocar los rangos es: (1,101) == comenzando en el 1 y terminando antes del 101
        //la funcion "thread_rgn()" define una funcion derivada de la biblioteca rand, esta funcion ejecuta un generador de numeros aleatorios, al ejecutar esta funcion se debe pasar un rango valido el cual, en lugar de pasarlo literalmente como argumento dentro de los parentesis de la funcion, es introducido con apoyo de la funcion: gen_range()
        //la funcion gen_range() es una funcion asociada que pertenece al trait "Rng" que mandamos a llamar en el preludio, lo que hace esta libreria es que permite definir un "rango"

        

        let mut guess: String = String::new();
        //redeclaramos la variable guess HACIENDO SHADOWING == 1)reusamos el identificador de la localidad de memoria y con ello se libera automaticamente la localidad de memoria gracias al compilador
        //2) se llena la variable reutilizada con un valor.

        io::stdin()
            .read_line(&mut guess)
            .expect("Fallo al obtener la entrada de datos del teclado proveida por el usuario");

            /*
            
            let guess: u32 = guess 
            //Realizamos shadowing a la variable Guess == Re-declaramos toda la variable GUESS, cambiando asi su DataType y por tanto el valor que contiene
            //el valor que contendra nuestra variable ahora sera: el valor anterior 
                .trim() //eliminamos si es que existen los espacios en blanco solo al principio, y al final de nuestra cadena de texto (nota1: deja intactos los espacios de enmedio). (nota2: se consideran espacios en blanco cualquier "caracter de escape" ejemplo: "\r == " ó "\n"=="newline")       
                .parse()//Luego de eliminar los espacios en blanco, usamos parse para convertir esa string a un numero, Es NECESARIO especificar el DataType al cual queremos convertir, esto se realiza dentro de la parte de la asignacion de la variable: vease: "let guess: u32"
                        //si es exitoso retorna el valor en formato u32, de lo contrario retorna un valor con DataType "Result" el cual avisa de un error en el codigo. 
                .expect("Por favor, introduzca solo numeros"); // se empleara luego el metodo: .expect() el cual sirve para administrar posibles fallas, en caso de que el parse sea exitoso, recibe el valor y lo retorna sin hacer nada mas, en caso de que parse() no haya sido exitoso retorna en lugar del valor: un mensaje!!!, y en lugar de crashear el programa si no se mepleara este metodo, muestra en su lugar el mensaje!!! de error.

        

             */
        let guess: u32 = //Realizamos shadowing a la variable Guess == Re-declaramos toda la variable GUESS, cambiando asi su DataType y por tanto el valor que contiene
        
        match guess.trim().parse() { //Esto genera un DataType Result el cual podemos alterar mediante superposicion de datos empleando un MATCH
                Ok(num) => num,
                Err(_) => continue,
            };
        
        println!("El numero secreto es: {}", numero_secreto);
        println!("tu respuesta introducida es: {}",guess);

        match guess.cmp(&numero_secreto){ //comparamos guess utilizando la funcion ".cmp()" para ver si es mas grande, mas chico o igual que "numero_secreto"
        //lo cual causa que un estado dentro de la definicion Ordering sea activado, esta activacion de un estado crea una instancia de la variante activada proveniente de esa misma definicion del DataType: Enumerador "Ordering"
        
        //cmp entonces retorna una instancia del enumerador Ordering, pero dentro de esta instancia del enumerador ya se encuentra tambien instanciada solo una variante en particular la cual fue exitosamente instanciada, la cual es leida por la EXPRESION match, 
        //es entonces que la expresion match verifica que exista una igualdad de datos entre el valor leido y las definiciones de variantes que posee, siendo este estado de igualdad el estado de activacion que crea una instancia en particular de una sola variante entre las las posibles definiciones de variantes dentro de esta expresion match.

        //al instanciarse una variante en esta expresion match se desencadena/activa la ejecucion del codigo que este directamente asociado a la variante instanciada.

            Ordering::Less => println!("Tu numero es menor que el numero secreto"),
            Ordering::Equal => { 
                println!("Tu numero es igual a el numero secreto");
                break;
            },//sale del programa en cuanto el numero es igual
            Ordering::Greater => {
                println!("Tu numero es mayor a el numero secreto");
            }, //otra manera de colocar los comandos en caso de que necesitemos ejecutar de forma ordenada un bloque de codigo con varias lineas de codigo dentro.
        }

        

    }

    
}
