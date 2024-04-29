use std::vec; // importamos el módulo vec de la librería estándar de Rust


fn main() {

 
    
//  📌 Macro "println!" Display the message "Hello, world!"
    
    // println!("1 - Hello, world!");

    /*
    📌 Pasar argumentos a la macro entre corchetes "brakes"
    llamar a macro println! con 4 argumentos: a string, a valor, string, a valor
    */
    // let text1 = "2 - La primera letra del alfabeto";
    // let text2 = "La última letra del alfabeto:";
    // let sa = (13, false);

    // println!("{}: {} y {} {} y \n2 - array pos 0 = {} \n2 - array pos 1 = {}", text1, 'A', text2, 'Z', sa.0, sa.1);
   

    // let mut _b_number = 5;
    // _b_number += 1;
    // println!(
    //     "4 - Este es el nuevo valor que reemplaza el anterior '5' por ser la misma variable: {}",
    //     _b_number
    // );

//  📌 Aquí los tipos los e implementado yo, después de la variable insertar los ":" y el tipo.

    // let _x: u32 = 42; // integer de 32 bits sin signo
    // let url: String = String::from("http://desarrollosdigitales.info"); // tipo "String"
    // let a_url = "http://desarrollosdigitales.info"; // tipo texto "&str"

    // println!("5 - Esto es un String: {}", url);
    // println!("6 - Esto es un Texto &str: {}", a_url);

/*  📌 ARRAYS - Un array es una colección de longitud fija de elementos de datos del mismo tipo.
       El tipo de datos para un array es [T;N] siendo T el tipo del elemento, y N la longitud fija 
       conocida en tiempo de compilación. Los elementos individuales se pueden recuperar con el 
       operador [x], siendo x un índice de tipo usize (empezando por 0) del elemento que quieras.              
*/
    // let nums: [i32;3] = [1,2,3];
    // println!("7 - Arrays es una colección con longitud fija de elementos de datos del mismo tipo  {:?}", nums);
    // println!("7 - Acceder a la posición 2 de un array nums[1]= {}", nums[1]);





//  📌 ARRAYS - VECTORES | Estructutas de datos

//     let mut notas_array: [u32; 5] = [0; 5];  // Array con tamaño fijo de 5 elementos e inicializadas las 5 posiciones con ceros
//     notas_array[0] = 1;
//     notas_array[1] = 6;
//     println!("7 - Nota 1 = {}\n    Nota 2 = {}\n", notas_array[0], notas_array[1]);

// //  📌 VECTORES | Estructutas de datos 

//     let mut notas_vec: Vec<i32> = vec!(); // Vector dinámico (No fijo) vació, integer 32 bits con signo 
//     notas_vec.push(1); // escribimos un valor
//     notas_vec.push(6); // escribimos un segundo valor
//     println!("8 - Nota 1 = {}\n    Nota 2 = {}\n", notas_vec[0], notas_vec[1]);

//  📌 CONSTANTES
    
    // const PI:f64 = 3.14159;
    //  println!("9 - Vamos de paseo, {} {} {}", PI, PI, PI);

//  📌 CONSTANTES y casting ("as")

    // const costante: f64 = 3.14;   // constante valor para PI, SCREAMING_SNAKE_CASE para las constantes
    // let xa = 42;       // variable con asignación de tipo y valor
    // let xa = (xa as f64) + costante; // la palabra reservada "as" es hacer casting "convertir de tipo, 
    //                                 // y al mismo tiempo estamos haciendo "shadowing" al  redefinir la variable "xa"
    // println!("9 - El valor de xa es: {}", xa);

    //  📌 Casting - conversión de tipos
    // let a = 13u8;
    // let b = 7u32;
    // let c = a as u32 + b;
    // println!("9 - a convertido a u32: {}", c);
    // let t = true;
    // println!("{}", t as u8);

    /*
    📌 TUPLAS, son como una estructura sin nombre de campos, una especie de array donde 
    cada elemento puede ser de un tipo diferente pero especificado de antemano
    */

    // let tupla = (23,"Javier", true);       // Formamos la tupla directamente con valores de tipo (integer, texto, boleano)    
    // let (random, z_name, has_beers) = tupla; // Desestructuramos la tupla y obtenemos 3 variables
    // // Imprimimos las variables obtenidas
    // println!("\n10 - {}", random);
    // println!("11 - {}", z_name);
    // println!("12 - {}\n", has_beers);

    /*
    📌 EXPRESIONES AVANZADAS con variables "let", expresión condicional con "if, else", 
       en Rust si algo no lleva punto y coma se vuelve Y evalúa como una expresión
    */

    // let age: u8 = 15;
    // let xx = if age > 17
    // {
    //     "Mayor de edad"
    // } else 
    // {
    //     "Menor de edad"
    // };
    // println!("13 - Eres{}",xx);

//  📌 EXPRESIONES AVANZADAS con variables "let", una operación de "a*b"
    
    // let u = 2;
    // let _xa = u+age;                // el valor de "u" se suma al de "age"
    // println!("14 - Valor de x: {}",_xa);        // imprime la expresión avanzada de "x"

    /*
    📌 Rust no deja la memoria al descubierto ni usa GC. Para ello el compilador realiza 
       una tarea de dueños y préstamos que veremos a continuación.
       Las REGLAS -> Cada valor en Rust tiene una variable que es su dueña
                  -> Un valor solo puede tener un dueño a la vez
                  -> Cuando el dueño desaparece, el valor lo hace a su vez, de forma automática
    */

//  📌 La trait COPY y CLONE

    /*
       Distinto comportamiento según el tipo de un valor, la trait Copy (la mayoría de tipos primitivos), 
       entonces su comportamiento por defecto es de copia, la copia es barata y rápida y no influye 
       que existan varias copias de lo mismo.
       Se trata de valores que se almacenan en el stack.
       Clone permite hacer copias de datos más complejos, por ejemplo de un vector
       La primitiva String solo incorpora Clone, crear una copia dela variable daría error como el ejemplo de abajo

       fn main(){
       let s1 = String::from("Adios - Xavier Cugat");
       let s2 = s1;
       println!("{}",s1); // ERROR hemos movido el valor de s1 a "s2" "s1" ya no es dueño de ese valor", ya no podemos imprimir s1 
       ERRROR  
       }
    */

    /*
       Este código da error porque el tipo String no implementa Copy. Entonces la línea let s2 = s1; 
       lo que ha hecho en realidad ha sido mover el valor. Mover significa que le ha transferido el 
       ser dueño del valor de la cadena de texto a s2. Por tanto s1 ya no es dueña del valor y no puede 
       operar con él. Esto pasa en los tipos que no implementan Copy, que transfieren la propiedad 
       a otra variable. Si queremos hacer una copia real, tendremos que recurrir al clonado. 
    */
    
    /*
    📌 El tipo String implementa Clone así que es posible generar otro dato String exactamente igual 
       pero independiente al original.
    */
    
//  📌 Ejemplo de CLONE para un String

    // let s1 = String::from("Adios - Xavier Cugat");
    // let s2 = s1.clone();
    // println!("15 - Hemos clonado \"s2\" desde \"s1\" ahora las dos variable tienen el mismo valor\n     y cada una es propietaria de si misma, -> valor =  {}",s2);

    /*
    📌 IMPLICACIONES - Pasar una variable tal cual a una función si no es del tipo Copy 
       implica que ¡perdemos el acceso a ese valor!
    */

    /*
         let s1 =  String::from("Bolero - Maurice Ravel");
         f(s1);
         println!("15 - {}",s1);

       Este código da error. Al hacer la llamada a la función f hemos transferido la propiedad 
       del valor de s1 a f. Por ello, cuando intentamos hacer el print no vamos a poder ya que 
       s1 ya no es dueña de la cadena de texto. 
       Para solucionar estos problemas tenemos los préstamos.
    */

    /*
    📌 PRESTAMOS (Prestando en Rust) 2 maneras: solo lectura o con escritura
       NORMA: solo una con permisos de escritura pero infinidad con permiso de lectura, nunca las dos a la vez. 
       El prestamo se realiza con el operador "&" que es una "referencia" de lectura al valor
       La variable sigue siendo la dueña del valor, solo lo ha prestado y entrega una referencia
    */
    
//        -->-->--> // AQUÍ BUSCAR EJEMPLOS de prestasmos referencia de lectura 

    
//  📌 PRESTAMOS (Prestando en Rust) prestasmo en modo escritura, debemos utilizar "&mut"
    
    // fn f(s: &mut String) {
    //     s.push_str(" & Adios - Xavier Cugat");
    // }
    // let mut s1 = String::from("16 - Bolero - Maurice Ravel");
    // f(&mut s1);
    // println!("{}",s1);

    /*
    📌 FUNCIONES SIMPLES - Si la función devuelve un valor se debe poner una flecha
       y el tipo del valor de devolución. Para devolver un valor se puede usar return 
       o se puede dejar la última línea sin punto y coma.
    */

    // fn suma(a:i32, b:i32) ->i32 {
    //     a+b
    // }
    // let a = 5;
    // let b = 42;
    // let c= suma(a,b);
    // println!("17 - resultado es: {}",c);

//  📌 Rust como tal no admite devolver varios valores a la vez, pero es posible usar tuplas y simularlo.

    // fn string_length_and_lines(txt: &String) -> (usize,usize) {
    // (txt.len(),txt.lines().count()) // función contar cantidad caracteres
    // }
    // let ss = String::from("Europe's Skies - Alexander Rybak\nSuper Strut - Deodato\nEl Cóndor Pasa - Uña Ramos");
    // // asignamos 3 lineas de tipo Striterminalng a variable "ss"
    // let (length,lines) = string_length_and_lines(&ss); // asignamos el valor de la función contar_cantidad_caracteres
    // // a variable longitud y lineas
    // println!("18 - La lista de canciones tiene una longitud de {} caracteres y {} líneas",length,lines); // salida por pantalla variables resultados

//  📌  las funciones son elementos de primer nivel, lo que quiere decir que pueden pasarse por argumentos 
//      entre funciones

    // fn ladrar () {
    //     println!("19 - Guau");
    // } // función ladrar
    // fn hacer_n_veces(f:fn(),n:i64) {
    //     for _ in 0..n {
    //       f();
    //     } // bucle for in
    // }
    // hacer_n_veces(ladrar,10); // imprime 10 veces resultado de la función ladrar

//  📌  Aqui generacidad, poner algo

//  📌  Aqui CLOSURES

//  📌  Structs, traits y POO en Rust

//  📌 ESTRUCTURAS son datos agrupados por clave-valor.

//     #[derive(Copy,Clone)]
//     struct Punto{
//     x: i32,
//     y: i32
// }

//     struct Rectangulo{
//     origen: Punto,
//     ancho: i32,
//     alto: i32
// }  
//     let p = Punto {x: 50, y: 50};
//         println!("20 - Punto X: {}",p.x);

//  📌 

    
// #[derive(Copy,Clone)]
//     struct Punto{
//     x: i32,
//     y: i32
// }

//     struct Rectangulo{
//     origen: Punto,
//     ancho: i32,
//     alto: i32
// }  
//     let p = Punto {x: 50, y: 50};
//         println!("20 - Punto X: {}",p.x);

      

    










    





    


    // 📌  CARGO - Administrador de paquetes y compilador de Rust

    // cargo new -> crea un directorio de proyecto
    // cargo build -> compila
    // cargo run  -> compila si hay cambios en el proyecto y ejecuta el mismo
    // cargo check -> testea el proyecto
    // cargo run --bin [programa_rust] -> si el proyecto esta en otro directorio
    // cargo build --release -> para compilarlo con optimizaciones
    // cargo edit -> nos ayuda con las dependencias inserta o importa el nombre de un crate (libreria o módulo)
    // -> reescribe el archivo Cargo.toml para adicionar de pendencias -> https://github.com/killercup/cargo-edit
    // -> viendo la versión que necesitas en crates.io
    
    

    // 📌 APUNTES Y NOTAS VARIAS

    // #[allow(dead_code)] // suprime las advertencias de código no utilizado
    // Guión bajo (underscores) como sufijo de las variables para que no salga la advierta de "variable no utilizada
    // Es una convención en Rust utilizar snake_case para: variables, funciones y archivos
    // SCREAMING_SNAKE_CASE -> para constantes y estáticas, en mayusculas y guiones bajos
    // PascalCas -> se utiliza para tipos, rasgos y enums


    //  📌   TIPOS BÁSICOS

    // Booleanos - bool para representar verdadero/falso.
    // Números enteros sin signo - u8 u32 u64 u128 para representar números enteros positivos.
    // Números enteros con signo - i8 i32 i64 i128 para representar números enteros positivos y negativos.
    // Números enteros de tamaño de puntero - usize isize se usan para representar índices y tamaños de elementos en memoria.
    // Números en coma flotante - f32 f64.
    // En relación a textos - str char.
    // Tuplas - (valor,valor,...) para pasar secuencias fijas de valores en la pila.
    // Slices - &[T] para referenciar “vistas” en secuencias de valores en la memoria.
}
