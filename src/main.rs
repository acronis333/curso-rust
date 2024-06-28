// IMPORTACIONES DE LIBRERIAS, MÓDULOS o CRATES
// use std::vec; // importamos el módulo vec de la librería estándar de Rust

// 📌   TIPOS DATOS BÁSICOS

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

// 📌 TIPOS DE DATOS - (char) - dato alfabético más primitivo del lenguaje que representa un único carácter Unicode, de 32 bits
// se puede utilizar para almacenar un solo carácter Unicode, como una letra, un número, un emoji, etc. entre comillas simples
// significa que puede representar mucho más que ASCII. Letras acentuadas; Caracteres chinos, japoneses y coreanos; Emojis etc.
// - Recordar que el espacio en blanco también es un carácter Unicode. 
// - Se puede utilizar para almacenar, comparar, convertir y mostrar caracteres Unicode.
// - Almacenar caracteres individuales: Se puede utilizar el tipo char para almacenar un único carácter Unicode en una variable. 
// esto puede ser útil para trabajar con cadenas de texto o para procesar caracteres individuales.
// - Comparar caracteres: Se pueden comparar dos valores char para determinar si son iguales o diferentes. 
// esto puede ser útil para ordenar cadenas de texto o para buscar caracteres específicos en una cadena.
// - Convertir caracteres a otros tipos: Se pueden convertir valores char a otros tipos de datos, como String o u8. 
// esto puede ser útil para trabajar con datos de cadena o para almacenar caracteres en formato binario.
//
// fn main() {
//     let primera_letra = 'A'; // tipo de dato "char" con comillas simples
//     let espacio = ' '; // tipo de dato "char" con comillas simples
//     let cono_fiesta = '🎉'; // Emoji también son char, gracias al Unicode otros lenguajes también son char
//     print!("0 - {} {} {}\n", primera_letra, espacio, cono_fiesta);
//
// ejemplos:
//
// ALMACENAR un carácter en una variable
//     let caracter: char = 'a';

// COMPARAR dos caracteres
//     let caracter1: char = 'b';
//     let caracter2: char = 'c';
//
//     if caracter1 == caracter2 {
//     println!("Los caracteres son iguales");
//     } else {
//     println!("Los caracteres son diferentes");
//     }
//
// CONVERTIR un carácter a una cadena
//     let caracter: char = 'd';
//     let cadena = caracter.to_string();
//     println!("La cadena es: {}", cadena);
//
// CONVERTIR un carácter a un valor u8
//     let caracter: char = 'e';
//     let valor_u8: u8 = caracter as u8;
//     println!("El valor u8 es: {}", valor_u8);
//
// 📌 TIPOS DE DATOS COMPUESTOS - Tipos compuestos pueden agrupar múltiples valores en un solo tipo. 
// - Tipos compuestos pueden agrupar múltiples valores en un solo tipo. Rust tiene dos tipos compuestos primitivos: tuplas y arreglos.
// Tipo tupla: Una tupla es una colección de valores de diferentes tipos. Los valores de una tupla se llaman elementos.
// - Los elementos de una tupla pueden ser de cualquier tipo, incluidos otros tipos compuestos como tuplas y arreglos.
// - Los elementos de una tupla se pueden acceder mediante índices, que comienzan en 0.
// // - Los elementos de una tupla se pueden desestructurar para asignarlos a variables individuales.
// fn main() {
//     let tup = (500, 6.4, 1);
//
//     let (x, y, z) = tup;
//
//     println!("The value of y is: {y}");
// }
// - También podemos acceder directamente a un elemento de la tupla usando un punto (.) seguido del índice del valor que queremos acceder. 
//
// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);
//
//     let five_hundred = x.0;
//
//     let six_point_four = x.1;
//
//     let one = x.2;
// }
// 
// El Tipo Arreglo: Un arreglo es una colección de valores del mismo tipo. Los valores de un arreglo se llaman elementos.
// - Los elementos de un arreglo tienen un tamaño fijo, que se establece en tiempo de compilación.
// - Se pueden acceder mediante índices, que comienzan en 0, se pueden desestructurar para asignarlos a variables individuales.
// - Los elementos de un arreglo se pueden acceder directamente usando un índice entre corchetes.
// - Los arreglos son más útiles cuando sabe que el número de elementos no cambiará. Por ejemplo, si está utilizando los nombres 
// del mes en un programa, probablemente usaría un arreglo en lugar de un vector porque sabe que siempre contendrá 12 elementos:
// ejemplo:
// let meses = ["January", "February", "March", "April", "May", "June", "July", August", "September", "October", "November", "December"];
// ejemplo: let a: [i32; 5] = [1, 2, 3, 4, 5]; -> tipo = i32, longitud = 5 elementos 
// }

// 📌 INFERENCIA EN TIPO DE DATOS - Para la inferencia de tipos en los números utilizamos el tipo seguido del valor
//    ejemplo:. let a = 13u8; // tipo de dato "u8" sin signo de 8 bits.
//    también se puede añadir un guió bajo para separar los números y hacerlos más legibles
//    ejemplo:. let a = 1_000_000; // tipo de dato "i32" con signo de 32 bits.
//    ejemplo:. let b = 24_u8; // tipo de dato "u8" sin signo de 8 bits.
//    el guión bajo no afecta al valor, solo es para hacerlo más legible, se pueden poner varios guiones bajos.

// 📌 NÚMEROS DECIMALES - SE utiliza el punto y NO la coma, se utilizan dos tipos f32 y f64, por defecto f64
//    El compilador de Rust es inteligente y no elegirá f64 si necesitas f32
//    ejemplo:. let a = 42.0; // tipo de dato "f64" con coma flotante de 64 bits.
//    ejemplo:. let b = 42.0f32; // tipo de dato "f32" con coma flotante de 32 bits.
//    ejemplo:. let c = 1_000.0; // tipo de dato "f64" con coma flotante de 64 bits.
//    ejemplo:. let d = 1_000.0f32; // tipo de dato "f32" con coma flotante de 32 bits.
// fn main() {
//     let mi_decimal: f32 = 5.0; // Rust elige f64
//     let mi_otro_decimal = 8.5; // Normalmente Rust elegiría f64
//     // pero al conocer que lo vamos a sumar a un f32, elige un f32 para mi_otro_decimal
//     let tercer_decimal = mi_decimal + mi_otro_decimal;
// }
//
// 📌 FUNCIONES - funciones simples - Si la función devuelve un valor se debe poner una flecha
// y el tipo del valor de devolución. Para devolver un valor se puede usar return o simplemente un valor sin punto y coma.
//
// fn suma(a: i32, b: i32) -> i32 {
//     a + b
// }
// fn ladrar() {
//     println!("Guau");
// }
//
// fn main() {
// let b = 42;
// let c= suma(a,b);
// println!("17 - resultado es: {}",c);
// ladrar();
// }
//
// 📌 FUNCIONES - pasar como parametros,lo que quiere decir que pueden pasarse por entre funciones
//     
// fn hacer_n_veces(f:fn(),n:i64) {
//     for _ in 0..n {
//       f();
//     } // bucle for in
// }
// fn main() {
//     fn ladrar() {
//         println!("Guau");
//     }
//     hacer_n_veces(ladrar,10); // imprime 10 veces resultado de la función ladrar
// }
// 📌 - Rust como tal no admite devolver varios valores a la vez, pero es posible usar tuplas y simularlo.
//
// fn string_length_and_lines(txt: &String) -> (usize,usize) {
// (txt.len(),txt.lines().count()) // función contar cantidad caracteres
// }
//
// fn main() {
// let ss = String::from("Europe's Skies - Alexander Rybak\nSuper Strut - Deodato\nEl Cóndor Pasa - Uña Ramos"); // asignamos 3 lineas de tipo String a variable "ss"
// let (length,lines) = string_length_and_lines(&ss); // asignamos el valor de la función contar_cantidad_caracteres
// // a variable longitud y lineas
// println!("18 - La lista de canciones tiene una longitud de {} caracteres y {} líneas",length,lines); // salida por pantalla variables resultados
// }
//



// 📌 MACRO "println!" - Display the message "Hello, world!"
// fn main() { 
//     println!("Hello, world!");
// }

// 📌 MACRO "format!" - Para crear un String
// fn main() {
//     let s = format!(" Hello, world!");
//     println!("{}", s);
//     // otra forma.
//     let nombre = "Javier";
//     let apellido = "García";
//     let ciudad = "Elche";
//     let imprimir = format!("1 - Hola, soy {} {} y vivo en {} ", nombre, apellido, ciudad);
//     println!("{}", imprimir);
//     // Otra forma de construir un string con .into() y .to_string()
//     let mi_string = "Hola, mundo".to_string();
//     let mi_string2: String = "Hola, mundo".into(); // de esta forma se debe especificar el tipo de dato.
//     println!("mi_string: {} y mi_string2: {}", mi_string, mi_string2);
// }

// 📌 Pasar argumentos a la macro println!() entre corchetes "brakes"
// llamar a macro println! con 4 argumentos: a string, a valor, string, a valor
// fn main() { 
//     println!("2 - {} - {} - {} - {}", "Hola", 42, "mundo", 13);
// }

// 📌 Imprimir valores directos y con variables
// fn main() {
    // println!("Valor directo sin pasar variable: {}", 42); // imprimir valor directo
    // let a = 42;
    // println!("Valor con variable a: {}", a); // imprimir variable "a"
// }

// 📌 Imprimir valor pasado por función 
// fn main () {
//     println!("Hola número: {}", numero());
// } 
// fn numero() -> i32 {
//     42 // ojo devuelve un valor de tipo i32 si no lleva punto y coma, con ella daría error
// }
// ==========================================================================================

// 📌 VARIABLES
// Las variables existen dentro ed un bloque de código, se declaran con "let" y se pueden reasignar, pero desaparecen al salir del bloque, ejemplo:. la linea de impresión de "b" da error porque no existe fuera del bloque
// fn main() {
//     let a = 42;
//     {
//         let _b = 13;
//     }
//     println!("1 - Valor de a: {}", a);
//     // println!("1 - Valor de b: {}", _b); // ERROR, b no existe fuera del bloque
// }

// 📌  IMPRESIÓN - más sobre ella
// "#r" antepuesto ala variable le permite utilizar nombres reservados, ej, como let, fn, struct, etc.
// "#r", a veces se necesita imprimir muchas " y caracteres de escape,
// para ello se usa al comienzo "#r" antes de las primeras comillas 
// Lo siguiente imprime los códigos ASCII de todas las letras a imprimir, tienen que ser solo vocales y no llevar tilde.
//  fn main() {
//     println!("{:?}", b"Esto es un texto"); // imprime los códigos ASCII de todas las letras a imprimir.
//  }
// Se pueden poner nombres de variables en las llaves, ej:. "{ciudad}", "{pais}", "{provincia}", etc. o incluso números, ej:. "{1}", "{2}", "{3}", etc. o repetirlos, ej:. "{ciudad}", "{ciudad}", "{ciudad}", etc.
// fn main() {
//     let ciudad = "Elche";
//     let pais = "España";
//     let provincia = "Alicante";
//     println!("1 - Ciudad: {ciudad}, País: {país}, Provincia: {provincia} Este repite ciudad -> {ciudad}");
// }  

// 📌 Usar bloque de código para devolver un valor, devolución sin punto y coma, de lo contrario devolvería nada "()"
// fn main() {
//     let mi_numero = {
//         let segundo_numero = 29;
//         segundo_numero + 13
//     };
//     println!("1 - Valor de a: {}", mi_numero);
//     println!("1 - Valor de a: {:?}", mi_numero); // otra forma de imprimir valor, con {:?} se imprime el valor de la variable
// }

// 📌 VISUALIZACIÓN Y DEPURACIÓN
// Existen variables que no se pueden imprimir usando {} en println! aquí es necesario usar la impresión de depuración
// Existen variables que no se pueden imprimir usando {:?}, como los booleanos, para ello se usa "{:}"
// Esta forma {#:?} se llama "pretty print" y es muy útil para depurar
// Con print! se imprime sin salto de línea, con println! se imprime con salto de línea
// Si queremos ver el mayor y menor valor de un tipo de dato, podemos usar std::mem::size_of_val(&variable)
// fn main() {
//     println!("El menor de i8 es \"{}\" y el mayor es \"{}\"", i8::MIN, i8::MAX);
//     // así con todos los tipos de datos, por ejemplo: i16, u16,i32,u32,i64,u64,i128,u128
// }

// 📌 Los valores numéricos se pueden imprimir en binario, octal, hexadecimal, etc.
// fn main() {
//     let a = 42; // variable "a" con valor 42
//     println!("6 - Valor de \"a\" en binario: {:b}", a); // imprimimos el valor de "a" en binario
//     println!("6 - Valor de \"a\" en octal: {:o}", a); // imprimimos el valor de "a" en octal
//     println!("6 - Valor de \"a\" en hexadecimal: {:x}", a); // imprimimos el valor de "a" en hexadecimal
// }

// 📌 Se pueden añadir números entre las llaves para indicar el orden de las variables a utilizar
// fn main () {
//     let nombre_padre = "Juan";
//     let nombre_hijo = "Pedro";
//     let apellido = "Pérez";
//     println!("Este es {1}  {2}, hijo de {0} {2}.", nombre_padre, nombre_hijo, apellido);
// }

// 📌 Imprimir variables y una tupla de diferentes tipos
// fn main() {
//     let text1 = "3 - La primera letra del alfabeto";
//     let text2 = "La última letra del alfabeto:";
//     let sa = (13, false);
//     println!("3 - {}: {} y {} {} y \n2 - array pos 0 = {} \n2 - array pos 1 = {}", text1, 'A', text2, 'Z', sa.0, sa.1);
// }

// 📌 Aquí los tipos los e implementado yo, después de la variable insertar los ":" y el tipo.
// fn main() {
//     let _x: u32 = 42; // integer de 32 bits sin signo
//     let url: String = String::from("http://desarrollosdigitales.info"); // tipo "String"
//     let a_url: &str = "http://desarrollosdigitales.info"; // tipo texto "&str"
//     println!("5 - Esto es un String: {}", url);
//     println!("5 - Esto es un Texto &str: {}", a_url);
// }

// 📌 VARIABLES Y MUTABILIDAD
// fn main() { 
//     let mut number = 5; // mut proporciona mutabilidad a la variable, pero no podemos cambiar el tipo de dato
//     number += 1;
//     println!("valor que reemplaza el anterior '5' por misma variable: {}",number);
// }

// 📌 SHADOWING, OCULTACIÓN
// fn main() {
//     let number = 5; // variable "number" con valor 5
//     println!("Valor de number: {}", number); // imprimimos el valor de "number" = 5
//     let number = 9.9; // redefinimos la variable "number" con el valor 9,8 y de tipo f64, pero es completamente diferente
//     println!("Valor de number: {}", number); // imprimimos el valor de "number" = 9.9
//     // hemos ocultado la variable "number" con otra variable de diferente tipo y valor
//     // no se destruye la variable anterior, solo se bloquea, se oculta, "shadowing"
//     // ejemplo de utilidad: para hacer varios calculos con la misma variable.
// }

// 📌 SHADOWING, OCULTACIÓN
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

// 📌 LA PILA Y LA MEMORIA DINÁMICA - REFERENCIAS
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

// 📌 MÁS SOBRE REFERENCIAS
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

// 📌  REFERENCIAS MUTABLES
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
//
// 📌  REFERENCIAS MUTABLES
// Se dispone de {p} para imprimir la dirección de memoria de una variable, ej:. "{:p}"
// fn main() {
//     let a = 42; // variable "a" con valor 42
//     let b = &a; // variable "b" con referencia a "a"
//     println!("6 - Dirección de memoria de \"a\": {:p}", b); // imprimimos la dirección de memoria de "a"
//     println!("6 - Valor de \"a\": {}", a); // imprimimos el valor de "a"
//     println!("6 - Valor de \"b\": {}", b); // imprimimos el valor de "b"
// }

// 📌 PASO DE REFERENCIAS A FUNCIONES
// Regla de Rust para todas los valores, "un valor solo puede tener un dueño a la vez".
// fn print_pais(pais_nombre: String) {
//     println!("{}", pais_nombre);
// }
// fn main() {
//     let pais = String::from("España"); // se crea la variable "pais" con valor "España"
//     print_pais(pais); // se llama a la función "print_pais" con la variable "pais"
// print_pais(pais); // ⚠️  ERROR, no se puede usar una variable que ya no es dueña del valor.
// Al pasar la variable "pais" a la función "print_pais" se transfiere la propiedad del valor.
// a la función, por lo que la variable "pais" ya no es dueña del valor.
// } 
// Es mejor evitar que la función se apropie del valor, para ello se pueden pasar referencias (prestamos los valores) a la función, ej:. "&String"  
// fn print_pais(pais_nombre: &String) {
//     println!("{}", pais_nombre);
// }
// fn main() {
//     let pais = String::from("España"); // se crea la variable "pais" con valor "España"
//     print_pais(&pais); // se llama a la función "print_pais" con la variable "pais"
//     print_pais(&pais); // 😀 ahora si funciona, se puede usar la variable "pais" en varias funciones
//     println!("{}", pais); // comprobamos que la variable "pais" sigue siendo dueña del valor.  
// } 

// 📌 COPIA 
// Rust tiene una característica especial para los tipos de datos primitivos, la trait "Copy", que permite que los valores se copien en lugar de moverse.
// Son valores de tamañol fijo, conocido y pequeño que se almacenan en el stack (enteros,flotantes y char), no en el heap, por lo que son rápidos de copiar y no influye que existan varias copias de lo mismo.
// Pueden copiarse cuando se pasan por parametro a una función, se asignan a otra variable o se devuelven de una función.
// fn print_number(number: i32) { // Esta función no devuelve nada
//     // Si el  número no se copiara, se movería y no se podría usar, la función seria su dueña.                   
// println!("{}", number);
// }
// fn main() {
//     let mi_numero = 8;
//     print_number(mi_numero); // Imprime 8, la función obtiene una copia del valor de "mi_numero"
//     print_number(mi_numero); // Imprime 8 de nuevo, la función obtiene una copia del valor de "mi_numero".
// }

// 📌 CLONE
// El tipo String, no implementa la característica copiar, por lo que el valor de la variable se mueve al pasarla la primera vez, para poder copiarla se usa la trait "Clone".
// Lo ideal es utilizar la referencia es más eficiente porque clone copia el valor gastando más memoria y la referencia solo el puntero. 
// fn print_country(country_name: String) { // Esta función no devuelve nada
//     println!("{}", country_name);
//     }
//     fn main() {
//         let country = String::from("España");
//         print_country(country.clone());
//         print_country(country);
//     }

// 📌 TIPOS COLECCIÓN
// Rust tiene varios tipos de colecciones, como vectores, arrays, tuplas, etc.
// Sirven para guardar más de un valor en un mismo lugar.
// Empezamos con los arrays, que son colecciones de longitud fija de elementos de datos del mismo tipo y los más simples y rápidos.

// 📌 ARRAYS - array es una colección de longitud fija de elementos de datos del mismo tipo.
//  El tipo de datos para un array es [T;N] siendo T el tipo del elemento, y N la longitud fija 
//  conocida en tiempo de compilación. Los elementos individuales se pueden recuperar con el 
//  operador [x], siendo x un índice de tipo usize (empezando por 0) del elemento que quieras. 
// Los arrays no pueden cambiar el tamaño y sus datos tienen que ser del mismo tipo, sin   embargo son muy rápidos y eficientes.
 
// 📌 ARRAYS - se puede obtener una sección (slice) de un array utilizando una referencia "&" y después utilizando ".." para mostrar el rango
// Los indices empiezan en 0, por lo que el primer elemento es el 0, el segundo el 1, etc.
// Los rangos son inclusivos en el primer número y exclusivos en el segundo, por lo que [2..5] obtiene los elementos 2, 3 y 4.
// Para que se incluya el último número se puede usar de esta forma [0..=10].
// Para que se incluya el último número, se puede usar [1..] y para que se incluya el primero, se puede usar [..5].
// fn main() {
//     let numeros: [i32;10] = [1,2,3,4,5,6,7,8,9,10]; // array literal de 10 elementos de tipo i32
//     println!("Todo el array: {:?}", numeros); // imprimimos el slice
//     let _slice_1_al_3 = &numeros[1..3]; // obtebemos indices 1 al 2
//     let _slice_todos = &numeros[1..]; // obtenemos indices 1 al 9 o final
//     let _slice_1_al_4 = &numeros[..10]; // obtenemos indices 0 al 9
//     println!("Slice de 1 al 3: {:?}", _slice_1_al_3); // imprimimos el slice
//     println!("Slice de 1 al final: {:?}", _slice_todos); // imprimimos el slice
//     println!("Slice de 1 al 4: {:?}", _slice_1_al_4); // imprimimos el slice
// }

// 📌 ARRAYS
// fn main() {
//     let numeros: [i32;3] = [1,2,3];
//     println!("Array {:?}", numeros);
//     //
    
// }

// 📌 ARRAYS
// fn main() {
//     let mut notas_array: [u32; 5] = [0; 5];  // Array con tamaño fijo de 5 elementos e inicializadas las 5 posiciones con ceros
//     let meses = ["Enero, febrero, marzo, abril, mayo, Junio, Julio, Agosto, Septiembre, Octubre, Noviembre, Diciembre"]; // Array de tipo String
//     notas_array[0] = 1;
//     notas_array[1] = 6;
//     println!("7 - Nota 1 = {}\n    Nota 2 = {}\n  Todas = {:?}" , notas_array[0], notas_array[1], notas_array);
//     println!("7 - {:?}", meses);
// }

// 📌 VECTORES 
// Los vectores son estructuras de datos dinámicas que permiten almacenar más de un valor en un solo lugar, colecciones de longitud variable de elementos del mismo tipo.
// Se pueden añadir o quitar elementos, pero no se pueden mezclar tipos de datos.
// Los vectores son más flexibles que los arrays, pero son más lentos y ocupan más memoria.
// Se pueden añadir elementos con el método "push" y quitar elementos con el método "pop".
// Se pueden acceder a los elementos con el método "get" y se pueden modificar con el método "set".
// Se pueden obtener secciones de un vector con el método "slice".
// Se pueden recorrer los elementos de un vector con un bucle "for" y modificarlos con el método "iter_mut". 
// Se pueden declarar vectores de forma dinámica con la macro "vec!".
// Se pueden declarar vectores al igual que los arrays, con el tipo de dato y la longitud, ej:. let mut notas_vec: Vec<i32> = vec!();
// Se pueden declarar vectores al igual que un string mediante el método "new", ej:. let mut notas_vec = Vec::new(); <- vector vacío, no es necesario especificar el tipo de dato.
// Los vectores siempre contienen valores, para eso son sirven los paréntesis angulares "<>".
// Un Vec<String> es un vector que contiene elementos de tipo String.
// Un Vec<i32> es un vector que contiene elementos de tipo i32.
// Un Vec<(i32, i32)> es un vector que contiene tuplas de dos elementos de tipo i32.
// Los vectores se relocalizan en la memoria si se añaden más elementos de los que puede contener.
// Los vectores se pueden clonar con el método "clone".
// Los vectores se pueden convertir en arrays con el método "as_slice".
// Los vectores se pueden convertir en strings con el método "join".
// Los vectores se pueden convertir en strings con el método "join" y en mayúsculas con el método "to_uppercase".
// Si se conoce el número de elementos que va a contener el vector, se puede usar la macro "with_capacity" para reservar memoria, Vec::with_capacity(10), para que funcione más rápido.

// 📌 VECTORES
// fn main() {
//     let producto1 = String::from("Agua");
//     let producto2 = String::from("Leche");
//     let mut mi_vector = Vec::new();
//     // Si se compilara este prgrama hasta aquí el compilador daría error.
//     // ya que no conoce el tipo de dato que se va a almacenar en el vector.
//     mi_vector.push(producto1); // Ahora si lo conoce, es de tipo String.
//     mi_vector.push(producto2);
//     print!("{:?}", mi_vector); // imprime los elementos del vector.
// }
// En lugar de usar .push() para deducir el tipo de elementos que contiene se puede especificar el tipo de dato.

// 📌 VECTORES
// fn main() {
//     let mut mi_vector3: Vec<String> = Vec::new(); // El compilador ya sabe que el vector contiene
//                                                   // elementos de tipo String.
//     mi_vector3.push(String::from("Agua")); // inserta un elemento de tipo String.
//     mi_vector3.push("Café".to_string()); // inserta elemento de tipo String con método to_string().
//     println!("{:?}", mi_vector3); // imprime los elementos del vector Agua y Café.
// }

// 📌 VECTORES
// otra forma de declarar un vector con la macro "vec!"
// fn main() { 
//     let mut mi_vector4 = vec![1,2,3,4,5]; // Vector de 5 elementos de tipo i32.
//     mi_vector4.push(6); // inserta un elemento de tipo i32.
//     mi_vector4.push(7); // inserta un elemento de tipo i32.
//     println!("{:?}", mi_vector4); // imprime los elementos del vector.
// }

// 📌 VECTORES
// Se pueden obtener secciones de un vector igual que en los arrays, con el método "slice".
// fn main() {
//     let mut mi_vector5 = vec![1,2,3,4,5]; // Vector de 5 elementos de tipo i32.
//     let slice = &mi_vector5[1..3]; // obtenemos los elementos 1 y 2.
//     println!("{:?}", slice); // imprime los elementos del slice.

// 📌 VECTORES
// fn main () {
//         let mut notas_vec: Vec<i32> = vec!(); // Vector dinámico (No fijo) vació, integer 32 bits con signo 
//         notas_vec.push(1); // escribimos un valor "1" en la posicion 0
//         notas_vec.push(6); // escribimos un segundo valor "6" en la posicion 1.
//         println!("Nota 1 = {}\n    Nota 2 = {}\n", notas_vec[0], notas_vec[1]);
// }

// 📌 VECTORES
// Se pueden obtener secciones de un vector igual que en los arrays, con el método "slice".
// fn main() {
//     let random_vector = vec!["Esto es un texto", "8", "a", "b", "8,9,10", "7.7"];
//     println!(
//         "El interior del vector contiene Primer elemento: {:?}
//         Segundo elemento: {:?}
//         Tercer elemento: {:?}  
//         Cuarto elemento: {:?}
//         Quinto elemento: {:?}
//         Sexto elemento: {:?}",
//                 random_vector[0],
//                 random_vector[1],
//                 random_vector[2],
//                 random_vector[3],
//                 random_vector[4],
//                 random_vector[5]
//     );
//     for elementos in &random_vector[4..] { // No se puede  hacer un slice (iterar) en tuplas
//         println!("{}", elementos);
//     }
// }

// 📌 TUPLAS Y DESESTRUCTURACIÓN, son como una estructura sin nombre de campos, una especie de array donde 
//  cada elemento puede ser de un tipo diferente pero especificado de antemano
// fn main () {
    // let tupla = (23,"Javier", true);       // Formamos la tupla directamente con valores de tipo (integer, texto, boleano)    
    // let (random, z_name, has_beers) = tupla; // Desestructuramos la tupla y obtenemos 3 variables
    // // Imprimimos las variables obtenidas
    // println!("\n10 - {}", random);
    // println!("11 - {}", z_name);
    // println!("12 - {}\n", has_beers);
// } convertido a u32: {}", c);
    // let t = true;
    // println!("{}", t as u8);
// }

// 📌 TUPLAS
// fn main() {
//     let random_tuple = ("Esto es un texto", "8", "vec!['a']", "'b'", "[8,9,10], 7.7");
//     println!(
//         "El interior de la tupla contiene Primer elemento: {:?}
//         Segundo elemento: {:?}
//         Tercer elemento: {:?}  
//         Cuarto elemento: {:?}
//         Quinto elemento: {:?}
//         Sexto elemento: {:?}",
//                 random_tuple.0,
//                 random_tuple.1,
//                 random_tuple.2,
//                 random_tuple.3,
//                 random_tuple.4,
//                 random_tuple.5
//     );
// }
  
// 📌 ENUM - Enumeraciones
// Las enumeraciones son un tipo de dato que permite definir un tipo de dato con un número limitado de valores posibles.
// Se pueden definir enumeraciones con la palabra reservada "enum" seguida del nombre de la enumeración y entre llaves los valores posibles separados por comas. (la coma en el último valor es opcional pero importante si solemos cambiar de sitio los valores).
// Diferencia entre "struct" y "enum", las estructuras permiten definir un tipo de dato con varios campos (Representar una cosa Y otra cosa a la vez), mientras que las enumeraciones permiten definir un tipo de dato con un número limitado de valores posibles, (Representar una cosa O alguna cosa diferente).

// 📌 ENUM - ejemplo de enum
// Según la hora del día, el cielo puede tener el sol o las estrellas, creamos una enumeración con dos valores posibles.
// Creamos una función que "devuelve el estado del cielo" según la hora del día, la cual se la pasamos en el argumento de la función, la llamamos "tiempo" y a traves de match devolvemos el valor del enum. que corresponda según el rango de horas.
// Para llamar a un valor del enum se usa "::" después del nombre del enum y el valor.
// match es una expresión que permite comparar un valor con una serie de patrones y ejecutar un bloque de código dependiendo de qué patrón coincida, en este caso, el valor de la variable "tiempo" con los valores de la enumeración.
// la flecha después del patron de rango indica que se devuelva el valor del enum que corresponda.

// 📌 ENUM
// #[derive(Debug)]
// #[allow(dead_code)]
// enum CosasEnElCielo {   // Enumeración con dos valores posibles
//     Sol(String),                // Representa el dia -> Sol
//     Estrellas(String),          // Representa la noche -> Estrellas
// }
//  fn crear_estado_en_el_cielo(tiempo:i32) -> CosasEnElCielo { // Función que "devuelve" el estado del cielo
//     match tiempo {                                           // Según la hora del día a la variable "tiempo"
//         6..=8 => CosasEnElCielo::Sol(String::from("Puedo ver el Sol")),
//         _ => CosasEnElCielo::Estrellas(String::from("Puedo ver las estrellas")),
//     } 
// }
// fn comprobar_el_cielo(estado: &CosasEnElCielo) {
//     match estado {
//         CosasEnElCielo::Sol(a) => println!("{}",a),
//         CosasEnElCielo::Estrellas(b) => println!("{}",b),
//     }
// }
// fn main() {
//     let tiempo = 7; // "Ahora son las 8" creamos variable "tiempo" con valor 8 para pasar a la función "estado_en_el_cielo".
//     let estado = crear_estado_en_el_cielo(tiempo);
//     comprobar_el_cielo(&estado);   
// }

// 📌 BUCLES - Podemos crear un bucle con loop hacer que continue hasta que queramos que se detenga con "break", por ejemplo comparando un valor
// fn main() {
//     loop {
//         break; // Este bucle se ejecuta indefinidamente si no se le pone un "break".
//     }
// } 

// 📌 BUCLES
// fn main() {
//     let mut contador = 0; // inicializamos la variable "contador" con valor 0.
//     loop {                // creamos un bucle "loop".
//         contador += 1;    // incrementamos el valor de "contador" en 1.
//         println!("Contador: {}", contador); // imprimimos el valor de "contador".
//         if contador == 10 { // si el valor de "contador" es igual a 10.
//             println!("¡Hemos llegado a 10!"); // imprimimos el mensaje.
//             break; // salimos del bucle.
//         }
//     }
// }

// 📌 BUCLES ANIDADOS - Si se inserta un bucle dentro de otro es posible darles nombre para indicar a Rust a qué bucle salir cuando se ejecuta una sentencia break. 
// fn main() {
//     let mut contador = 0; // inicializamos la variable "contador" con valor 0.
//     let mut contador2 = 0; // inicializamos la variable "contador2" con valor 0.
//     println!("Entro en el primer bucle");
//     'primer_bucle: loop {
//         // Damos nombre de esta forma 'nombre_bucle: loop.
//         contador += 1; // incrementamos el valor de "contador" en 1.
//         println!("El contador es ahora: {}", contador);
//         if contador > 9 {
//             // Inicia un segundo bucle
//             'segundo_bucle: loop {
//                 contador2 += 1; // incrementamos el valor de "contador2" en 1.
//                 if contador2 == 3 {
//                     break 'primer_bucle; // Salimos del primer bucle para saliar del programa.
//                 }
//             }         
//         } 
//     } 
// }  

// 📌 BUCLES - While
// Este bucle se ejecuta mientras la condición sea verdadera " TRUE", si la condición es falsa "FALSE" no se ejecuta.
// fn main() {
//     let mut contador = 0; // inicializamos la variable "contador" con valor 0.
//     while contador < 10 {
//         contador += 1; // incrementamos el valor de "contador" en 1.
//         println!("Contador: {}", contador); // imprimimos el valor de "contador".
//     }
// }

// 📌 BUCLES - for
// El bucle "for" repite la ejecución un número determinado de veces, también se utiliza para recorrer una colección de elementos, como un array o un vector.
// // ".." y "..=", crea un rango de valores, el primero es excluyente ("0..5", rrecorre el 0,1,2,3,4) y el segundo es incluyente ("0..=5", recorre el 0,1,2,3,4,5).
// fn main() {
//     for contador in 0..5 {
//         println!("Contador excluyente: {}", contador);
//     }
//     println!(" "); // imprime un espacio en blanco
//     for contador in 0..=5 {
//         println!("Contador incluyente: {}", contador);
//         if contador == 2 {
//             break; // Salimos del bucle y devolvemos el valor de "contador".
//         }
//     }
// }
// En los bucles for se puede usar "continue" para saltar a la siguiente iteración y "break" para salir del bucle.
// En los bucles for se crea una variable a la que le asignamos el valor de cada iteración, en este caso "contador". si no se necesita la variable se puede usar "_" para indicar que no se va a usar.

// 📌 BUCLES - loop con break y devolución de valor.
// Break tambien se puede usar para devolver un valor, para ello escribimos "break valor" y el valor o variable que queremos devolver.
// 📌 
// fn main() {
//     let mut contador = 5;
//     let mi_numero = loop {
//         contador +=1;
//         if contador % 53 == 3 {
//             break contador;
//         }
//     };
//     println!("{}", mi_numero);
// }

// 📌 BUCLES - for (Comprobación de colores con un bucle)
// fn match_colores(rbg: (i32, i32, i32)) {
//     println!("Comparación de un color con {} rojo, {} azul, y {} verde:", rbg.0, rbg.1, rbg.2);
//     let new_vec = vec![(rbg.0, "rojo"), (rbg.1, "azul"), (rbg.2, "verde")]; // Coloca los colores en un vec. Dentro son tuplas con los nombres de los colores
//     let mut todos_tienen_al_menos_10 = true; // Comienza a verdadero y se cambia a falso si algún compomente no tiene 10
//     for item in new_vec { // cada item es una tupla con un número y un nombre de color.
//         if item.0 < 10 { // item.0 es el número, si es menor que 10
//             todos_tienen_al_menos_10 = false; // Ahora es false
//             println!("No mucho {}.", item.1) // item.1 es el color, se imprime el nombre del color.
//         }
//     }
//     if todos_tienen_al_menos_10 { // Comprueba si es verdadero e imprime si lo es
//         println!("Cada compomente de color tiene al menos 10.")
//     }
//     println!(); // Añade una línea vacía para separar
// }

// 📌 BUCLES -
// fn main() {
//     let first = (200, 20,0);
//     let second = (50, 50, 50);
//     let third = (200, 50, 0);
//     match_colores(first);
//     match_colores(second);
//     match_colores(third);
// }

// 📌  IMPLEMENTACIÓN - Implementaciones de funciones asociadas a los tipos definidos como tal.
// Se utiliza el bloque "impl" para "implementar funciones asociadas" a un tipo de dato definido por el usuario
// como son "struct" y "enum".
// En un bloque "impl" se pueden definir 2 tipos diferentes de métodos:
// MÉTODOS: que toman "self" (o &self o &mut self) como primer argumento, llamados métodos de instancia.
// Estos métodos utilizan un punto "." sobre una variable del tipo struct o enum para llamar a la función, 
// ej:. "mi_instancia.mi_metodo()", x.clone(), et.
// FUNCIONES ASOCIADAS al tipo: que no toman "self" como primer parámetro, en otros lenguajes se llaman métodos estáticos.
// funciones relacionadas con el tipo de datos pero que no necesitan una instancia del tipo para funcionar.
// Se llaman con el nombre del tipo seguido de dos puntos dobles "::" y el nombre de la función, 
// ej:. "MiTipo::mi_funcion()", String::from(), etc.
// Ejemplo de implimentación para crear animales y los imprime.
// para poder usar {:?} al imprimir un tipo de dato personalizado, se debe añadir 
// #[derive(Debug)] antes de la definición del tipo, a esto se le denomina atributo
//
// #[derive(Debug)]
// struct Animal {   // Definimos un tipo de dato "Animal" con dos campos.
//     edad: u8,
//     tipo_animal: TipoAnimal,
// }
// #[derive(Debug)]
// enum TipoAnimal {
//     Gato,
//     Perro,
// }
// impl Animal {
//     fn new() -> Self {  // creamos una función asociada al tipo "Animal" que devuelve un "Animal", se refiere a la estructura Animal
//         // También se puede usar "Animal" en lugar de "Self"
//         Self {
//             // Cuandfn main() {
//     let mut animal_nuevo = Animal::new();
//     animal_nuevo.comprobar_tipo();
//     animal_nuevo.cambiar_a_perro();
//     animal_nuevo.comprobar_tipo();
//     animal_nuevo.cambiar_a_gato();
//     animal_nuevo.comprobar_tipo();
// }
//     }
//     fn cambiar_a_perro(&mut self) {
//       // como está dentro de Animal, &mut self significa &mut Animal.
//       // Usa .cambiar_a_perro() para convertir el gato en un perro.
//       // con &mut self, se puede modificar el valor de la estructura.                                                
//         println!("!Cambiando el animal a perro¡");
//         self.tipo_animal = TipoAnimal::Perro;
//     }
//     fn cambiar_a_gato(&mut self) {
//       // como está dentro de Animal, &mut self significa &mut Animal.
//       // Usa .cambiar_a_gato() para convertir el perro en un gato.
//       // con &mut self, se puede modificar el valor de la estructura.                                                
//       println!("!Cambiando el animal a gato");
//       self.tipo_animal = TipoAnimal::Gato;
//     }
//     fn comprobar_tipo(&self) {
//        // se lee a sí mismo self, por lo que no se puede modificar.
//        match self.tipo_animal {
//         TipoAnimal::Perro => println!("Es un perro"),
//         TipoAnimal::Gato => println!("Es un gato"),
//        } 
//     }
// }
// fn main() {
//     let mut animal_nuevo = Animal::new();
//     animal_nuevo.comprobar_tipo();
//     animal_nuevo.cambiar_a_perro();
//     animal_nuevo.comprobar_tipo();
//     animal_nuevo.cambiar_a_gato();
//     animal_nuevo.comprobar_tipo();
// }

// 📌 impl - ejemplo de imnplementación de un struct un enum.
// Crear un coche, con datos Marca, Modelo, Año
// Cambiar de marca ese coche
// Comprobar la marca de ese coche
// Se debe recordar que Selg (el tipo Self) y self (la variable self) funcionan como abreviaturas del tipo que sea en cada momento.
// #[allow(dead_code)] 
// #[derive(Debug)]
// struct Coche {      // Definimos una estructura con un tipo de dato "Coche" con tres campos.
//     año: u16,
//     modelo: String,
//     marca_coche: MarcaCoche,
// }
// #[derive(Debug)]
// #[allow(dead_code)] 
// enum MarcaCoche {  // Definimos una enum con un tipo de dato "MarcaCoche" con tres valores posibles.
//     Citroen,
//     Nissan,
//     Mercedes,
// } 
// impl Coche {
//     fn new() -> Self {
//         Self {
//             marca_coche:MarcaCoche::Citroen,
//             año: 1998,
//             modelo: String::from("gti"),
//         }
//     }
//     fn cambiar_marca_mercedes(&mut self) {
//         println!("Cambiando la marca a Mercedes");
//         self.marca_coche = MarcaCoche::Mercedes;
//     }
//     fn comprobar_marca_coche(&self) {
//         match self.marca_coche {
//             MarcaCoche::Citroen => println!("Es un Citroen"),
//             MarcaCoche::Mercedes => println!("Es un Mercedes"),
//             MarcaCoche::Nissan => println!("Es un Nissan"),
//         }
//     }
// }
// fn main() {
//     let mut nuevo_coche1 = Coche::new();
//     println!("{:?}",nuevo_coche1);
//     nuevo_coche1.cambiar_marca_mercedes();
//     nuevo_coche1.comprobar_marca_coche();
// }

// 📌 impl - ejemplo de imnplementación de un enum.
// enum Estado {
//     Bueno,
//     Malo,
//     Somnoliente,
// }
// impl Estado {
//     fn consultar(&self) {
//         match self {
//             Estado::Bueno => println !("Me siento bien"),
//             Estado::Malo => println !("Eh, no me siento tan bien"),
//             Estado::Somnoliente => println !("Necesito dormir ahora"),
//         }
//     }
// }
// fn main() {
//     let mi_estado = Estado::Malo;
//     mi_estado.consultar();
// }

// 📌 IMPLEMENTACIÓN
// #[derive(Debug)]
// #[allow(dead_code)]
// struct Rectangulo {
//     ancho: i32,
//     alto: i32,
//  }
// impl Rectangulo {
//     fn area(&self) -> i32 {
//         self.alto * self.ancho
//     }
// }
// fn main() {
//    let rectangulo1 = Rectangulo {
//     ancho: 35,
//     alto: 32,
//    };
//    let rect = rectangulo1.area();
//    println!("{}", rect);
// }

// 📌 DESESTRUCTURAR - los valores de un struct
// #[derive(Debug)]
// #[allow(dead_code)]
// struct Persona { // Crea una estructura simple para Persona
//     nombre: String,
//     real_nombre: String,
//     altura: u8,
//     felicidad: bool,
// }
// fn main() {
//      let papa_doc = Persona { // se crea la variable papa_doc
//          nombre: "Papa Doc".to_string(),
//          real_nombre: "Javier".to_string(),
//          altura: 170,
//          felicidad: false,
//      };
//      println!("{:?}", papa_doc);
//      // ahora desestructuramos a la variable papa_doc
//      let Persona {
//         nombre: a,
//         real_nombre: b,
//         altura: c,
//         felicidad: d,
//      } = papa_doc;
//      println!("{} {} {} {}", a,b,c,d,);
// } 

// 📌 DESESTRUCTURAR
// #[derive(Debug)]
// #[allow(dead_code)]
// struct Ciudad {
//     nombre: String,
//     nombre_antes: String,
//     poblacion: u32,
//     fecha_fundacion: u32,
// }
// impl Ciudad {
//     fn new(nombre: String, nombre_antes: String, poblacion: u32, fecha_fundacion: u32) -> Self {
//         Self {
//             nombre,
//             nombre_antes,
//             poblacion,
//             fecha_fundacion,
//         }
//     }
// }
// fn procesar_valores_ciudad(ciudad: &Ciudad) {
//     let Ciudad {
//         nombre,
//         nombre_antes,
//         ..
//     } = ciudad;
//         // se dispone de los valores separados
//     let dos_nombres = vec![nombre, nombre_antes]; 
//     println!("Los dos nombres de la ciudad son {:?}", dos_nombres);
//     println!("{:?}{:?}", nombre_antes, nombre);
// }
// fn main() {
//     let tallinn = Ciudad::new("Tallinn".to_string(), "Reval".to_string(),  426_538, 1219);
//     procesar_valores_ciudad(&tallinn);
//     let elche = Ciudad::new("Elche".to_string(), "Elx".to_string(), 300_000, 19_04_2024);
//     println!("{:?}", elche);
// }

// 📌 REFERENCIAS Y EL OPERADOR PUNTO.
// Las referencias son una forma de referenciar un valor sin tener que moverlo, se crean con "&" seguido del nombre de la variable.
// Las referencias no pueden ser mutables y no se pueden modificar.
// Para modificar una referencia se usa "&mut" seguido del nombre de la variable.
// El operador punto "." se usa para acceder a los campos de una estructura o enum a través de una referencia.
// Para acceder al valor de una referencia se usa "*" seguido del nombre de la referencia.
// fn main() {
//     let mi_numero = 9;           // Creamos una variable "mi_numero" con valor 9.
//     let referencia = &mi_numero; // Creamos una referencia a "mi_numero".

//     println!("{}", mi_numero == referencia); // ⚠️ Comparamos "mi_numero" con el valor de la referencia, pero da error, hay que utilizar "*" en la referencia.
// }
// fn main() {
//     let mi_numero = 9;           // Creamos una variable "mi_numero" con valor 9.
//     let referencia = &mi_numero; // Creamos una referencia a "mi_numero".

//     println!("{}", mi_numero == *referencia); // 🎉 Comparamos "mi_numero" con el valor de la referencia, con el asterisco en la referencia si funciona.
// }

// 📌 REFERENCIAS Y EL OPERADOR PUNTO. 
// struct Item { // Creamos una estructura "Item" con un campo "numero" de tipo u8.
//     numero: u8,
// }
// fn main() {
//     let item = Item { // Creamos una variable item con una estructura de tipo "Item" y  con un campo "numero" de tipo u8.
//         numero: 8,    // tipo u8 Integer de 8 bits sin signo.
//     };
//     let referencia_numero = &item.numero; // el tipo de referencia_numero es &u8
//     println!("{}", *referencia_numero == 8); // ⚠️ &u8 y u8 no se pueden comparar entre sí.
// }

// 📌 REFERENCIAS Y EL OPERADOR PUNTO. 
// Con el operador punto se desreferencia automáticamente, por lo que no es necesario usar "*".
// struct Item {
//     numero: u8,
// }
// fn main() {
//     let item = Item {
//         numero: 8,
//     };
//     let referencia_item = &item; // el tipo de referencia_item es &Item
//     println!("{}", referencia_item.numero == 8); // así sí funciona sin el asterisco.
// }

// 📌 GENÉRICOS
// En las funciones genéricas se puede usar cualquier tipo de dato, se definen con "<T>" después del nombre de la función.
// Se puede usar cualquier letra o palabra después de "<" y antes de ">" para definir un genérico, normalmente se usa "T" para "Type" y "E" para "Element" y "K" para "Key"n y "V" para "Value" y "S" para "String"y "I" para "Integer".
// Se puede usar un genérico en el nombre de la función, en los argumentos y en el tipo de retorno.

// 📌 GENÉRICOS
// En las funciones es necesario especificar el tipo de dato de cada parámetro de entrada y de salida.
// fn devuelve_numero(numero: i32) -> i32 {
//     println!("Ahí va tu numero.");
//      numero
// }
// fn main() {
//     let nuevo_numero = devuelve_numero(8);
//     println!("El número es: {}", nuevo_numero);
// }

// 📌 GENÉRICOS   
// Los generics data types son una forma de definir una función que puede aceptar cualquier tipo de dato y evita duplicar código.
// fn devuelve_numero<T>(numero: T) -> T {
//     println!("Ahí va tu numero.");
//     numero
// }
// fn main() {
//     let nuevo_numero = devuelve_numero(8);
//     println!("El número es: {}", nuevo_numero);
// }

// 📌 GENÉRICOS
// Vamos a procesar una lista de números y devolver un vector con los números que sean mayores a un determinado valor
// fn numeros_mayores_a<T: std::cmp::PartialOrd>(lista: &Vec<T>, numero: T) -> Vec<&T> {
//     let mut resultado: Vec<&T> = vec![];
//     for elemento in lista {
//         if *elemento > numero  {
//             resultado.push(elemento);
//         }
//     }
//     resultado
// }
// fn main() {
//     let lista = vec![10, 12, 5, 6, 8]; // variable lista con un vector de números
//     let numero = 9;                    // variable numero con valor 9 para comparar con los números de la lista
//     let resultado =  numeros_mayores_a(&lista, numero); // llamamos a la función "numeros_mayores_a" con la lista y el número y obtenemos el resultado.
//     println!("Los números mayores a {} son {:?}", numero, resultado); // resultado es un vector con los números mayores a 9, lo imprimimos.
//     for elemento in resultado {        // creamos un bucle para imprimit los elementos del vector resultado.
//         println!("número :{}", elemento);
//     } 
// } 

// 📌 GENERICOS
// fn mayor_i32(lista: &[i32]) -> i32 {
//     let mut mayor = lista[0];
//     for &item in lista {
//         if item > mayor {
//             mayor = item;
//         }
//     }
//     mayor
// }
// fn main() {
//     let lista_numeros = vec![20,45, 77, 13,66];
//     let resultado = mayor_i32(&lista_numeros);
//     println!("El mayor numero de la lista es:{}", resultado);
// }







//  📌 CONSTANTES Y STATIC
// Las variables estáticas son como las constantes pero pueden ser mutables, se declaran con "static" y en mayúsculas, son como variables globales, no se eliminan cuando termina el bloque en el que se declaran o la función, permanecen en memoria hasta que el programa finaliza, su alcance es global se pueden acceder desde cualquier parte del programa, el timepo de vida es el mismo que el del programa, son inmutables por defecto, lo que significa que su valor no se puede modificar después de su inicialización. Sin embargo, se pueden declarar como mutables utilizando la palabra clave mut.
// ejemplo: "static mut VARIABLE_ESTATICA_MUTABLE: tipo_de_dato = valor_inicial;"
// Por convención, las constantes se suelen escribir con todas las letras en mayúsculas, normalmente están fuera del main para que existan en todo el programas.

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
//  📌 CASTING - Convertir tipos de datos de forma segura con "as"
//    fn main() {
//        let a = 13u8; // tipo de dato "u8" sin signo de 8 bits
//        let b = 7u32; // tipo de dato "u32" sin signo de 32 bits
//        let c = a as u32 + b; // convertimos "a" a "u32" y sumamos "b"
//        println!("variable \"a\" convertido a u32: {}", c); // imprimimos el resultado
// //     Doble conversión de tipo de dato:
// //     let mi_numero = 100; println!("{}", mi_numero as u8 as char) 
//        let a = 65u8; // para convertirlo a char debe ser un integer de 8 bits sin signo, 
//                   // si fuera i32 o u32 deberíamos convertirlo antes a u8.
// //     otro ejemplo de casting de tipo integer -> tipo  char, tipo booleano -> tipo integer. 
//        let falso: bool = false; // tipo de dato "bool" falso.
//        println!("0 - {} -> {}", a as char, falso as u8); // imprimimos el resultado.
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
    
// let s1 =  String::from("Bolero - Maurice Ravel");
// let s2 = s1.clone();
// f(s2);ste 
// Este código daría error si al hacer la llamada a la función f hemos transferido la propiedad 
// del valor de s1 a f. Por ello, cuando intentamos hacer el print no vamos a poder ya que 
// s1 ya no es dueña de la cadena de texto. 
// Para solucionar estos problemas tenemos los préstamos, tal y como ha quedado el código.
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



//  📌  Aqui generacidad, poner algo

//  📌  Aqui CLOSURES

//  📌  Structs, traits y POO en Rust

//  📌 ESTRUCTURAS 
// Las estructuras son un tipo de dato que permite agrupar varios campos con diferentes tipos en un solo objeto.
// Se definen con la palabra clave "struct" seguida del nombre de la estructura y los campos o atributos y sus tipos de datos.
// Se pueden crear instancias de una estructura con la palabra clave "let" seguida del nombre de la estructura y los valores.
// Se pueden acceder a los valores de una estructura con el operador "." seguido del nombre del valor.
// Se pueden modificar los valores de una estructura con la palabra clave "mut".
// Se pueden crear métodos para una estructura con la palabra clave "impl" seguida del nombre de la estructura y los métodos.

//  📌 ESTRUCTURAS e IMPLEMENTACIONES con "impl"
// #![allow(unused)] // atributo para desactivar advertencias de variables no utilizadas
// #[derive(Debug)]  // atributo para imprimir la estructura con println!("{:?}", estructura)
// #[derive(Copy, Clone)] // añadimos el atributo "Copy" para que se puedan copiar las estructuras
// struct Punto {
//     x: i32,
//     y: i32,
// }
// struct Rectangulo {
//     origen: Punto,
//     alto: i32,
//     ancho: i32,
// }
// impl Rectangulo {
//     fn area (&self) -> i32 {
//         self.alto * self.ancho
//     } 
// }
// fn main() {
//     let p = Punto {x: 50, y: 24,};
//     println!("punto x: {}", p.x);
//     let rectangulo = Rectangulo {origen: p, alto: 50, ancho: 50};
//     println!("El Area es: {}", rectangulo.area());
// }
// 📌 CALCULAR MEMORIA OCUPADA - ejemplo un char y contar los caracteres de un str.
// fn main() {
//     let a = 'A'; // tipo de dato "char" con comillas simples
//     let b = '🎉'; // Emoji también son char, gracias al Unicode otros lenguajes también son char
//     let c = "Esto es un str";
//     println!("0 - Tamaño de un char: {} bytes", std::mem::size_of_val(&a)); // imprimimos el tamaño de un char
//     println!("0 - Tamaño de un char: {} bytes", std::mem::size_of_val(&b)); // imprimimos el tamaño de un char
//     println!("0 - Tamaño en caracteres de un str: {}", c.chars().count()); // imprimimos el tamaño de un str
//     println!("0 - Otra forma de imprimir el tamaño: {}", c.len()); // imprimimos el tamaño de un str
// }
// 📌 MÉTODO .len() Calcular la longitud en bytes
//    El método len() en Rust es un método genérico que devuelve la longitud de una colección.
//    Se puede usar en vectores, arrays, slices, strings, etc.
// fn main() {

//     let vec: Vec<i32> = vec![1, 2, 3];
//     let length = vec.len(); // length == 3
//     println!("El tamaño del vector es: {}", length);  

//     let mut map: HashMap<String, i32> = HashMap::new();
//     map.insert("foo", 1);
//     map.insert("bar", 2);
//     let length = map.len(); // length == 2
//     println!("El tamaño del mapa es: {}", length); 

//     let s: String = "Hello, world!".to_string();
//     let length = s.len(); // length == 13
//     println!("El tamaño de la cadena de texto es: {}", length);
// }

























// 📌 ESTRUCTURAS e IMPLEMENTACIONES con "impl"



      

    










    





    


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

// #[allow(dead_code)] suprime las advertencias para código muerto o  no utilizado en el ámbito donde se encuentra la directiva.
// #![allow(unused)] directiva de nivel de atributo que desactiva las advertencias para el código no utilizado en todo el archivo. 
// #[ no_mangle ] // evita que el compilador cambie el nombre de la función, cuando optimice el código.
// #[derive(Debug)] // permite imprimir la estructura con println!("{:?}", estructura)
// Guión bajo (underscores) como sufijo de las variables (delante de ellas) para que no salga la advierta de "variable no utilizada
// Es una convención en Rust utilizar snake_case para: variables, funciones y archivos
// SCREAMING_SNAKE_CASE -> para constantes y estáticas, en mayusculas y guiones bajos
// PascalCas -> se utiliza para tipos, rasgos y enums
// CamelCase -> se utiliza para funciones y métodos
// Rust es un lenguaje de programación de sistemas, de bajo nivel, con un alto rendimiento y seguro
// Rust es un lenguaje de programación de propósito general, multi-paradigma, concurrente y seguro
// En Rust hay que favorecer el uso de variables locales, en lugar de globales siempre que sea posible, si necesitamos compartir datos entre funciones, se pueden usar argumentos y retornos de funciones o estructuras de datos compartidas.


   
    
// 📌 PRUEBAS arrays, vectores, 
//#![allow(unused)] // desactiva las advertencias para el código no utilizado en el ámbito donde se encuentra la directiva.
// fn main() {
// // Arrays
// let mut notas_array: [i32;5] = [0;5];   
// notas_array[0] = 1;
// println!("La nota es: {}", notas_array[0]); 
// println!("La nota es: {:?}", notas_array);
// // Vectores
// let mut notas_vec: Vec<i32> = vec![1;5];
// notas_vec.push(2);
// println!("La nota es: {}", notas_vec[0]);
// println!("La nota es: {:?}", notas_vec);
// //
// // Constantes, let shadowing y casting
// const PI:f64 = 3.14159;
// let x = 42;
// let x = (x as f64) + PI;
// println!("El valor de x es: {}", x);
// //
// // Tuplas y desestructuración
// // Podemos entender una tupla como una estructura de datos que puede contener varios valores de diferentes tipos pero sin nombre, como una especi de array con tipos de datos diferentes pero especificado de antemano.
// let tupla = (42, "hola", true); // asignamos una tupla con 3 valores
// let (valor, saludo, verdad) = tupla; // desestructuración en 3 variables
// println!("Mi saludo: {}", saludo); // imprimimos la variable saludo
// let (_, _, verdad ) = tupla; // desestructuración de la tupla en 3 variables, pero solo nos interesa la tercera
// println!("Mi saludo: {}", verdad);
// let cojo_valor = tupla.0; // accedemos al valor de la tupla con el punto y el índice
// println!("Mi valor: {}", cojo_valor);
// // Expresiones avanzadas con let
// let edad = 57;
// let compara = if edad > 57 { // el resultado de la expresión se asigna a la variable compara
//     "Eres mayor de 56"       //  si no hay punto y coma, se convierte en una expresión, no se vuelve un statement o sentencia
// } else {    
//     "Eres menor de 56"
// };
// println!("Eres: {}", compara);
// // Más expresiones avanzadas con let
// let age = 15;
// let a = 5;
// let b = 42;
// let x = {               // se complican las expresiones con let hasta niveles insospechados, pero es posible hacerlo en Rust, pero es bastante cómodo en la práctica
//     let u = a*b;
//     u+age
// };
// println!("El valor de x es: {}", x);
// }




