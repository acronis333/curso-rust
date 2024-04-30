// use std::vec; // importamos el módulo vec de la librería estándar de Rust
  
// 00 📌 TIPOS DE DATOS - char
// fn main() {
//     let primera_letra = 'A'; // tipo de dato "char" con comillas simples
//     let espacio = ' '; // tipo de dato "char" con comillas simples
//     let cono_fiesta = '🎉'; // Emoji también son char, gracias al Unicode otros lenguajes también son char
//     print!("0 - {} {} {}\n", primera_letra, espacio, cono_fiesta);
// }

// 00 📌 Convertir tipos de datos de forma segura con "as"
// fn main() {
//     let a = 13u8; // tipo de dato "u8" sin signo de 8 bits
//     let b = 7u32; // tipo de dato "u32" sin signo de 32 bits
//     let c = a as u32 + b; // convertimos "a" a "u32" y sumamos "b"
//     println!("0 - a convertido a u32: {}", c); // imprimimos el resultado
// otro ejemplo de casting de tipo integer -> tipo  char, tipo booleano -> tipo integer. 
//     let a = 65u8; // para convertirlo a char debe ser un integer de 8 bits sin signo, si fuera u32 deberíamos convertirlo antes a u8.
//     let falso: bool = false; // tipo de dato "bool" falso.
//     println!("0 - {} -> {}", a as char, falso as u8); // imprimimos el resultado.
// }

// 00 📌 Calcular la memoria que ocupa por ejemplo un char y contar los caracteres de un str.
// fn main() {
//     let a = 'A'; // tipo de dato "char" con comillas simples
//     let b = '🎉'; // Emoji también son char, gracias al Unicode otros lenguajes también son char
//     let c = "Esto es un str";
//     println!("0 - Tamaño de un char: {} bytes", std::mem::size_of_val(&a)); // imprimimos el tamaño de un char
//     println!("0 - Tamaño de un char: {} bytes", std::mem::size_of_val(&b)); // imprimimos el tamaño de un char
//     println!("0 - Tamaño en caracteres de un str: {}", c.chars().count()); // imprimimos el tamaño de un str
//     println!("0 - Otra forma de imprimir el tamaño: {}", c.len()); // imprimimos el tamaño de un str
// }

// 00 📌 Para la inferencia de tipos en los números utilizamos el tipo seguido del valor
// ejemplo:. let a = 13u8; // tipo de dato "u8" sin signo de 8 bits.
// también se puede añadir un guió bajo para separar los números y hacerlos más legibles
// ejemplo:. let a = 1_000_000; // tipo de dato "i32" con signo de 32 bits.
// ejemplo:. let b = 24_u8; // tipo de dato "u8" sin signo de 8 bits.
// el guión bajo no afecta al valor, solo es para hacerlo más legible, se pueden poner varios guiones bajos.
 
// 01 📌 MACRO "println!" Display the message "Hello, world!"
fn main() { 
    println!("1 - Hello, world!");
}

// 0a 📌 MACRO "format!" para crear un string
// fn main() {
//     let s = format!(" Hello, world!");
//     println!("{}", s);
//     // otra forma.
//     let nombre = "Javier";
//     let apellido = "García";
//     let ciudad = "Elche";
//     let imprimir = format!("1 - Hola, soy {} {} y vivo en {}", nombre, apellido, ciudad);
//     print!("{}", imprimir);
//     // Otra forma de construir un string con .into() y .to_string()
//     let mi_string = "Hola, mundo".to_string();
//     let mi_string2: String = "Hola, mundo".into(); // de esta forma se debe especificar el tipo de dato.
//     print!("mi_string: {} y mi_string2: {}", mi_string, mi_string2);
// }

// 01 📌 Pasar argumentos a la macro println!() entre corchetes "brakes"
// llamar a macro println! con 4 argumentos: a string, a valor, string, a valor
// fn main() { 
//     println!("2 - {} - {} - {} - {}", "Hola", 42, "mundo", 13);
// }

// 01 📌 Imprimir valores directos y con variables
// fn main() {
    // println!("Valor directo sin pasar variable: {}", 42); // imprimir valor directo
    // let a = 42;
    // println!("Valor con variable a: {}", a); // imprimir variable "a"
// }

// 01 📌 Imprimir valor pasado por función =========================================================
// fn main () {
//     println!("Hola número: {}", numero());
// } 
// fn numero() -> i32 {
//     42 // ojo devuelve un valor de tipo i32 si no lleva punto y coma, con ella  da error
// }
// ========================================================================================

// 01 📌 VARIABLES
// Las variables existen dentro ed un bloque de código, se declaran con "let" y se pueden reasignar, pero desaparecen al salir del bloque, ejemplo:. la linea de impresión de "b" da error porque no existe fuera del bloque
// fn main() {
//     let a = 42;
//     {
//         let _b = 13;
//     }
//     println!("1 - Valor de a: {}", a);
//     // println!("1 - Valor de b: {}", _b); // ERROR, b no existe fuera del bloque
// }

// 01 📌  MÁS SOBRE LA IMPRESIÓN
// "#r" antepuesto ala variable le permite utilizar nombres reservados, ej, como let, fn, struct, etc.
// "#r", a veces se necesita imprimir muchas " y caracteres de escape, para ello se usa al comienzo "#r" antes de las primeras comillas
// Lo siguiente imprime los códigos ASCII de todas las letras a imprimir, tienen que ser solo vocales y no llevar tilde.
//  fn main() {
//     println!("{:?}", b"Esto es un texto"); // imprime los códigos ASCII de todas las letras a imprimir.
//  }
// Se pueden poner nombres de variables en las llaves, ej:. "{ciudad}", "{pais}", "{provincia}", etc. o incluso números, ej:. "{1}", "{2}", "{3}", etc. o repetirlos, ej:. "{ciudad}", "{ciudad}", "{ciudad}", etc.
// fn main() {
//     let ciudad = "Elche";
//     let pais = "España";
//     let provincia = "Alicante";
//     println!("1 - Ciudad: {ciudad}, País: {pais}, Provincia: {provincia} Este repite ciudad -> {ciudad}");
// }  

 
// 01 📌 Usar bloque de código para devolver un valor, devolución sin punto y coma, de lo contrario devolvería nada "()"
// fn main() {
//     let mi_numero = {
//         let segundo_numero = 29;
//         segundo_numero + 13
//     };
//     println!("1 - Valor de a: {}", mi_numero);
//     println!("1 - Valor de a: {:?}", mi_numero); // otra forma de imprimir valor, con {:?} se imprime el valor de la variable
// }

// 01 📌 VISALIZACIÓN Y DEPURACIÓN
// Existen variables que no se pueden imprimir usando {} en println! aquí es necesario usar la impresión de depuración
// Existen variables que no se pueden imprimir usando {:?}, como los booleanos, para ello se usa "{:}"
// Esta forma {#:?} se llama "pretty print" y es muy útil para depurar
// Con print! se imprime sin salto de línea, con println! se imprime con salto de línea
// Si queremos ver el mayor y menor valor de un tipo de dato, podemos usar std::mem::size_of_val(&variable)
// fn main() {
//     println!("El menor de i8 es \"{}\" y el mayor es \"{}\"", i8::MIN, i8::MAX);
//     // así con todos los tipos de datos, por ejemplo: i16, u16,i32,u32,i64,u64,i128,u128
// }

// 01 📌 Los valores númericos se pueden imprimir en binario, octal, hexadecimal, etc.
// fn main() {
//     let a = 42; // variable "a" con valor 42
//     println!("6 - Valor de \"a\" en binario: {:b}", a); // imprimimos el valor de "a" en binario
//     println!("6 - Valor de \"a\" en octal: {:o}", a); // imprimimos el valor de "a" en octal
//     println!("6 - Valor de \"a\" en hexadecimal: {:x}", a); // imprimimos el valor de "a" en hexadecimal
// }

// 01 📌 Se pueden añadir numeros entre las llaves para indicar el orden de las variables a utilizar
// fn main () {
//     let nombre_padre = "Juan";
//     let nombre_hijo = "Pedro";
//     let apellido = "Pérez";
//     println!("Este es {1}  {2}, hijo de {0} {2}.", nombre_padre, nombre_hijo, apellido);
// }

// 01 📌 Imprimir variables y una tupla de diferentes tipos
// fn main() {
//     let text1 = "3 - La primera letra del alfabeto";
//     let text2 = "La última letra del alfabeto:";
//     let sa = (13, false);
//     println!("3 - {}: {} y {} {} y \n2 - array pos 0 = {} \n2 - array pos 1 = {}", text1, 'A', text2, 'Z', sa.0, sa.1);
// }

// 01 📌 Aquí los tipos los e implementado yo, después de la variable insertar los ":" y el tipo.
// fn main() {
//     let _x: u32 = 42; // integer de 32 bits sin signo
//     let url: String = String::from("http://desarrollosdigitales.info"); // tipo "String"
//     let a_url: &str = "http://desarrollosdigitales.info"; // tipo texto "&str"

//     println!("5 - Esto es un String: {}", url);
//     println!("5 - Esto es un Texto &str: {}", a_url);
// }

// 02 📌 VARIABLES Y MUTABILIDAD
// fn main() { 
//     let mut number = 5; // mut proporciona mutabilidad a la variable, pero no podemos cambiar el tipo de dato
//     number += 1;
//     println!("valor que reemplaza el anterior '5' por misma variable: {}",number);
// }

// 02 📌 SHADOWING, OCULTACIÓN
// fn main() {
//     let number = 5; // variable "number" con valor 5
//     println!("Valor de number: {}", number); // imprimimos el valor de "number" = 5
//     let number = 9.9; // redefinimos la variable "number" con el valor 9,8 y de tipo f64, pero es completamente diferente
//     println!("Valor de number: {}", number); // imprimimos el valor de "number" = 9.9
//     // hemos ocultado la variable "number" con otra variable de diferente tipo y valor
//     // no se destruye la variable anterior, solo se bloquea, se oculta, "shadowing"
//     // ejemplo de utilidad: para hacer varios calculos con la misma variable.
// }

// 02 📌 SHADOWING, OCULTACIÓN
// Recordamos que el ocultamiento de variables no destruye la variable anterior, solo la bloquea, la oculta, "shadowing", con el uso de referencias se puede acceder a la variable anterior.
// fn main() {

//     let pais = String::from("España"); // variable "pais" con valor "España"
//     let pais_ref = &pais; // variable "pais_ref" con referencia a "pais"
//     let pais = 8; // redefinimos la variable "pais" con el valor 8
//     println!("{}, {}", pais_ref, pais); // imprimimos el valor de "pais_ref" y "pais"
//     // hemos ocultado la variable "pais" con otra variable de diferente tipo y valor
//     // no se destruye la variable anterior, solo se bloquea, se oculta, "shadowing"
//     // la variable pais se destruirá al salir del bloque,
// }

// 02 📌 LA PILA Y LA MEMORIA DINÁMICA - REFERENCIAS
// El puntero que se ve en rust se denomina "referencia" y se representa con "&", ej:. &variable
// &variable1, es una referencia a la variable, no es el valor en sí, es una referencia a la dirección de memoria
// esto significa que variable1 sigue siendo la dueña del valor, solo lo ha prestado y entrega una referencia

// ejemplo de referencia: &variable1, es una referencia a la variable, no es el valor en sí, es una referencia a la dirección de memoria
// fn main() {
//     let pais = "España"; // variable "pais" con valor "España"
//     let ref_uno = &pais; // variable "ref_uno" con referencia a "pais"
//     let ref_dos = &pais; // variable "ref_dos" con referencia a "pais"
//     let ref_tres = &ref_dos; // variable "ref_tres" con referencia a "ref_dos"
//     println!("{}", ref_uno);
//     println!("{}", ref_dos);
//     println!("{}", ref_tres);
// }


// 02 📌 MÁS SOBRE REFERENCIAS
// Como protege rust el acceso a zonas de memoria erróneas, no permite el acceso a zonas de memoria que no le pertenecen, un ejemplo.
// fn return_str() -> &'static str {
//     let pais = String::from("España");
//     let pais_ref = &pais;
//     pais_ref    // ⚠️ emoji warning <- ERROR, no se puede devolver una referencia a un valor que se destruirá al salir de la función.
// }
// fn main() {
//     let pais = return_str();
//     println!("{}", pais);
// }
// ===================================================================================

// 02 📌  REFERENCIAS MUTABLES
// Regla: no se puede usar una referencia mutable al mismo tiempo que una referencia inmutable
// fn main() {
//     let mut mi_numero = 8;
//     let num_ref = &mut mi_numero;
//     *num_ref += 10; // desreferenciamos con "*" el valor de "num_ref" y le sumamos 10
//     // "*" es lo opuesto a "&", "&" es una referencia, "*" es desreferenciar
//     println!("{}", mi_numero);
//     let num_modify = &mi_numero;
//     println!("{}", num_modify);
// }

// Se dispone de {p} para imprimir la dirección de memoria de una variable, ej:. "{:p}"
// fn main() {
//     let a = 42; // variable "a" con valor 42
//     let b = &a; // variable "b" con referencia a "a"
//     println!("6 - Dirección de memoria de \"a\": {:p}", b); // imprimimos la dirección de memoria de "a"
//     println!("6 - Valor de \"a\": {}", a); // imprimimos el valor de "a"
//     println!("6 - Valor de \"b\": {}", b); // imprimimos el valor de "b"
// }

//  📌 PASO DE REFERENCIAS A FUNCIONES
// Regla de Rust para todas los valores, "un valor solo puede tener un dueño a la vez".
// fn print_pais(pais_nombre: String) {
//     println!("{}", pais_nombre);
// }
// fn main() {
//     let pais = String::from("España"); // se crea la variable "pais" con valor "España"
//     print_pais(pais); // se llama a la función "print_pais" con la variable "pais"
//     // print_pais(pais); // ⚠️  ERROR, no se puede usar una variable que ya no es dueña del valor

//     // Al pasar la variable "pais" a la función "print_pais" se transfiere la propiedad del valor a la función y su nuevo dueño es "pais_nombre"
// } 

//  📌 ARRAYS - array es una colección de longitud fija de elementos de datos del mismo tipo.
//  El tipo de datos para un array es [T;N] siendo T el tipo del elemento, y N la longitud fija 
//  conocida en tiempo de compilación. Los elementos individuales se pueden recuperar con el 
//  operador [x], siendo x un índice de tipo usize (empezando por 0) del elemento que quieras.              

// fn main() {
//     let números: [i32;3] = [1,2,3];
//     println!("7 - Array {:?}", números);
// }
 
// fn main() {
//     let mut notas_array: [u32; 5] = [0; 5];  // Array con tamaño fijo de 5 elementos e inicializadas las 5 posiciones con ceros
//     let meses = ["Enero, febrero, marzo, abril, mayo, Junio, Julio, Agosto, Septiembre, Octubre, Noviembre, Diciembre"]; // Array de tipo String
//     notas_array[0] = 1;
//     notas_array[1] = 6;
//     println!("7 - Nota 1 = {}\n    Nota 2 = {}\n  Todas = {:?}" , notas_array[0], notas_array[1], notas_array);
//     println!("7 - {:?}", meses);
// }


// 📌 VECTORES | Estructutas de datos 

// fn main () {
//         let mut notas_vec: Vec<i32> = vec!(); // Vector dinámico (No fijo) vació, integer 32 bits con signo 
//         notas_vec.push(1); // escribimos un valor
//         notas_vec.push(6); // escribimos un segundo valor
//         println!("8 - Nota 1 = {}\n    Nota 2 = {}\n", notas_vec[0], notas_vec[1]);
// }

// fn main () {
//     let mut notas_vec: Vec<i32> = vec!(); // Vector dinámico (No fijo) vació, integer 32 bits con signo 
//     notas_vec.push(1); // escribimos un valor
//     notas_vec.push(6); // escribimos un segundo valor
//     println!("8 - Nota 1 = {}\n    Nota 2 = {}\n", notas_vec[0], notas_vec[1]);
// }

//  📌 CONSTANTES Y STATIC
// fn main () {
//     const PI:f64 = 3.14159; // las constantes no cambian nunca su valor, se declaran con "const" y en mayúsculas
//     static ESTACIONES: [&str; 4] = ["Primavera", "Verano", "Otoño", "Invierno"]; // las variables estáticas pueden ser mutables, se declaran con "static" y en mayúsculas, son como variables globales.
//     println!("Vamos de paseo, {} {} {}", PI, PI, PI);
//     println!("Las estaciones son: {:?}\n", ESTACIONES);
// }

//  📌 CONSTANTES y casting ("as")

// fn main () {
//     const CONSTANTE: f64 = 3.14;   // constante valor para PI, SCREAMING_SNAKE_CASE para las constantes
//     let xa = 42;       // variable con asignación de tipo y valor
//     let xa = (xa as f64) + CONSTANTE; // la palabra reservada "as" es hacer casting "convertir de tipo, 
//                                     // y al mismo tiempo estamos haciendo "shadowing" al  redefinir la variable "xa"
//     println!("9 - El valor de xa es: {}", xa);
// }

//  📌 CASTING - conversión de tipos
 
// fn main () {
    // let a = 13u8;
    // let b = 7u32;
    // let c = a as u32 + b;
    // println!("9 - a convertido a u32: {}", c);
    // let t = true;
    // println!("{}", t as u8);
// }

    
//  📌 TUPLAS, son como una estructura sin nombre de campos, una especie de array donde 
//  cada elemento puede ser de un tipo diferente pero especificado de antemano
// fn main () {
    // let tupla = (23,"Javier", true);       // Formamos la tupla directamente con valores de tipo (integer, texto, boleano)    
    // let (random, z_name, has_beers) = tupla; // Desestructuramos la tupla y obtenemos 3 variables
    // // Imprimimos las variables obtenidas
    // println!("\n10 - {}", random);
    // println!("11 - {}", z_name);
    // println!("12 - {}\n", has_beers);
// }
    
//  📌 EXPRESIONES AVANZADAS con variables "let", expresión condicional con "if, else", 
//   en Rust si algo no lleva punto y coma se vuelve Y evalúa como una expresión
// fn main () {
    // let age: u8 = 15;
    // let xx = if age > 17
    // {
    //     "Mayor de edad"
    // } else 
    // {
    //     "Menor de edad"
    // };
    // println!("13 - Eres{}",xx);
// }

//  📌 EXPRESIONES AVANZADAS con variables "let", una operación de "a*b"
    
// fn main () {
//     let u = 2;
//     let _xa = u+age;                // el valor de "u" se suma al de "age"
//     println!("14 - Valor de x: {}",_xa);        // imprime la expresión avanzada de "x"
// }
    
// 📌 Rust no deja la memoria al descubierto ni usa GC. Para ello el compilador realiza
// una tarea de dueños y préstamos que veremos a continuación
// Las REGLAS -> Cada valor en Rust tiene una variable que es su dueñ
//            -> Un valor solo puede tener un dueño a la ve
//            -> Cuando el dueño desaparece, el valor lo hace a su vez, de forma automátic

//  📌 La trait COPY y CLONE
// Distinto comportamiento según el tipo de un valor, la trait Copy (la mayoría de tipos primitivos), 
// entonces su comportamiento por defecto es de copia, la copia es barata y rápida y no influye 
// que existan varias copias de lo mismo.
// Se trata de valores que se almacenan en el stack.
// Clone permite hacer copias de datos más complejos, por ejemplo de un vector
// La primitiva String solo incorpora Clone, crear una copia dela variable daría error como el ejemplo de abajo

// fn main(){
//     let s1 = String::from("Adios - Xavier Cugat");
//     let s2 = s1;
//     println!("{}",s1); // ERROR hemos movido el valor de s1 a "s2" "s1" ya no es dueño de ese valor", ya no podemos imprimir s1 
//     // ERROR NO IMPRIme s1
// }
    

// Este código da error porque el tipo String no implementa Copy. Entonces la línea let s2 = s1; 
// lo que ha hecho en realidad ha sido mover el valor. Mover significa que le ha transferido el 
// ser dueño del valor de la cadena de texto a s2. Por tanto s1 ya no es dueña del valor y no puede 
// operar con él. Esto pasa en los tipos que no implementan Copy, que transfieren la propiedad 
// a otra variable. Si queremos hacer una copia real, tendremos que recurrir al clonado. 
    
    
    
// 📌 El tipo String implementa Clone así que es posible generar otro dato String exactamente igual 
// pero independiente al original.


// 📌 Ejemplo de CLONE para un String
// let s1 = String::from("Adios - Xavier Cugat");
// let s2 = s1.clone();
// println!("15 - Hemos clonado \"s2\" desde \"s1\" ahora las dos variable tienen el mismo valor\n     y cada una es propietaria de si misma, -> valor =  {}",s2);

// 📌 IMPLICACIONES - Pasar una variable tal cual a una función si no es del tipo Copy 
// implica que perdemos el acceso a ese valor!
    
// fn main() {
    
//     // let s1 =  String::from("Bolero - Maurice Ravel");
//     // let s2 = s1.clone();
//     // f(s2);ste 

//     // Este código daría error si al hacer la llamada a la función f hemos transferido la propiedad 
//     // del valor de s1 a f. Por ello, cuando intentamos hacer el print no vamos a poder ya que 
//     // s1 ya no es dueña de la cadena de texto. 
//     // Para solucionar estos problemas tenemos los préstamos, tal y como ha quedado el código.
// }

    
// 📌 PRESTAMOS (Prestando en Rust) 2 maneras: solo lectura o con escritura
// NORMA: solo una con permisos de escritura pero infinidad con permiso de lectura, nunca las dos a la vez. 
// El prestamo se realiza con el operador "&" que es una "referencia" de lectura al valor
// La variable sigue siendo la dueña del valor, solo lo ha prestado y entrega una referencia

    
//        -->-->--> // AQUÍ BUSCAR EJEMPLOS de prestasmos referencia de lectura 

    
//  📌 PRESTAMOS (Prestando en Rust) prestasmo en modo escritura, debemos utilizar "&mut"
    
// fn f(s: &mut String) {
//     s.push_str(" & Adios - Xavier Cugat");
// }
// let mut s1 = String::from("16 - Bolero - Maurice Ravel");
// f(&mut s1);
// println!("{}",s1);

// 📌 FUNCIONES SIMPLES - Si la función devuelve un valor se debe poner una flecha
// y el tipo del valor de devolución. Para devolver un valor se puede usar return  o se puede dejar la última línea sin punto y coma. 
// fn suma(a:i32, b:i32) ->i32 {
//     a+b
// }
// fn main () 
// let a = 5;
// let b = 42;
// let c= suma(a,b);
// println!("17 - resultado es: {}",c);
// }

//  📌 Rust como tal no admite devolver varios valores a la vez, pero es posible usar tuplas y simularlo.
// fn string_length_and_lines(txt: &String) -> (usize,usize) {
// (txt.len(),txt.lines().count()) // función contar cantidad caracteres
// }
// fn main() {
// let ss = String::from("Europe's Skies - Alexander Rybak\nSuper Strut - Deodato\nEl Cóndor Pasa - Uña Ramos"); // asignamos 3 lineas de tipo Striterminalng a variable "ss"
// let (length,lines) = string_length_and_lines(&ss); // asignamos el valor de la función contar_cantidad_caracteres
// // a variable longitud y lineas
// println!("18 - La lista de canciones tiene una longitud de {} caracteres y {} líneas",length,lines); // salida por pantalla variables resultados
// }

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
// # [ no_mangle ] // evita que el compilador cambie el nombre de la función, cuando optimice el código.
// Guión bajo (underscores) como sufijo de las variables (delante de ellas) para que no salga la advierta de "variable no utilizada
// Es una convención en Rust utilizar snake_case para: variables, funciones y archivos
// SCREAMING_SNAKE_CASE -> para constantes y estáticas, en mayusculas y guiones bajos
// PascalCas -> se utiliza para tipos, rasgos y enums
// CamelCase -> se utiliza para funciones y métodos
// Rust es un lenguaje de programación de sistemas, de bajo nivel, con un alto rendimiento y seguro
// Rust es un lenguaje de programación de propósito general, multi-paradigma, concurrente y seguro
// En Rust hay que favorecer el uso de variables locales, en lugar de globales siempre que sea posible, si necesitamos compartir datos entre funciones, se pueden usar argumentos y retornos de funciones o estructuras de datos compartidas.

//  📌   TIPOS BÁSICOS

// Booleanos - bool para representar verdadero/falso.
// Números enteros sin signo - u8 u32 u64 u128 para representar números enteros positivos.
// Números enteros con signo - i8 i32 i64 i128 para representar números enteros positivos y negativos.
// Números enteros de tamaño de puntero - usize isize se usan para representar índices y tamaños de elementos en memoria.
// Números en coma flotante - f32 f64.
// En relación a textos - str char.
// Tuplas - (valor,valor,...) para pasar secuencias fijas de valores en la pila.
// Slices - &[T] para referenciar “vistas” en secuencias de valores en la memoria.
// Un string siempre ocupa 24 bytes en la pila, independientemente de su longitud, es de tamaño fijo.
// Un i8 siempre ocupa 1 byte en la pila, independientemente de su valor, es de tamaño fijo.
// Un i32 siempre ocupa 4 bytes en la pila, independientemente de su valor, es de tamaño fijo.
 // Un f32 siempre ocupa 4 bytes en la pila, independientemente de su valor, es de tamaño fijo.
// Un f64 siempre ocupa 8 bytes en la pila, independientemente de su valor, es de tamaño fijo.
// Un char siempre ocupa 4 bytes en la pila, independientemente de su valor, es de tamaño fijo.
// Un bool siempre ocupa 1 byte en la pila, independientemente de su valor, es de tamaño fijo.
// Un usize e isize siempre o}año de la cadena de texto en la pila, es de tamaño variable + el tamaño de la referencia, generalmente 4 u 8 bytes.
   
    

