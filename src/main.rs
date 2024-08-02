
// use std::vec; // importamos el módulo vec de la librería estándar de Rust

// 📌   TIPOS DATOS BÁSICOS
//
// Rust es un lenguaje de programación que se destaca por su seguridad de memoria y su enfoque en la eficiencia. 
// Uno de los pilares fundamentales de Rust es su sistema de tipos, que permite al compilador verificar la corrección 
// -de tu código antes de ejecutarlo, los tipos de datos primitivos son la base de este sistema y proporcionan 
// -los bloques constructores para crear tipos más complejos.

// 📌   TIPOS DATOS BÁSICOS (Primitivos en Rust) se dividen en escalares y compuestos
// 
// Los tipos de datos primitivos son los tipos de datos básicos que el lenguaje proporciona de forma nativa. 
// Son los bloques constructores a partir de los cuales se crean tipos más complejos. 
// En Rust los tipos primitivos se dividen en escalares y compuestos.

// 📌   TIPOS DE DATOS ESCALARES: - Representan un único valor y difieren en tamaño y signo.
// 
// NÚMEROS ENTEROS SIN SIGNO: -> u8, u16, u32, u64, u128, usize
//
// u8    = (0 al 255) ocupa 1 byte en la pila
// u16   = (0 al 65,536) ocupa 2 bytes en la pila
// u32   = (0 al 4,294,967,296) ocupa
// u64   = (18,446,744,073,709,551,616) ocupa 8 bytes en la pila
// u128  = (3.40282367 × 10^38) ocupa 16 bytes en la pila
// usize = ( normalmente 2^64-1 para representar números enteros positivos) ocupa 8 bytes en la pila
// -para representar índices, punteros y tamaños de elementos en memoria, depende de la arquitectura
// -un índice no puede ser negativo, por lo que tiene que ser uno de los tipos de dato con una "u" delante.
//
// NÚMEROS ENTEROS CON SIGNO: -> i8, i16, i32, i64, i128, isize -> ocupan lo mismo en la pila que los números sin signo.
//
// i8    = (-128 al 127)
// i16   = (-32,768 al 32,767)
// i32   = (-2,147,483,648 al 2,147,483,647)
// i64   = (-9,223,372,036,854,775,808 <- al -> 9,223,372,036,854,775,807)
// i128  = (-170,141,183,460,469,231,731,687,303,715,884,105,727 <- al -> 170,141,183,460,469,231,731,687,303,715,884,105,727)
// isize = (normalmente 2^63-1 para representar números enteros positivos y negativos) 
// -para representar índices, punteros y tamaños de elementos en memoria, depende de la arquitectura.
//
// NÚMEROS FLOTANTES: -> f32, f64 representan números de coma flotante de precisión simple (32) y doble (64).
// f32 = (32 bits)
// f64 = (64 bits)
//
// BOOLEANOS: -> bool, representan valores lógicos: true o false, ocupan 1 byte en la pila independientemente de su valor.
// true = verdadero
// false = falso
//
//
// Para manejar explícitamente la posibilidad de desbordamiento, puede usar estas familias de métodos 
// -proporcionados por la biblioteca estándar para tipos numéricos primitivos:
//
// Envolver en todos los modos con los métodos wrapping_*, como wrapping_add.
// Devolver el valor None si hay desbordamiento con los métodos checked_*.
// Devolver el valor y un booleano que indica si hubo desbordamiento con los métodos overflowing_*.
// Saturar en los valores mínimos o máximos del valor con los métodos saturating_*.

// 📌   CARACTERES: -> (char UTF-8) representa un único carácter Unicode, entre comillas simples.
// Un char siempre ocupa 4 bytes en la pila, independientemente de su valor, es de tamaño fijo.
// Puede representar mucho más que ASCII, letras y números hasta emojis y caracteres especiales, 
// caracteres chinos, etc. y se utiliza para almacenar un solo carácter Unicode
//
// Las letras y símbolos básicos suelen necesitar solo 1 de los 4 bytes 
// Otras letras como las diéresis y tildes necesitan 2 de los 4 bytes
// Los caracteres coreanos, japoneses o chinos necesitan 3 de los cuatro bytes
// Cuando los caracteres se usan como parte de una cadena, esta se codifica para usar la menor 
// -cantidad de memoria necesaria para cada carácter.
//
// Podemos convertir desde i32 a u8 y viceversa, pero no de i32 a char y viceversa.
// Desde u8 con "as" se puede convertir a char, pero no al revés.. 
// El espacio en blanco también es un carácter Unicode. 
// Se puede utilizar para almacenar, comparar, convertir y mostrar caracteres Unicode.
// Se puede utilizar el tipo char para almacenar un único carácter Unicode en una variable. 
// -esto puede ser útil para trabajar con cadenas de texto o para procesar caracteres individuales.
// Se pueden comparar dos valores char para determinar si son iguales o diferentes. 
// Puede ser útil para ordenar cadenas de texto o para buscar caracteres específicos en una cadena.
// Se pueden convertir valores char a otros tipos de datos, como String o u8. 
// En general puede ser útil para trabajar con datos de cadena o para almacenar caracteres en formato binario.

// 📌   MÉTODOS MAS USADOS DE LOS CARACTERES:
// .to_string() -> convierte un carácter en una cadena.
// .as_u8() -> convierte un carácter en un valor u8.
// .is_alphabetic() -> determina si un carácter es una letra.
// .is_numeric() -> determina si un carácter es un número.
// .is_alphanumeric() -> determina si un carácter es una letra o un número.
// .is_whitespace() -> determina si un carácter es un espacio en blanco.
// .is_ascii() -> determina si un carácter es un carácter ASCII.
// .is_uppercase() -> determina si un carácter es una letra mayúscula.
// .is_lowercase() -> determina si un carácter es una letra minúscula.
// .to_ascii_uppercase() -> convierte un carácter en mayúsculas.
// .to_ascii_lowercase() -> convierte un carácter en minúsculas.
// .to_uppercase() -> convierte un carácter en mayúsculas.
// .to_lowercase() -> convierte un carácter en minúsculas.
// .escape_unicode() -> escapa un carácter en formato Unicode.
// .escape_debug() -> escapa un carácter en formato de depuración.
// .escape_default() -> escapa un carácter en formato predeterminado.
// .len() -> determina la longitud de un carácter en formato Unicode.
// .len_utf8() -> determina la longitud de un carácter en formato UTF-8.
// .chars() -> convierte un carácter en un iterador de caracteres.
// .encode_utf8() -> codifica un carácter en formato UTF-8.
// .count() -> cuenta la cantidad de caracteres en un carácter.
// .next() -> obtiene el siguiente carácter en un iterador de caracteres.
// .nth() -> obtiene el carácter en una posición específica en un iterador de caracteres.
// .is_control() -> determina si un carácter es un carácter de control.

// 📌   CALCULAR MEMORIA OCUPADA por CHAR - ejemplo un char y contar los caracteres de un str.
//
// fn main() {
//     let a = 'A'; // tipo de dato "char" con comillas simples
//     let b = '🎉'; // Emoji también son char, gracias al Unicode otros lenguajes también son char
//     let c = "Esto es un str";
//     println!("0 - Tamaño de un char: {} bytes", std::mem::size_of_val(&a)); // imprimimos el tamaño de un char
//     println!("0 - Tamaño de un char: {} bytes", std::mem::size_of_val(&b)); // imprimimos el tamaño de un char
//     println!("0 - Tamaño en caracteres de un str: {}", c.chars().count()); // imprimimos el tamaño de un str
//     println!("0 - Otra forma de imprimir el tamaño: {}", c.len()); // imprimimos el tamaño de un str
// }

// 📌   TIPOS DE DATOS COMPUESTOS: -> agrupan múltiples valores en un solo tipo Tuplas y Arrays
//
// Rust tiene dos tipos de datos compuestos primitivos las Tuplas y los Arreglos (Arrays).
//
// TUPLAS: -> (Valor1, Valor2, Valor3, ..., ValorN) Es una colección que agrupa múltiples valores de DIFERENTES TIPOS
// -ordenados y de tamaño fijo en un solo tipo.
// Los valores de una tupla se llaman elementos y están encerrados entre paréntesis 
// -pasan a ser un solo (tipo/valor) y se guardan en la pila.
//
// ARRAYS: -> [Valor1, Valor2, Valor3, ..., ValorN] Es una colección que agrupa múltiples valores del MISMO TIPO
// -ordenados y de tamaño fijo en un solo tipo.
// Los valores de un array se llaman elementos y están encerrados entre corchetes
// -pasan a ser un solo (tipo/valor) y se guardan en la pila.

// 📌   TIPOS DE DATOS COMPUESTOS
// 
// Tipos compuestos pueden agrupar múltiples valores en un solo tipo, tenemos dos tipos compuestos primitivos: tuplas y arreglos.
//
// TIPO TUPLA: 
// Una tupla es una colección de valores de diferentes tipos. Los valores de una tupla se llaman elementos.
// Los elementos de una tupla pueden ser de cualquier tipo, incluidos otros tipos compuestos como tuplas y arreglos.
// Los elementos de una tupla se pueden acceder mediante índices, que comienzan en 0.
// podemos acceder directamente a un elemento de la tupla usando un punto (.) seguido del índice del valor que queremos acceder. 
// Los elementos de una tupla se pueden desestructurar para asignarlos a variables individuales.
//
// fn main() {
//     let tup = (500, 6.4, 1);                 // tupla con tres elementos
//     let (x, y, z) = tup;                     // desestructuración de la tupla
//     println!("The value of y is: {y}");      // impresión de un valor de la tupla
// }
//
// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);   // tupla con tres elementos
//     let five_hundred = x.0;                  // acceso a un elemento de la tupla
//     let six_point_four = x.1;                // acceso a otro elemento de la tupla
//     let one = x.2;                           // acceso al último elemento de la tupla
// }
// 
// TIPO ARREGLO (Array): 
// Un arreglo es una colección de valores del mismo tipo. Los valores de un arreglo se llaman elementos
// Los elementos de un arreglo tienen un tamaño fijo, que se establece en tiempo de compilación.
// -estos se pueden acceder mediante índices, que comienzan en 0
// Se pueden desestructurar para asignarlos a variables individuales.
// Se pueden acceder directamente usando un índice entre corchetes "[0]".
//
// Los arreglos son más útiles cuando sabe que el número de elementos no cambiará, oor ejemplo, si está utilizando los nombres 
// del mes en un programa, probablemente usaría un arreglo en lugar de un vector porque sabe que siempre contendrá 12 elementos:
//
// ejemplo:.  let meses = ["Enero", "Febrero", "Marzo", "Abril", "Mayo",
//              "Junio", "Julio", Agosto", "Septiembre", "Octubre", "Noviembre", "Diciembre"];
//
// ejemplo:.  let a: [i32; 5] = [122, 12, 63, 43, 5]; -> tipo = i32, longitud = 5 elementos 

// 📌   INFERENCIA DE TIPOS
//
// La inferencia de tipos de datos consiste en que si no se le indica el tipo al compilador
// -el lo puede determinar por sí mismo, él decide que tipo utilizar. El compilador siempre necesita 
// conocer el tipo de las variables, pero no siempre es necesario decírselo expresamente.
//
// Para especificar un tipo, se añaden dos puntos después del nombre de la variable seguido del tipo.
// Para los números, se puede especificar el tipo después del número, no se necesita un espacio 
// solo teclearlo justo después del número.
//
// También se puede añadir _ para añadir claridad a la lectura.
// El _ no modifica el número. Solo lo hace más fácil de leer. Y no importa el cuantos _ se utilizan.
// ejemplo:.
//
// fn main() {
//     let numero = 0________u8;
//     let numero2 = 1___6______2____4______i32;
//     println!("{}, {}", numero, numero2);
// }
//
// El tipo numérico por defecto suele ser i32, si se quiere otro tipo se debe especificar.

// 📌   INFERENCIA DE TIPOS  
// 
// Para la inferencia de tipos en los números utilizamos el tipo seguido del valor
// ->  ejemplo:. let a = 13u8; // tipo de dato "u8" sin signo de 8 bits.
// también se puede añadir un guió bajo para separar los números y hacerlos más legibles
// ->  ejemplo:. let a = 1_000_000; // tipo de dato "i32" con signo de 32 bits.
// ->  ejemplo:. let b = 24_u8; // tipo de dato "u8" sin signo de 8 bits.
// El guión bajo no afecta al valor, solo es para hacerlo más legible, se pueden poner varios guiones bajos en cualquier posición.

// 📌   VARIABLES - Declaración y los bloques de código
// todo!(cambiar valores de las variables y ver como se comporta el programa y los errores que da)
// todo!(cambiar valores de los ejemplos para el libro)
// Se usa la palabra reservada let para declarar una variable (para decirle a Rust que construya una variable).
// Las variables se declaran con let y se pueden reasignar, pero desaparecen al salir del bloque.
// Las variables se pueden declarar sin valor, pero se debe especificar el tipo de dato.
// Las variables se pueden declarar en cualquier ámbito, incluido el global.
// Las variables existen dentro de un bloque de código {} Y desaparecen al salir del bloque, a no ser que sean estáticas.
// Las variables son inmutables por defecto, para hacerlas mutables se debe añadir mut después de let.
// A tener en cuenta con las variables -> la Inferencia, Alcance, Mutabilidad, Sombreado, Constantes, 
// -Estáticas, la Pila y la Memoria Dinámica.
//
// Usos comunes: para almacenar valores, para realizar cálculos, para almacenar referencias, para almacenar punteros
// como patron de coincidencia, para almacenar valores de retorno, para almacenar valores de entrada, para almacenar 
// valores de salida y para extraer valores de estructuras de datos como tuplas, enumeraciones y opciones.
//
// fn main() {
//     let primera_letra = 'A'; // tipo de dato "char" con comillas simples
//     let espacio = ' '; // tipo de dato espacio en blanco "char" con comillas simples
//     let cono_fiesta = '🎉'; // Emoji también son char, gracias al Unicode otros lenguajes también son char
//     print!("0 - {} {} {}\n", primera_letra, espacio, cono_fiesta);
// }
//
// // ALMACENAR un carácter en una variable
// fn main() { let caracter: char = 'a'; println!("{}", caracter);}
//
// // IMPRIMIR CARACTERES
// fn main() {
//     let primera_letra = 'A'; // tipo de dato "char" con comillas simples
//     let espacio = ' '; // tipo de dato "char" con comillas simples
//     let cono_fiesta = '🎉'; // Emoji también son char, gracias al Unicode otros lenguajes también son char
//     print!("0 - {} {} {}\n", primera_letra, espacio, cono_fiesta);
// }
//
// // COMPARAR dos caracteres
// fn main() {
//     let caracter1: char = 'b';
//     let caracter2: char = 'c';
//     if caracter1 == caracter2 {
//     println!("Los caracteres son iguales");
//     } else {
//     println!("Los caracteres son diferentes");
//     }
// }
//
// fn main() {
// // CONVERTIR un carácter a una cadena
//     let caracter: char = 'd';
//     let cadena = caracter.to_string();
//     println!("La cadena es: {}", cadena)
// // CONVERTIR un carácter a un valor u8
//     let caracter: char = 'e';
//     let valor_u8: u8 = caracter as u8;
//     println!("El valor u8 es: {}", valor_u8);
// }

// 📌   VARIABLES - ámbito de una variable
//
// Se pueden asignar variables sin valor, pero se debe especificar el tipo de dato, ej:. let a: i32;
// Las variables existen dentro de un bloque, se declaran con "let" y se pueden reasignar, pero 
// -desaparecen al salir del bloque la linea de impresión de "b" da error porque no existe fuera del bloque
//
// fn main() {
//     let a = 42;
//     {
//         let _b = 13;
//     }
//     println!("Valor de a: {}", a);
//     println!("Valor de b: {}", _b); // 🤣 ERROR, b no existe fuera del bloque
// }

// 📌   VARIABLES CON NÚMEROS DECIMALES 
//
// Se utiliza el punto y NO la coma, se utilizan dos tipos f32 y f64, por defecto f64
// El compilador de Rust es inteligente y no elegirá f64 si necesitas f32
//
//    ejemplo:. let a = 42.0; // tipo de dato "f64" con coma flotante de 64 bits.
//    ejemplo:. let b = 42.0f32; // tipo de dato "f32" con coma flotante de 32 bits.
//    ejemplo:. let c = 1_000.0; // tipo de dato "f64" con coma flotante de 64 bits.
//    ejemplo:. let d = 1_000.0f32; // tipo de dato "f32" con coma flotante de 32 bits.
//
// fn main() {
//     let mi_decimal: f32 = 5.0; // Rust elige f64
//     let mi_otro_decimal = 8.5; // Normalmente Rust elegiría f64
//     // pero al conocer que lo vamos a sumar a un f32, elige un f32 para mi_otro_decimal
//     let tercer_decimal = mi_decimal + mi_otro_decimal;
// }

// 📌   VARIABLES Y MUTABILIDAD
// 
// Las variables son inmutables por defecto, para hacerlas mutables se debe añadir mut después de let.
// Las variables inmutables no se pueden cambiar una vez que se les ha asignado un valor.
// Las variables mutables se pueden cambiar después de haber sido asignadas.
// Las variables mutables se pueden reasignar, pero no se puede cambiar su tipo.
// Las variables mutables se pueden cambiar en cualquier momento, pero no se pueden cambiar de nuevo a inmutables.
// Las variables mutables se pueden cambiar en cualquier ámbito, pero no se pueden cambiar en un ámbito inmutable.
// Las variables mutables se pueden cambiar en cualquier bloque de código, pero no se pueden cambiar en un bloque inmutable.
// Las variables mutables se pueden cambiar en cualquier función, pero no se pueden cambiar en una función inmutable.
// Las variables mutables se pueden cambiar en cualquier módulo, pero no se pueden cambiar en un módulo inmutable.
// Las variables mutables se pueden cambiar en cualquier archivo, pero no se pueden cambiar en un archivo inmutable.
// Las variables mutables se pueden cambiar en cualquier proyecto, pero no se pueden cambiar en un proyecto inmutable.
// Las variables mutables se pueden cambiar en cualquier biblioteca, pero no se pueden cambiar en una biblioteca inmutable.

// 📌   VARIABLES Y MUTABILIDAD
//
// Para poder modificar la variable se debe añadir mut después de let
//
// fn main() { 
//     let mut number = 5; // mut proporciona mutabilidad a la variable en cuanto al dato, pero no podemos cambiar el tipo de dato
//                         // salvo que hagamos shadowing (ocultación) de la variable.
//     number += 1;
//     println!("valor que reemplaza el anterior '5' por misma variable: {}",number);
// }

// 📌   VARIABLES - copia
//
// Rust tiene una característica especial para los tipos de datos primitivos, la trait "Copy" 
// que permite que los valores se copien en lugar de moverse.
//
// Son valores de tamaño fijo, conocidos y pequeños que se almacenan en el stack (enteros,flotantes y char) 
// -y no en el heap, por lo que son rápidos de copiar y no influye que existan varias copias de lo mismo.
//
// Pueden copiarse cuando se pasan como argumentos a una función, se asignan a otra variable o se devuelven de una función.
//
// fn print_number(number: i32) { // Esta función no devuelve nada
//     // Si el  número no se copiara, se movería y no se podría usar, la función seria su dueña.                   
// println!("{}", number);
// }
//
// fn main() {
//     let mi_numero = 8;
//     print_number(mi_numero); // Imprime 8, la función obtiene una copia del valor de "mi_numero"
//     print_number(mi_numero); // Imprime 8 de nuevo, la función obtiene una copia automaticamente del valor de "mi_numero".
// }

// 📌   VARIABLES - clone
//
// El tipo String, no implementa la característica copy por lo que el valor de la variable se mueve 
// -al pasarla la primera vez, para poder copiarla se usa la trait "Clone".
//
// Lo ideal es utilizar la referencia es más eficiente porque clone copia el valor gastando más memoria y la referencia solo el puntero. 
//
// fn print_country(country_name: String) {         // Esta función no devuelve nada
//     println!("{}", country_name);
//     }
//
// fn main() {
//         let country = String::from("España");
//         print_country(country.clone());
//         print_country(country);
// }

// 📌   CADENA DE CARACTEREs - String y &str
//
// Rust tiene dos tipos de datos para representar cadenas de caracteres: String y &str.
// Ambos tipos de datos se utilizan para almacenar cadenas de caracteres, pero tienen diferencias importantes.
// Los dos son UTF-8
// &str es una cadena de caracteres, una referencia a un bloque de memoria en la pila 
// -que contiene la cadena de caracteres.
// Cuando se escribe let mi_variable = "¡Hola, mundo!" se crea una &str, este tipo es muy rápido.
//
// &str tiene "&" como primer carácter debido a que es necesaria una referencia para utilizar el tipo str. 
// -esto es porque necesita conocer el tamaño, así que se le da una referencia, las referencias siempre tienen el mismo tamaño.
// Al utilizar & una referencia para interactuar con el tipo str, nunca se es dueño del tipo
// 
// String es una cadena de caracteres que reside en el heap, es más complejo pero tiene mas funciones.
// Un String es un puntero a un bloque de memoria en el heap que contiene la cadena de caracteres.
// Cuando se escribe let mi_variable = String::from("¡Hola, mundo!") se crea un String, este tipo es más lento.
// String siempre es dueño de la cadena de caracteres, por lo que se encarga de liberar la memoria cuando ya no se necesita.
//
// ejemplo:. let mi_variable1 = "¡Hola, mundo!"; // tipo de dato "&str" 
// ejemplo:. let mi_variable2 = String::from("¡Hola, mundo!"); // tipo de dato "String"
//
// Por eso es necesario usar &, porque así se construye un puntero (tipo de tamaño fijo) que puede almacenarse 
// -en la pila. Si se escribiera str, Rust no sabría qué hacer al no conocer su tamaño.
//
// std::mem::size_of_val() devuelve el tamaño en bytes de una variable
//
// Hay varias formas de construir un String, la más común es con el método from de la clase "String::from()".
// También se puede construir un String a partir de un &str con el método ".to_string()".
//
// Otra forma de construir un String es con el método ".to_owned()" que convierte un &str en un String.
// También se puede construir un String con la macro "format!" que permite formatear una cadena de caracteres.
// 

// 📌   CONSTANTES
//
// Las constantes son valores inmutables que se pueden definir en cualquier ámbito, incluidos los globales.
// Se definen con la palabra clave "const" y se les debe asignar un tipo de dato.
//
// Se les debe asignar un valor constante, no se les puede asignar un valor que se calcule en tiempo de ejecución.
// Se escriben en mayúsculas y con guiones bajos para separar las palabras.
// Las constantes no pueden ser sombreadas por variables con el mismo nombre, son validas en todo el tiempo 
// -de vida del programa dentro del ámbito en el que se declararon y se pueden declarar en cualquier ámbito, incluido el global.
//
// SCREAMING_SNAKE_CASE -> para constantes y estáticas, en mayusculas y guiones bajos
// ejemplo: const MAX_POINTS: u32 = 100_000; // constante de tipo u32 con valor 100_000

// 📌   SHADOWING - Ocultación
//
// Recordamos que el ocultamiento de variables no destruye la variable anterior, solo la bloquea, la oculta, "shadowing" 
// -con el uso de referencias se puede acceder a la variable anterior, solo si no se cambio el tipo de dato 
// -o esta en un bloque diferente.
// En general, se usa la ocultación de variables en estos casos, cuando se quiere usar una variable para un cálculo 
// -y luego otro más, sin tener mucho interés por los valores intermedios.
//
// fn main() {
//     let pais = String::from("España"); // variable "pais" con valor "España"
//     let pais_ref = &pais; // variable "pais_ref" con referencia a "pais"
//     let pais = 8; // redefinimos la variable "pais" con el valor 8
//     println!("{}, {}", pais_ref, pais); // imprimimos el valor de "pais_ref" y "pais"
//     // hemos ocultado la variable "pais" con otra variable de diferente tipo y valor
//     // no se destruye la variable anterior, solo se bloquea, se oculta, "shadowing"
//     // la variable pais se destruirá al salir del bloque,
// }

// 📌   SHADOWING - Ocultación
//
// fn main() {
//     let number = 5;                          // variable "number" con valor 5
//     println!("Valor de number: {}", number); // imprimimos el valor de "number" = 5
//     let number = 9.9;            // redefinimos la variable "number" con el valor 9,8 y de tipo f64, pero es completamente diferente
//     println!("Valor de number: {}", number); // imprimimos el valor de "number" = 9.9
//     // hemos ocultado la variable "number" con otra variable de diferente tipo y valor
//     // no se destruye la variable anterior, solo se bloquea, se oculta, "shadowing"
//     // ejemplo de utilidad: para hacer varios calculos con la misma variable.
// }

// 📌   IMPRESIÓN - VISUALIZACIÓN Y DEPURACIÓN
// 
// Para imprimir valores en la consola se utilizan las macros println! y print!
// Con print! se imprime sin salto de línea, con println! se imprime con salto de línea
//
// Las macros son similares a las funciones, pero se invocan con un signo de exclamación (!)
// Dentro de los paréntesis van los valores y variables a imprimir, se pueden usar llaves {} para imprimir valores o variables
//
// ejemplo:. println!("El valor de la variable es: {}", variable);
//
// Para obtener el tamaño de un tipo de dato en bytes, se puede usar -> std::mem::size_of_val(&variable);
//
// La variable puede ir dentro de las llaves, se pueden imprimir varias variables incluso repetirlas 
// - también se pueden imprimir valores directos
//
// Existen variables que no se pueden imprimir usando {} con la macro println! aquí es necesario usar 
// la impresión de depuración {:?} como por ejemplo los vectores, las estructuras y enumeraciones 
// -para ello se usa "{:?}" llamada "pretty print" o representación de depuración.
// Esta forma {#:?} se llama "pretty print" y es muy útil para depurar
//
// Para visualizaciones más sofisticadas, como gráficos o diagramas, puedes utilizar bibliotecas externas 
// Para los booleanos, para ello se usa "{:}"ara estas visualizaciones para gráficos interactivos 
// -en 2d o 3d e interfaces de usuario gráficas.
//
// 
// ejemplo:. -> println!("El menor i8 es {} y el mayor i8 es {}.", std::i8::MIN, std::i8::MAX); // tipo de dato i8
//
// fn main() {
//      let variable = 42; // variable "variable" con valor 42
//      println!("El menor de i8 es \"{}\" y el mayor es \"{}\"", i8::MIN, i8::MAX);
//      // así con todos los tipos de datos, por ejemplo: i16, u16,i32,u32,i64,u64,i128,u128
//      println!("tamaño de la variable: {}",std::mem::size_of_val(&variable));
//      // tamaño del tipo de dato "variable" en bytes
// } 

// 📌   IMPRESIÓN - VISUALIZACIÓN Y DEPURACIÓN - MACRO "println!" - Display the message "Hello, world!"
// fn main() { 
//     println!("Hello, world!");
// }

// 📌   IMPRESIÓN - VISUALIZACIÓN Y DEPURACIÓN - Pasar argumentos a la macro println!() entre corchetes "brakes"
//
// Llamar a macro println! con 4 argumentos: string, valor, string, valor
//
// fn main() { 
//     println!("2 - {} - {} - {} - {}", "Hola", 42, "mundo", 13);
// }

// 📌   IMPRESIÓN - VISUALIZACIÓN Y DEPURACIÓN - Aquí los tipos los e implementado yo ...
//
// Aquí los tipos los e implementado yo, después de la variable insertar los ":" y el tipo.
//
// fn main() {
//     let _x: u32 = 42;                                                    // integer de 32 bits sin signo
//     let url: String = String::from("http://desarrollosdigitales.info");  // tipo "String"
//     let a_url: &str = "http://desarrollosdigitales.info";                // tipo texto "&str"
//     println!("Esto es un String: {}", url);
//     println!("Esto es un Texto &str: {}", a_url);
// }

// 📌   IMPRESIÓN - VISUALIZACIÓN Y DEPURACIÓN - Valores directos y con variables
//
// fn main() {
    // println!("Valor directo sin pasar variable: {}", 42); // imprimir valor directo
    // let a = 42;
    // println!("Valor con variable a: {} {}", a, 32); // imprimir variable "a"
// }

// 📌   IMPRESIÓN - VISUALIZACIÓN Y DEPURACIÓN - Se pueden añadir números entre las llaves ...
//
// Se pueden añadir números entre las llaves para indicar el orden de las variables a utilizar.
//
// fn main () {
//     let nombre_padre = "Juan";
//     let nombre_hijo = "Martinez";
//     let apellido = "Pérez";
//     println!("Este es {1} {2}, hijo de {0} {2}.", nombre_padre, nombre_hijo, apellido);
// }

// 📌   IMPRESIÓN - VISUALIZACIÓN Y DEPURACIÓN - Imprimir una cadena de caracteres compleja con muchas variables
//
// Imprimir una cadena de caracteres compleja con muchas variables dentro de las llaves
// O puede que se necesite imprimir la misma variable dos o más veces. 
// Para ello, se pueden añadir nombres a las llaves
//
// ejemplo:. 
//
//  fn main() {
//     println!(
//         "{city1} está en {pais} y {city2} también está en {pais},
// pero {city3} no está en {pais}.",
//         city1 = "Seul",
//         city2 = "Busan",
//         city3 = "Tokio",
//         pais = "Korea"
//     );
// }
//

// 📌   IMPRESIÓN - VISUALIZACIÓN Y DEPURACIÓN - Impresión de Variables y una tupla de diferentes tipos
// 
// fn main() {
//     let text1 = "La primera letra del alfabeto";
//     let text2 = "La última letra del alfabeto:";
//     let sa = (13, false);
//     println!("{}: {} y {} {} y \n - array pos 0 = {} \n - array pos 1 = {}", text1, 'A', text2, 'Z', sa.0, sa.1);
// }

// 📌   IMPRESIÓN - VISUALIZACIÓN Y DEPURACIÓN - Formatos
//
// "\n" -> salto de línea
// "\t" -> tabulación
// "\r" -> retorno de carro
// "\0" -> fin de cadena
// "\xNN" -> caracter ASCII
// "\u{NNNN}" -> caracter Unicode
// "{}" -> valor de la variable
// "{:b}" -> valor de la variable en binario
// "{:o}" -> valor de la variable en octal
// "{:x}" -> valor de la variable en hexadecimal
// "{:X}" -> valor de la variable en hexadecimal en mayúsculas
// "{:p}" -> puntero de la variable, imprime la dirección de memoria, cuando se imprime una referencia
// "{:e}" -> valor de la variable en notación científica
// "{:E}" -> valor de la variable en notación científica en mayúsculas
// "{:?}" -> valor de la variable en formato de depuración modo display 
// "{:#?}" -> valor de la variable en formato de depuración con formato modo display bonito
// "{:.N}" -> valor de la variable con N decimales
// "{:.*}" -> valor de la variable con decimales variables
// "{:t}" -> valor de la variable en binario, octal, hexadecimal, etc.
// "{:c}" -> valor de la variable como carácter
//
// Los valores numéricos se pueden imprimir en binario, octal, hexadecimal, etc.
// Se pueden añadir números entre las llaves para indicar el orden de las variables a utilizar
// Se pueden añadir nombres de variables en las llaves, o incluso números, o repetirlos
// Se puede indicar el número de decimales a imprimir
// Se puede imprimir el código ASCII de las letras a imprimir con b"texto"
//
// fn main() {
//   const PI:f32 = 3.14159265359;       // recordamos que las constantes se declaran con "const" y en mayúsculas
//   println!("Valor de pi: {:.2}", pi); // imprime el valor de "pi" con dos decimales
// }
//
// fn main() {
//     let a = 42; // variable "a" con valor 42
//     println!("Valor de \"a\" en binario: {:b}", a);          // imprimimos el valor de "a" en binario
//     println!("Valor de \"a\" en octal: {:o}", a);            // imprimimos el valor de "a" en octal
//     println!("Valor de \"a\" en hexadecimal: {:x}", a);      // imprimimos el valor de "a" en hexadecimal
// }
//
// Editar de forma compleja el formato de la impresión -> {variable:relleno alineamiento mínimo.máximo}
//
// ejemplo:.
// 
//  fn main() {
//     let titulo = "NOTICIAS DE HOY";
//     // sin variable, relleno con -, centrado, longitud de 30 caracteres
//     println!("{:-^30}", titulo); 
//     let barra = "|";
//     // sin variable, relleno con espacios, 15 caracters cada uno, una barra a izquierda y otra a derecha
//     println!("{: <15}{: >15}", barra, barra); 
//     let a = "SEUL";
//     let b = "TOKIO";
//      // variables city1 y city2, relleno con -, a izquierda y a derecha
//      println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
// }

// 📌   IMPRESIÓN - VISUALIZACIÓN Y DEPURACIÓN - Avanzado 
//
// "#r" antepuesto a la variable le permite utilizar nombres reservados, ej, como let, fn, struct, etc.
// ejemplo:.
//
// fn main() {
//     let r#let = 6; // let como nombre de variable
//     let mut r#mut = 10; // Esta variable se llama mut
//     println!("{} {}", r#let, r#mut);
// }
//
// otro ejemplo de utilizar una palabra reservada como nombre de variable
//
//  fn r#return() -> u8 {
//     println!("Ahí va tu número.");
//     8
// }
//
// fn main() {
//     let mi_numero = r#return();
//     println!("{}", mi_numero);
// }
//
// "#r", a veces se necesita imprimir muchas comillas dobles " y caracteres de escape en el texto
// -para ello se usa al comienzo "#r" antes de las primeras comillas y "#" al final de las últimas comillas.
//
// para escapar un slash se usa "\\" y para escapar una comilla doble se usa "\""
// para escapar una comilla simple se usa "\'"
// para escapar un tabulador se usa "\t"
// para escapar un salto de línea se usa "\n"
//
// Si se necesita imprimir el caracter "#" se debe usar "##" al comienzo de la cadena y "##" al final
// Si se usaran más de dos consecutivos, se pueden seguir añadiendo # al comienzo y al final, hasta 
// -que no coincida con nada contenido en el texto.
//
// Se puede imprimir el código ASCII de las letras a imprimir con b"texto"
// Lo siguiente imprime los códigos ASCII de todas las letras a imprimir, tienen que ser solo vocales y no llevar tilde.
//
//  fn main() {
//     println!("{:?}", b"Esto es un texto");      // Se puede imprimir el código ASCII de las letras a imprimir con b"texto"
//  }
// //
// Se pueden poner nombres de variables en las llaves, ej:. "{ciudad}", "{pais}", "{provincia}", etc. 
// -o incluso números, ej:. "{1}", "{2}", "{3}", etc. o repetirlos, ej:. "{ciudad}", "{ciudad}", "{ciudad}", etc.
//
// fn main() {
//     let ciudad = "Elche";
//     let pais = "España";
//     let provincia = "Alicante";
//     println!("Ciudad: {ciudad}, País: {país}, Provincia: {provincia} Este repite ciudad -> {ciudad}");
// }  

// 📌   IMPRESIÓN - VISUALIZACIÓN Y DEPURACIÓN - Imprimiendo valores pasados por valor o por referencia
//
// Imprimiendo Valores Pasados por Valor
// Cuando pasamos un valor por valor a una función, podemos imprimirlo directamente dentro de esa función.
// En este caso, la función recibe el valor y lo imprime directamente, sin necesidad de devolverlo.
// ejemplo:.
// fn imprimir_valor(x: i32) {
//     println!("El valor es: {}", x);
// }
// fn main() {
//     let num = 42;
//     imprimir_valor(num);
// }
//
// Imprimiendo Valores Pasados por Referencia
// Cuando pasamos un valor por referencia a una función, podemos imprimirlo dentro de esa función.
// En este caso, la función recibe una referencia al valor y lo imprime directamente, sin necesidad de devolverlo.
// ejemplo:.
// fn imprimir_valor(x: &i32) {
//     println!("El valor es: {}", x);
// }
// fn main() {
//     let num = 42;
//     imprimir_valor(&num);
// }

// 📌   IMPRESIÓN - VISUALIZACIÓN Y DEPURACIÓN - Imprimiendo con la macro "format!()" - Para crear un String
//
// fn main() {
//     let s = format!(" Hello, world!");
//     println!("{}", s);
//     // otra forma.
//     let nombre = "Javier";
//     let apellido = "García";
//     let ciudad = "Elche";
//     let imprimir = format!("Hola, soy {} {} y vivo en {} ", nombre, apellido, ciudad);
//     println!("{}", imprimir);
//     // Otra forma de construir un string con .into() y .to_string()
//     let mi_string = "Hola, mundo".to_string();     // de esta forma se debe especificar el tipo de dato.
//     let mi_string2: String = "Hola, mundo".into(); // de esta forma se debe especificar el tipo de dato.
//     println!("mi_string: {} y mi_string2: {}", mi_string, mi_string2);
// }

// 📌   IMPRESIÓN - VISUALIZACIÓN Y DEPURACIÓN - Imprimiendo el retorno de una función
//
// fn numero() -> i32 {     // función que devuelve un valor de tipo i32, en este caso 8, al no tener punto y coma.
//    8
// }
//
// fn main() {
//   println!("¡Hola, mundo número {}!", numero()); // llamamos a la función numero() dentro de la macro y la imprimimos.
// }

// 📌   IMPRESIÓN - VISUALIZACIÓN Y DEPURACIÓN - Temas relacionados
//
//
// Impresión personalizada con std::fmt: Crear formatos personalizados para tus tipos de datos.
// Impresión de estructuras recursivas: Cómo manejar estructuras que se refieren a sí mismas.
// Impresión de resultados de cálculos: Formatear números con precisión y unidades.

// 📌   FUNCIONES
//
// Las funciones son bloques de código reutilizables que realizan una tarea específica, son fundamentales para
// -organizar el código, mejorar la legibilidad y facilitar el mantenimiento de programas más grandes.
//
// Las funciones se definen con la palabra clave fn seguida del nombre de la función y los paréntesis 
// -junto a su contenido encerrado entre llaves.
//
// ejemplo:. fn nombre_funcion() { // código de la función }
// 
// Se pueden pasar argumentos a las funciones, que se especifican entre los paréntesis.
// Se pueden devolver valores de las funciones, que se especifican después de los paréntesis.
// Las funciones pueden ser llamadas desde otras funciones o desde el código principal.
// Las funciones pueden ser reutilizadas en diferentes partes del código.
// Las funciones pueden ser recursivas, es decir, llamarse a sí mismas.
// Las funciones pueden ser anidadas, es decir, definirse dentro de otras funciones.
// Las funciones pueden ser genéricas, es decir, aceptar cualquier tipo de dato.
// Las funciones pueden ser de alto orden, es decir, aceptar otras funciones como argumentos.
// Las funciones pueden ser de primer orden, es decir, devolver otras funciones como valores.
// Las funciones pueden ser puras, es decir, no tener efectos secundarios.
// Las funciones pueden ser parciales, es decir, no devolver un valor en todos los casos.
// Las funciones pueden ser totales, es decir, devolver un valor en todos los casos.
//
// Argumentos de las funciones: por valor, por referencia, por valor mutable, por referencia mutable, por nombre, por posición.
// Devolución de valores de las funciones: con return, con expresión, con tuplas, con opciones, con resultados, con errores.
// Argumentos por valor: se pasan valores a las funciones, pero no se pueden modificar.
// Argumentos por referencia: se pasan referencias a los valores a las funciones, y se pueden modificar.
// Argumentos por valor mutable: se pasan valores a las funciones, y se pueden modificar.
// Argumentos por referencia mutable: se pasan referencias a los valores a las funciones, y se pueden modificar.
// Argumentos por nombre: se pasan valores a las funciones, y se pueden modificar.
// Argumentos por posición: se pasan valores a las funciones, y se pueden modificar.

// 📌   FUNCIONES - Funciones simples 
// 
// Si la función devuelve un valor se debe poner una flecha y el tipo del valor de devolución 
// Para devolver un valor se puede usar return o simplemente una expresión sin punto y coma.
//
// fn suma(a: i32, b: i32) -> i32 {
//     a + b              // expresión sin punto y coma
// }
// fn ladrar() {
//     println!("Guau");  // función sin valor de retorno
// }
// fn main() {
//     let a: i32 = 13;
//     let b = 42;
//     let c= suma(a,b);
//     println!("resultado es: {}",c);
//    
//     ladrar();
// }

// 📌   FUNCIONES - Pasar argumentos por valor
//
// Por valor: Se crea una copia del argumento y se pasa como parámetros a la función, cualquier modificación dentro de la función 
// -no afecta al valor original.
//
// fn duplicar(x: i32) -> i32 {     // función duplicar que recibe un valor de tipo i32 y devuelve un valor de tipo i32
//     x * 2                        // devuelve el valor de "x" multiplicado por 2
// }
//
// fn main() {
//     let a = 42;
//     let b = duplicar(a);
//     println!("El doble de {} es {}", a, b);
// }

// 📌   FUNCIONES - Pasar argumentos por referencia
//
// Por referencia: Se pasa un argumento de referencia a los parámetros de la función. cualquier modificación dentro de la función
// -afecta al valor original.
//
// fn duplicar(x: &mut i32) {       // función duplicar que recibe una referencia mutable a un valor de tipo i32 
//     *x *= 2;                     // multiplica el valor de "x" por 2, pero antes se debe desreferenciar el valor de "x" con "*"
// }
//
// fn main() {
//     let mut a = 42;
//     duplicar(&mut a);
//     println!("El doble de 42 es {}", a);
// }
//      

// 📌   FUNCIONES - Pasar función como argumentos
// 
// Pasar funciones como argumentos es una característica muy poderosa que nos permite crear código
// -más flexible y reutilizable, esto se conoce como funciones de primer nivel
//  
// Funciones de primer orden: Permiten pasar funciones como argumentos a otras funciones, 
// -lo que permite reutilizar código.
//
// fn ladrar() {                           // función ladrar que imprime "Guau"
//     println!("Guau");
// }
// fn hacer_n_veces(f:fn(),n:i64) {        // función hacer_n_veces que recibe una función y un valor de tipo i64
//     for _ in 0..n {                     // bucle for que se ejecuta n veces
//       f();                              // llama a la función f
//     }                                   // fin del bucle for
// }
//
// fn main() {                             // función principal
//        hacer_n_veces(ladrar,2);         // imprime 2 veces resultado de la función ladrar
// }
// 
// LECTURAS DE INTERÉS:
// Abstracción: Permite crear funciones más genéricas que pueden trabajar con diferentes tipos de operaciones 
// -sin conocer los detalles internos de esas operaciones
// 
// Callbacks: Se utiliza comúnmente para implementar mecanismos de devolución de llamada, donde una función
// -se ejecuta cuando ocurre un determinado evento.
// 
// Algoritmos genéricos: Facilita la creación de algoritmos que pueden trabajar con diferentes tipos de datos
// -siempre y cuando estos datos admitan ciertas operaciones.
//
// ejemplo avanzado:.
// 
// fn apply<F>(x: i32, f: F) ->i32
// where
//     F: Fn(i32) -> i32, 
// {
//     f(x)
// }
//
// fn main() {
//     let add_one = |x| x + 1;
//     let double = |x| x * 2;
//
//     let result1 = apply(5, add_one);
//     let result2 = apply(3, double);
//
//     println!("result1 = {}", result1);
//     println!("result2 = {}", result2);
// }
  
// 📌   FUNCIONES - Devolución de varios valores a la vez 
//
// Como tal no admite devolver varios valores a la vez, pero es posible usar tuplas y simularlo.
// todo!(cambiar valores de las variables y ver como se comporta el programa y los errores que da)
// fn string_length_and_lines(txt: &String) -> (usize,usize) {s
// (txt.len(),txt.lines().count()) // función contar cantidad caracteres
// }
//
// fn main() {
//     // asignamos 3 lineas de tipo String a variable "ss"
//     let ss = String::from("Europe's Skies - Alexander Rybak\nSuper Strut - Deodato\nEl Cóndor Pasa - Uña Ramos"); 
//     // asignamos el valor de la función contar_cantidad_caracteres a variable longitud y lineas
//     let (length,lines) = string_length_and_lines(&ss); 
//     // salida por pantalla variables resultados                                                             
//     println!("La lista de canciones tiene una longitud de {} caracteres y {} líneas",length,lines); 
// } 

// 📌   BLOQUE FUNCIONAL -  Devolución de valores sin punto y coma 
//
// Usamos un bloque de código para devolver un valor a traves de na expresión  sin punto y coma 
// -de lo contrario devolvería nada "()"
//
// fn main() {
//     let mi_numero = {
//         let segundo_numero = 29;
//         segundo_numero + 13          // expresión sin punto y coma
//     };
//     println!("1 - Valor de a: {}", mi_numero);
//     println!("2 - Valor de a: {:?}", mi_numero); // otra forma de imprimir valor, con {:?} se imprime el valor de la variable
// }

// 📌   STACK Y HEAP - La Pila y La Memoria Dinámica
//
// La pila ("stack" en inglés), la memoria dinámica ("heap" en inglés) y los punteros son elementos muy importantes en Rust
// La pila y la memoria dinámica son dos tipos de almacenamiento de los datos de un programa durante su ejecución
// 
// Algunos tipos no tienen un tamaño conocido en tiempo de compilación, no pueden guardarse en la pila. ¿Qué se puede hacer? 
// En primer lugar, se pone la información en la memoria dinámica ya que esta puede contener datos de cualquier tamaño. 
// En segundo lugar, se guarda un puntero en la pila, el tamaño de los punteros es conocido. 
// Así, para recuperar un valor de una variable que está en la memoria dinámica (heap), el ordenador va primero a la pila 
// obtiene el puntero y lo sigue hasta la memoria dinámica para localizar el contenido.
//
// Podemos comparar un puntero como un atabla de contenidos de un libro, el puntero es la página donde se encuentra la información
// y la memoria dinámica es el libro en sí, la pila es la tabla de contenidos.
//
// El puntero que se ve en Rust se denominan "referencias" y se representan con "&", ej:. &variable
// Una referencia apunta a la memoria de otro valor, pero no es propietaria de él, no se apropia de él. 
// Una referencia supone que se tome prestado el valor, pero no se apropia de él.
//
// DIFERENCIAS entre la pila y la memoria dinámica:
//
// LA PILA (stack): 
// Es una estructura de datos que almacena variables en un orden determinado, se accede a ellas mediante un puntero, 
// es muy rápida, la memoria dinámica (heap) no es que sea lenta, pero no le gana a la pila.
// Rust necesita conocer el tamaño de las variables en la pila en tiempo de compilación para guardarlas en ella 
// -la memoria dinámica no.
//
// No es posible utilizar la pila (stack) siempre, ya que su tamaño es limitado, la memoria dinámica (heap) no tiene límites.
// Algunos tipos de datos, como los vectores, las cadenas y las estructuras, se almacenan en la memoria dinámica (heap).
// 
// LA MEMORIA DINÁMICA (heap): 
// Es una estructura de datos que almacena variables en un orden aleatorio, se accede a ellas
// -mediante un puntero.
//
// Rust no utiliza un RECOLECTOR DE BASURA para gestionar la memoria dinámica (heap), utiliza el 
// -sistema de "propietarios y préstamos, el conteo de referencias, y ciclo de vida de las variables" para gestionar 
// -la memoria dinámica (heap), evitando fugas de memoria.
//
// Rust se encarga de liberar la memoria dinámica (heap) cuando ya no se necesita, evitando fugas de memoria.
//
// Ventajas de no tener un recolector de basura tradicional: Mayor rendimiento, menos consumo de memoria, menos errores.
// Mayor seguridad, menos errores, menos problemas de rendimiento, menos problemas de escalabilidad.
// Mayor control, menos errores, menos problemas de rendimiento, menos problemas de escalabilidad.
// Su única desventaja es que el programador debe gestionar la memoria manualmente, mayor complejidad y más código 
// -pero Rust facilita esta tarea.

// 📌   LA PILA (stack)
//
// El stack (o pila) es una región de memoria que se utiliza para almacenar datos de forma temporal 
// -durante la ejecución de un programa.
// 
// Imagina una pila de platos: el último plato que pones es el primero que quitas. 
// En el stack, el último dato que se almacena es el primero que se recupera.
//
// La gestión de la memoria en el stack es automática y muy rápida, ya que solo se necesita mover el puntero de la pila.
// La memoria en el stack se libera automáticamente cuando la función que la utiliza termina.
// Almacenamiento de datos locales, cuando se llama a una función se crea un nuevo marco de pila que 
// -contiene espacio para los argumentos de la función,las variables locales y valores de retorno si los hay.
// Cuando una función llama a otra, la dirección de retorno y los argumentos se almacenan en la pila.
//
// Sigue el principio LIFO (Last In, First Out), el último elemento que se añade es el primero que se elimina.
// La gestión es automática, el compilador se encarga de liberar la memoria cuando ya no se necesita.
// La memoria se libera automáticamente cuando la función que la utiliza termina.
// 
// Desventaja del stack: tamaño limitado, no se puede utilizar siempre, ya que su tamaño es limitado y puede provocar 
// -desbordamientos de pila si se realizan demasiadas llamadas recursivas o se declaran demasiadas variables locales.
//
// NECESARIO: leer desbordamiento de pila (stack overflow) consecuencias y cómo evitarlo y Optimización de la pila.

// 📌   LA MEMORIA DINÁMICA (heap)
//
// El heap (o montón) es una región de memoria que se utiliza para almacenar datos de forma dinámica 
// -durante la ejecución de un programa.
// Almacenamiento de datos dinámicos, se utiliza para almacenar datos cuyo tamaño no se conoce en tiempo de compilación.
// Imagina un montón de platos: puedes quitar los platos en cualquier orden y no necesariamente 
// -el último plato que pones es el primero que quitas, como pasa con el stack.
//
// Se necesita un puntero para acceder a los datos en el heap, ya que no se conoce el tamaño de estos datos.
// En el heap, los datos se almacenan de forma aleatoria y no se garantiza el orden de recuperación.
// La gestión de la memoria en el heap es manual y más lenta, ya que se necesita buscar la dirección de memoria.
// La memoria en el heap se libera manualmente, cuando ya no se necesita se debe liberar la memoria manualmente.
//
// Ventajas del heap: tamaño ilimitado, puede almacenar cualquier cantidad de datos y cualquier tamaño de datos.
// Desventajas del heap: gestión manual, más lento que el stack, posibles fugas de memoria, posibles errores de segmentación.

// 📌   LA PILA Y LA MEMORIA DINÁMICA - PUNTERO = REFERENCIA (que apunta a la memoria de otro valor)
// La pila "stack" es una estructura de datos que almacena variables en un orden determinado, se accede a ellas mediante un puntero.
// La memoria dinámica "heap" es una estructura de datos que almacena variables en un orden aleatorio
// se accede a ellas mediante un puntero.
// El puntero que se ve en rust se denomina "referencia" y se representa con "&", ej:. &variable
// &variable1, es una referencia a la variable, no es el valor en sí, es una referencia a la dirección de memoria
// esto significa que variable1 sigue siendo la dueña del valor, solo lo ha prestado y entrega una referencia
//
// fn main() {
//     let pais = "España"; // variable "pais" con valor "España"
//     let ref_uno = &pais; // variable "ref_uno" con referencia a "pais"
//     let ref_dos = &pais; // variable "ref_dos" con referencia a "pais"
//     let ref_tres = &ref_dos; // variable "ref_tres" con referencia a "ref_dos"
//     println!("{}", ref_uno);
//     println!("{}", ref_dos);
//     println!("{}", ref_tres);
// }

// 📌   REFERENCIAS
// Como protege rust el acceso a zonas de memoria erróneas, no permite el acceso a zonas de memoria que no le pertenecen, un ejemplo.
//
// fn return_str() -> &'static str {
//     let pais = String::from("España");
//     let pais_ref = &pais;
//     pais_ref    // ⚠️ warning <- ERROR, no se puede devolver una referencia a un valor que se destruirá al salir de la función.
// }
//
// fn main() {
//     let pais = return_str();
//     println!("{}", pais);
// }

// 📌   REFERENCIAS MUTABLES
// Regla: no se puede usar una referencia mutable al mismo tiempo que una referencia inmutable
//
// fn main() {
//     let mut mi_numero = 8;
//     let num_ref = &mut mi_numero;
//     *num_ref += 10; // desreferenciamos con "*" el valor de "num_ref" y le sumamos 10
//     // "*" es lo opuesto a "&", "&" es una referencia, "*" es desreferenciar
//     println!("{}", mi_numero);
//     let num_modify = &mi_numero;
//     println!("{}", num_modify);
// }

// 📌   REFERENCIAS MUTABLES
// Se dispone de {p} para imprimir la dirección de memoria de una variable, ej:. "{:p}"
// fn main() {
//     let a = 42; // variable "a" con valor 42
//     let b = &a; // variable "b" con referencia a "a"
//     println!("Dirección de memoria de \"a\": {:p}", b); // imprimimos la dirección de memoria de "a"
//     println!("Valor de \"a\": {}", a); // imprimimos el valor de "a"
//     println!("Valor de \"b\": {}", b); // imprimimos el valor de "b"
// }

// 📌   PASO DE REFERENCIAS A FUNCIONES
// Regla de Rust para todas los valores, "un valor solo puede tener un dueño a la vez".
//
// fn print_pais(pais_nombre: String) {
//     println!("{}", pais_nombre);
// }
//
// fn main() {
//     let pais = String::from("MÁS SOBRE España"); // se crea la variable "pais" con valor "España"
//     print_pais(pais); // se llama a la función "print_pais" con la variable "pais"
//     print_pais(pais); // ⚠️  ERROR, no se puede usar una variable que ya no es dueña del valor.
// Al pasar la variable "pais" a la función "print_pais" se transfiere la propiedad del valor.
// a la función, por lo que la variable "pais" ya no es dueña del valor.
// }
// Es mejor evitar que la función se apropie del valor, para ello se pueden pasar referencias (prestamos los valores) a la función, ej:. "&String" 
//
// fn print_pais(pais_nombre: &String) {
//     println!("{}", pais_nombre);
// }
//
// fn main() {
//     let pais = String::from("España"); // se crea la variable "pais" con valor "España"
//     print_pais(&pais); // se llama a la función "print_pais" con la variable "pais"
//     print_pais(&pais); // 😀 ahora si funciona, se puede usar la variable "pais" en varias funciones
//     println!("{}", pais); // comprobamos que la variable "pais" sigue siendo dueña del valor.  
// } 

// 📌   TIPOS COLECCIÓN
// Rust tiene varios tipos de colecciones, como vectores, arrays, tuplas, etc.
// Sirven para guardar más de un valor en un mismo lugar.
// Empezamos con los arrays, que son colecciones de longitud fija de elementos de datos del mismo tipo y los más simples y rápidos.

// 📌   ARRAYS - array es una colección de longitud fija de elementos de datos del mismo tipo.
//  El tipo de datos para un array es [T;N] siendo T el tipo del elemento, y N la longitud fija 
//  conocida en tiempo de compilación. Los elementos individuales se pueden recuperar con el 
//  operador [x], siendo x un índice de tipo usize (empezando por 0) del elemento que quieras. 
// Los arrays no pueden cambiar el tamaño y sus datos tienen que ser del mismo tipo,son muy rápidos y eficientes.
 
// 📌   ARRAYS - se puede obtener una sección (slice) de un array utilizando una referencia "&" y 
// después utilizando ".." para mostrar el rango
// Los indices empiezan en 0, por lo que el primer elemento es el 0, el segundo el 1, etc.
// Los rangos son inclusivos en el primer número y exclusivos en el segundo, por lo que [2..5] obtiene los elementos 2, 3 y 4.
// Para que se incluya el último número se puede usar de esta forma [0..=10].
// Para que se incluya el último número, se puede usar [1..] y para que se incluya el primero, se puede usar [..5].
//
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

// 📌   ARRAYS
// fn main() {
//     let numeros: [i32;3] = [1,2,3];
//     println!("Array {:?}", numeros);
// }
//
// Slices - &[T] para referenciar “vistas” en secuencias de valores en la memoria.
// Un string siempre ocupa 24 bytes en la pila, independientemente de su longitud, es de tamaño fijo.

// 📌   ARRAYS
// fn main() {
//     let mut notas_array: [u32; 5] = [0; 5];  // Array con tamaño fijo de 5 elementos e inicializadas las 5 posiciones con ceros
//     let meses = ["Enero, febrero, marzo, abril, mayo, Junio, Julio, Agosto, Septiembre, Octubre, Noviembre, Diciembre"]; // Array de tipo String
//     notas_array[0] = 1;
//     notas_array[1] = 6;
//     println!("7 - Nota 1 = {}\n    Nota 2 = {}\n  Todas = {:?}" , notas_array[0], notas_array[1], notas_array);
//     println!("7 - {:?}", meses);
// }

// 📌   VECTORES 
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

// 📌   VECTORES
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

// 📌   VECTORES
// fn main() {
//     let mut mi_vector3: Vec<String> = Vec::new(); // El compilador ya sabe que el vector contiene
//                                                   // elementos de tipo String.
//     mi_vector3.push(String::from("Agua")); // inserta un elemento de tipo String.
//     mi_vector3.push("Café".to_string()); // inserta elemento de tipo String con método to_string().
//     println!("{:?}", mi_vector3); // imprime los elementos del vector Agua y Café.
// }

// 📌   VECTORES
// otra forma de declarar un vector con la macro "vec!"
// fn main() { 
//     let mut mi_vector4 = vec![1,2,3,4,5]; // Vector de 5 elementos de tipo i32.
//     mi_vector4.push(6); // inserta un elemento de tipo i32.
//     mi_vector4.push(7); // inserta un elemento de tipo i32.
//     println!("{:?}", mi_vector4); // imprime los elementos del vector.
// }

// 📌   VECTORES
// Se pueden obtener secciones de un vector igual que en los arrays, con el método "slice".
// fn main() {
//     let mut mi_vector5 = vec![1,2,3,4,5]; // Vector de 5 elementos de tipo i32.
//     let slice = &mi_vector5[1..3]; // obtenemos los elementos 1 y 2.
//     println!("{:?}", slice); // imprime los elementos del slice.

// 📌   VECTORES 
// fn main () {
//         let mut notas_vec: Vec<i32> = vec!(); // Vector dinámico (No fijo) vació, integer 32 bits con signo 
//         notas_vec.push(1); // escribimos un valor "1" en la posicion 0
//         notas_vec.push(6); // escribimos un segundo valor "6" en la posicion 1.
//         println!("Nota 1 = {}\n    Nota 2 = {}\n", notas_vec[0], notas_vec[1]);
// }

// 📌   VECTORES 
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

// 📌   TUPLAS Y DESESTRUCTURACIÓN, son como una estructura sin nombre de campos, una especie de array donde 
// cada elemento puede ser de un tipo diferente pero especificado de antemano
// Las tuplas son un tipo de dato que permite agrupar varios valores en un solo valor compuesto.
// Podemos agrupar valores de diferentes tipos en una tupla, pero una vez definida, no se pueden añadir o quitar elementos.
// Podemos desestructurar una tupla para obtener los valores individuales, podemos acceder a los valores 
// de una tupla mediante el operador "." seguido del índice del valor.
//
// fn main () {
    // let tupla = (23,"Javier", true);       // Formamos la tupla directamente con valores de tipo (integer, texto, boleano)    
    // let (random, z_name, has_beers) = tupla; // Desestructuramos la tupla y obtenemos 3 variables
    // // Imprimimos las variables obtenidas
    // println!("\n{}", random);
    // println!("{}", z_name);
    // println!("{}\n", has_beers);
// } convertido a u32: {}", c);
    // let t = true;
    // println!("{}", t as u8);
// }

// 📌   TUPLAS
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
  
// 📌   ENUM - Enumeraciones
// Las enumeraciones son un tipo de dato que permite definir un tipo de dato con un número limitado de valores posibles.
// Se pueden definir enumeraciones con la palabra reservada "enum" seguida del nombre de la enumeración y entre llaves los valores posibles separados por comas. (la coma en el último valor es opcional pero importante si solemos cambiar de sitio los valores).
// Diferencia entre "struct" y "enum", las estructuras permiten definir un tipo de dato con varios campos (Representar una cosa Y otra cosa a la vez), mientras que las enumeraciones permiten definir un tipo de dato con un número limitado de valores posibles, (Representar una cosa O alguna cosa diferente).
// Los ENUM son perfectos para representar un conjunto de valores relacionados, como los estados de un programa o los tipos de errores.

// 📌   ENUM - ejemplo de enum
// Según la hora del día, el cielo puede tener el sol o las estrellas, creamos una enumeración con dos valores posibles.
// Creamos una función que "devuelve el estado del cielo" según la hora del día, la cual se la pasamos en el argumento de la función, la llamamos "tiempo" y a traves de match devolvemos el valor del enum. que corresponda según el rango de horas.
// Para llamar a un valor del enum se usa "::" después del nombre del enum y el valor.
// match es una expresión que permite comparar un valor con una serie de patrones y ejecutar un bloque de código dependiendo de qué patrón coincida, en este caso, el valor de la variable "tiempo" con los valores de la enumeración.
// la flecha después del patron de rango indica que se devuelva el valor del enum que corresponda.

// 📌   ENUM
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

// 📌   BUCLES - Podemos crear un bucle con loop hacer que continue hasta que queramos que se detenga con "break", por ejemplo comparando un valor
// fn main() {
//     loop {
//         break; // Este bucle se ejecuta indefinidamente si no se le pone un "break".
//     }
// } 

// 📌   BUCLES
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

// 📌   BUCLES ANIDADOS
//
// Si se inserta un bucle dentro de otro es posible darles nombre para indicar a Rust a qué bucle 
// salir cuando se ejecuta una sentencia break. 
//
// fn main() {
//     let mut contador = 0; // inicializamos la variable "contador" con valor 0.
//     let mut contador2 = 0; // inicializamos la variable "contador2" con valor 0.
//     println!("Entro en el primer bucle");
//     'primer_bucle: loop {
//         // Damos nombre de esta forma 'nombre_bucle: loop.
//         contador += 1; // incrementamos el valor de "contador" en 1.
//         println!("El contador es ahora: {}", contador);
//  Si se inserta un bucle dentro de otro es posible darles nombre para indicar a Rust 
// -a qué bucle salir cuando se ejecuta una sentencia break.               }
//             }         
//         } 
//     } 
// }  

// 📌   BUCLES - While
// Este bucle se ejecuta mientras la condición sea verdadera " TRUE", si la condición es falsa "FALSE" no se ejecuta.
// fn main() {
//     let mut contador = 0; // inicializamos la variable "contador" con valor 0.
//     while contador < 10 {
//         contador += 1; // incrementamos el valor de "contador" en 1.
//         println!("Contador: {}", contador); // imprimimos el valor de "contador".
//     }
// }

// 📌   BUCLES - for
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

// 📌   BUCLES - loop con break y devolución de valor.
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

// 📌   BUCLES - for (Comprobación de colores con un bucle)
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

// 📌   BUCLES -
// fn main() {
//     let first = (200, 20,0);
//     let second = (50, 50, 50);
//     let third = (200, 50, 0);
//     match_colores(first);
//     match_colores(second);
//     match_colores(third);
// }

// 📌   IMPLEMENTACIÓN - Implementaciones de funciones asociadas a los tipos definidos como tal.
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

// 📌   impl - ejemplo de imnplementación de un struct un enum.
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

// 📌   impl - ejemplo de imnplementación de un enum.
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

// 📌   IMPLEMENTACIÓN
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

// 📌   DESESTRUCTURAR - los valores de un struct
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

// 📌   DESESTRUCTURAR
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

// 📌   REFERENCIAS Y EL OPERADOR PUNTO.
// Las referencias son una forma de referenciar un valor sin tener que moverlo, se crean con "&" seguido del nombre de la variable.
// Las referencias no pueden ser mutables y no se pueden modificar.
// Para modificar una referencia se usa "&mut" seguido del nombre de la variable.
// El operador punto "." se usa para acceder a los campos de una estructura o enum a través de una referencia.
// Para acceder al valor de una referencia se usa "*" seguido del nombre de la referencia.
// fn main() {
//     let mi_numero = 9;           // Creamos una variable "mi_numero" con valor 9.
//     let referencia = &mi_numero; // Creamos una referencia a "mi_numero".
//
//     println!("{}", mi_numero == referencia); // ⚠️ Comparamos "mi_numero" con el valor de la referencia, pero da error, hay que utilizar "*" en la referencia.
// }
// fn main() {
//     let mi_numero = 9;           // Creamos una variable "mi_numero" con valor 9.
//     let referencia = &mi_numero; // Creamos una referencia a "mi_numero".
//
//     println!("{}", mi_numero == *referencia); // 🎉 Comparamos "mi_numero" con el valor de la referencia, con el asterisco en la referencia si funciona.
// }

// 📌   REFERENCIAS Y EL OPERADOR PUNTO. 
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

// 📌   REFERENCIAS Y EL OPERADOR PUNTO. 
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

// 📌   GENÉRICOS
// En las funciones genéricas se puede usar cualquier tipo de dato, se definen con "<T>" después del nombre de la función.
// Se puede usar cualquier letra o palabra después de "<" y antes de ">" para definir un genérico, normalmente se usa "T" para "Type" y "E" para "Element" y "K" para "Key"n y "V" para "Value" y "S" para "String"y "I" para "Integer".
// Se puede usar un genérico en el nombre de la función, en los argumentos y en el tipo de retorno.

// 📌   GENÉRICOS
// En las funciones es necesario especificar el tipo de dato de cada parámetro de entrada y de salida.
// fn devuelve_numero(numero: i32) -> i32 {
//     println!("Ahí va tu numero.");
//      numero
// }
// fn main() {
//     let nuevo_numero = devuelve_numero(8);
//     println!("El número es: {}", nuevo_numero);
// }

// 📌   GENÉRICOS   
// Los generics data types son una forma de definir una función que puede aceptar cualquier tipo de dato y evita duplicar código.
// fn devuelve_numero<T>(numero: T) -> T {
//     println!("Ahí va tu numero.");
//     numero
// }
// fn main() {
//     let nuevo_numero = devuelve_numero(8);
//     println!("El número es: {}", nuevo_numero);
// }

// 📌   GENÉRICOS
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

// 📌   GENERICOS
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

// 📌   CONSTANTES Y STATIC
// Las variables estáticas son como las constantes pero pueden ser mutables, se declaran con "static" y en mayúsculas, 
// son como variables globales, no se eliminan cuando termina el bloque en el que se declaran o la función, permanecen 
// en memoria hasta que el programa finaliza, su alcance es global se pueden acceder desde cualquier parte del programa, 
// el timepo de vida es el mismo que el del programa, son inmutables por defecto, lo que significa que su valor no se puede 
// modificar después de su inicialización. Sin embargo, se pueden declarar como mutables utilizando la palabra clave mut.
// ejemplo: "static mut VARIABLE_ESTATICA_MUTABLE: tipo_de_dato = valor_inicial;"
// Por convención, las constantes se suelen escribir con todas las letras en mayúsculas, normalmente están fuera del main 
// para que existan en todo el programas.
//
// fn main () {
//     const PI:f64 = 3.14159; // las constantes no cambian nunca su valor, se declaran con "const" y en mayúsculas
//     static ESTACIONES: [&str; 4] = ["Primavera", "Verano", "Otoño", "Invierno"]; // las variables estáticas pueden ser mutables, se declaran con "static" y en mayúsculas, son como variables globales.
//     println!("Vamos de paseo, {} {} {}", PI, PI, PI);
//     println!("Las estaciones son: {:?}\n", ESTACIONES);
// }

// 📌   CONSTANTES y casting ("as")
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
//        let b: u32 = 7; // tipo de dato "u32" sin signo de 32 bits
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
   
// 📌   EXPRESIONES AVANZADAS con variables "let", expresión condicional con "if, else", 
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

// 📌   EXPRESIONES AVANZADAS con variables "let", una operación de "a*b"
// fn main () {
//     let u = 2;
//     let _xa = u+age;                // el valor de "u" se suma al de "age"
//     println!("14 - Valor de x: {}",_xa);        // imprime la expresión avanzada de "x"
// }
    
// 📌   Rust no deja la memoria al descubierto ni usa GC. Para ello el compilador realiza
// una tarea de dueños y préstamos que veremos a continuación
// Las REGLAS -> Cada valor en Rust tiene una variable que es su dueñ
//            -> Un valor solo puede tener un dueño a la ve
//            -> Cuando el dueño desaparece, el valor lo hace a su vez, de forma automátic

// 📌   La trait COPY y CLONE
//    
// let s1 = String::from("Adios - Xavier Cugat");
// let s2 = s1.clone();
// println!("15 - Hemos clonado \"s2\" desde \"s1\" ahora las dos variable tienen el mismo valor\n     y cada una es propietaria de si misma, -> valor =  {}",s2);

// 📌   IMPLICACIONES - Pasar una variable tal cual a una función si no es del tipo Copy 
// 
// implica que perdemos el acceso a ese valor!
// fn main() {
//   
// let s1 =  String::from("Bolero - Maurice Ravel");
// let s2 = s1.clone();
// f(s2);
// Este código daría error si al hacer la llamada a la función f hemos transferido la propiedad 
// del valor de s1 a f. Por ello, cuando intentamos hacer el print no vamos a poder ya que 
// s1 ya no es dueña de la cadena de texto. 
// Para solucionar estos problemas tenemos los préstamos, tal y como ha quedado el código.
// }

// 📌   PRESTAMOS (Prestando en Rust) 2 maneras: solo lectura o con escritura
// NORMA: solo una con permisos de escritura pero infinidad con permiso de lectura, nunca las dos a la vez. 
// El préstamo se realiza con el operador "&" que es una "referencia" de lectura al valor
// La variable sigue siendo la dueña del valor, solo lo ha prestado y entrega una referencia
// fn main() {
//     let mut x = 5;
//
//     let r1 = &x; // Préstamo inmutable
//     println!("Valor de x: {}", r1); // Imprime "Valor de x: 5"
//
//     let r2 = &mut x; // Préstamo mutable
//     *r2 += 1; // Modifica el valor de x a 6
//
//     println!("Valor de x después de la modificación: {}", r2); // Imprime "Valor de x después de la modificación: 
// }
  
// 📌   PRESTAMOS (Prestando en Rust) prestasmo en modo escritura, debemos utilizar "&mut"    
//
// fn f(s: &mut String) {
//     s.push_str(" & Adios - Xavier Cugat");
// }
// //
// fn main() {
// let mut s1 = String::from("Bolero - Maurice Ravel");
// f(&mut s1);
// println!("{}",s1);
// }

// 📌   TRAITS - (rasgos)
// Los traits son una forma de definir un comportamiento que un tipo de dato puede tener.
// Se definen con la palabra clave "trait" seguida del nombre del trait y las funciones que define.
// Se pueden implementar traits para tipos de datos definidos por el usuario, como "struct" y "enum".
// ejemplo de traits -> "Display" para imprimir un tipo de dato, "Debug" para imprimir un tipo de dato con "{:?}"
// Copy y Clone también son traits, Copy se usa para tipos de datos que se copian en lugar de moverse
//  y Clone se usa para hacer copias de un tipo de dato.
// Para que un tipo tenga un trait, se debe implementar el trait con la palabra clave "impl" seguida del 
// nombre del trait y las funciones que define.
//
// Hay atributos que se pueden añadir a los traits, como "derive" para implementar traits automáticamente.
// Esto es lo que sucede con "Debug" y "Display", que se pueden implementar automáticamente con "derive".
// ejemplo: #[derive(Debug)] #[derive(Display)]
//
// #[derive(Debug)]
// struct Punto {
//     numero: usize,
// }
// fn main() {}
//
// Hay  otros traits que son más difíciles de implementar y hay que hacerlo manualmente con "impl"
// Ejemplo de implementación de un trait para un tipo de dato definido por el usuario.
//
// struct Animal {
//     nombre: String,
// }
// //
// trait Perro {
//     fn ladrar(&self) {
//         println!("¡Guau, guau!");
//     }
//     fn correr(&self) {
//         println!("¡Corre, corre!");
//     }
// }
// //
// impl Perro for Animal {  // Ahora el Animal implementa el trait Perro
//   fn correr(&self) {
//       println!("¡{} está corriendo!", self.nombre);
//   } 
// }
//
// //
// fn main() {
//     let toto = Animal {
//        nombre: "Toto".to_string(),
//     };
//     toto.ladrar();   // El Animal toto ahora puede ladrar
//     toto.correr();   // El Animal toto ahora puede correr
// }

// 📌   ENCADENAR MÉTODOS
// Se pueden encadenar métodos en Rust, es decir, llamar a varios métodos seguidos en la misma línea.
// Para encadenar métodos, se llama a un método después de otro, separados por un punto ".".
// Se pueden encadenar tantos métodos como se quiera, siempre que el método devuelva un valor que 
// se pueda usar en el siguiente método.
// fn main() {
//     let mut s = String::from("Hola, "); // Creamos un String con el valor "Hola, ".
//     s.push_str("mundo!"); // Añadimos "mundo!" al final del String.
//     println!("{}", s); // Imprimimos el String.
// }

// 📌   ENCADENAR MÉTODOS
// fn main() {
//     let s = String::from("Hola, ").push_str("mundo!"); // Creamos un String con el valor "Hola, " y añadimos "mundo!" al final.
//     println!("{}", s); // Imprimimos el String.
// }

// 📌   ENCADENAR MÉTODOS
// fn main() {
//     let s = String::from("Hola, ").push_str("mundo!").to_uppercase(); // Creamos un String con el valor "Hola, " 
//                                                                       // y añadimos "mundo!" al final y lo convertimos a mayúsculas.
//    println!("{}", s); // Imprimimos el String.
// }

// 📌   ENCADENAR MÉTODOS
// Con el estilo funcional de programación se pueden encadenar métodos. Encadenar métodos significa que 
// se pueden unir para formar una única sentencia. A continuación se muestra un ejemplo de muchos métodos unidos:
// fn main() {
//     let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//
//     println!("{:?}", new_vec);
// }

// 📌   ENCADENAR MÉTODOS
// Si se separan en diferentes líneas para hacerlo más fácil de leer (y se añaden comentarios explicativos):
// fn main() {
//     let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//
//     let new_vec = my_vec
//         .into_iter() // Itera a través de todos los elementos.
//                      // Esta función devuelve los elementos, no referencias a ellos.
//         .skip(3) // se salta tres elementos, en este caso: 0, 1 y 2
//         .take(4) // obtiene los cuatro primeros que quedan: 3, 4, 5 y 6
//         .collect::<Vec<i32>>(); // los coloca en un nuevo Vec<i32>
//
//     println!("{:?}", new_vec);
// }

// 📌   ITERADORES 
// Los iteradores son una forma de recorrer una colección de elementos en Rust.
// Se pueden crear iteradores con el método "iter" de un vector, que devuelve un iterador
// que recorre los elementos del vector.
// Se pueden recorrer los elementos de un iterador con un bucle "for" o con el método "for_each".
//
// .iter() -> para iterar a través de referencias a los elementos.
// .iter_mut() -> para iterar mediante referencias modificables (mutables).
// .into_iter() -> para obtener un iterador sobre valores, no referencias.
// .map() -> para transformar los elementos de un iterador.
// .collect() -> para recopilar los elementos transformados en un nuevo contenedor.
// .for_each() -> para ejecutar una función en cada elemento de un iterador.
//  
// fn main() {
//     let vec = vec![1, 2, 3, 4, 5]; // Creamos un vector con los valores 1, 2, 3, 4 y 5.
//     for i in vec.iter() { // Recorremos los elementos del vector con un bucle "for".
//         println!("{}", i); // Imprimimos cada elemento.
//     }
// }

// 📌   ITERADORES
// Iteradores con uso de iter(), map(), collect(), into_iter(), map(), collect(), iter_mut(), for_each()
//
// https://www.jmgaguilera.com/rust_facil/33.html -> Iteradores en Rust
//    fn main() {
//     let vector1 = vec![1, 2, 3]; // Se usará .iter() y .into_iter() sobre este vector
//     let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
//     let vector1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>();
//
//     let mut vector2 = vec![10, 20, 30]; // Se usará .iter_mut() sobre este otro vector
//     vector2.iter_mut().for_each(|x| *x +=100);
//
//     println!("{:?}", vector1_a);
//     println!("{:?}", vector2);
//     println!("{:?}", vector1_b);
// }

// 📌   ITERADORES - ejemplo: Biblioteca de libros
//
// #[derive(Debug)] // Se quiere usar print con {:?}
// struct Library {
//     library_type: LibraryType, // Este es el enum
//     books: Vec<String>, // lista de libros
// }
//
// #[derive(Debug)]
// enum LibraryType { // las bibliotecas pueden ser de la ciudad o del país
//     City,
//     Country,
// }
//
// impl Library {
//     fn add_book(&mut self, book: &str) { // Se usa add_book para añadir nuevos libros
//         self.books.push(book.to_string()); // Se toma un &str y lo convierte en String, para añadirlo después a Vec
//     }
//
//     fn new() -> Self { // Esta función crea una nueva biblioteca
//         Self {
//             library_type: LibraryType::City, // La mayoría son de ciudades, por lo qu ese usa como valor por defecto
//                                              // la mayor parte de las veces.
//             books: Vec::new(),
//         }
//     }
// }
//
// fn main() {
//     let mut my_library = Library::new(); // crea una nueva biblioteca
//     my_library.add_book("The Doom of the Darksword"); // se añaden algunos libros
//     my_library.add_book("Demian - die Geschichte einer Jugend");
//     my_library.add_book("구운몽");
//     my_library.add_book("吾輩は猫である");
// 
//     println!("{:?}", my_library.books); // se puede imprimir la lista de libros
//
// }

// 📌   CLOUSURES 
//
// Los cierres o closures (en inglés) son una especie de funciones rápidas que no necesitan un nombre. 
// En ocasiones se les denomina funciones lambdas, pueden tener argumentos y devolver un valor.
// Son fáciles de encontrar en el código debido a que utilizan || en lugar de ().
//
// fn main() {
//     let my_closure = || println!("Esto es un cierre"); // Se define un cierre que imprime un mensaje.
//     my_closure();                                      // Se llama al cierre.
// }
//
// Las clausuras son funciones anónimas que se pueden almacenar en variables y pasar como argumentos a otras funciones.
// Se definen con la palabra clave "move" seguida de una clausura entre llaves "{}".
// Las clausuras pueden capturar variables del entorno en el que se definen.
// Entre los símbolos || se pueden añadir variables de entrada y tipos
// ejemplo:.
// fn main() {
//     let my_closure = |x: i32| println!("{}", x);
//
//     my_closure(5);
//     my_closure(5+5);
// }

// 📌   CLOUSURES BIBLIOTECA
// https://www.jmgaguilera.com/rust_facil/34.html -> Closures "Cierres" en Rust
//
// Pero los cierres son especiales porque pueden guardarse valores de variables que se encuentren 
// fuera del ellos incluso aunque no reciban parámetros
// Una || que no encierra ninguna variable exterior en su interior es una función anónima. 
// No es, estrictamente, un closure.
// Una || que sí encierra una variable exterior en su interior sí es un cierre.
//
// fn main() {
//     let number_one = 6;
//     let number_two = 10;
//
//     let my_closure = || println!("{}", number_one + number_two);
//     my_closure();
// }

// 📌   CLOUSURES - Métodos utiles para sus usos con "cierres" o "clousures"
//
// fn main() {
//     let meses = vec![
//           "Enero",
//            "Febrero", 
//            "Marzo", 
//            "Abril", 
//            "Mayo", 
//            "Junio", 
//            "Julio", 
//            "Agosto", 
//            "Septiembre", 
//            "Octubre", 
//            "Noviembre", 
//            "Diciembre"
//      ];
//
//     let filtered_meses = meses
//         .into_iter()                         // crea un iterador
//         .filter(|month| month.len() <= 5)     // Solo los meses con cinco o menos caracters (byte)
//                                              // En este caso, todos los caracteres son de un byte, por eso funciona usar .len()
//         .filter(|month| month.contains("u")) // Se seleccionan solo los meses que contengan la letra u
//         .collect::<Vec<&str>>();
//
//     println!("{:?}", filtered_meses);
// }

// 📌   MACRO dbg!
//
// La macro dbg! es una macro de depuración que imprime el valor de una expresión con información adicional.
// alternativa a println! para depurar, porque es más fácil de teclear y da más información.
// se puede poner en cualquier parte del código y se puede usar en cualquier tipo de dato.
//  fn main() {
//     let mut my_number = dbg!(9);    // Se crea una variable "my_number" con el valor 9 y se imprime con dbg!.
//     dbg!(my_number += 10);               // Se incrementa el valor de "my_number" en 10 y se imprime con dbg!.
//
//     let new_vec = dbg!(vec![8, 9, 10]); // Se crea un vector con los valores 8, 9 y 10 y se imprime con dbg!.
//
//     let double_vec = dbg!(new_vec.iter().map(|x| x * 2).collect::<Vec<i32>>()); // Se duplican los valores del
// vector iterando sobre ellos y mapeándo cada valor multiplicado por 2 y se recolecta o colecciona en un nuevo 
// vector que se imprime con dbg!.
//
//     dbg!(double_vec);   // Se imprime el nuevo vector con dbg!.
// }

// 📌   MÉTODO .inspect()
// 
// Como .inspect() recibe de parámetro un cierre (closure), se puede codificar todo lo que se necesite:
//
// fn main() {
//     let new_vec = vec![8, 9, 10];               // Se crea un vector con los valores 8, 9 y 10.
//
//     let _double_vec = new_vec          // Se duplican los valores del vector
//         .iter()                                 // Se crea un iterador para recorrer los elementos del vector.         
//         .inspect(|first_item| {                   // &&i32 es un puntero mutable a un entero de 32 bits
//             println!("El elemento es: {}", first_item);  // Se imprime el primer elemento del vector.
//             match **first_item % 2 {                     // Se comprueba si el primer elemento es par o impar.
//                 0 => println!("Es par."),                // Si es par, se imprime "Es par."
//                 _ => println!("Es impar."),              // Si es impar, se imprime "Es impar."
//             }
//             println!("En binario es {:b}.", first_item); // Se imprime el primer elemento en binario.
//         })  
//         .map(|x| x * 2)  // 
//         .collect::<Vec<i32>>();                         // Se recolectan los valores en un nuevo vector.
// }

// 📌   MÉTODO .parse() - Convertir String a tipo numérico 
//
// .parse() es un método que convierte un String en un tipo numérico, pero hay 
// que tener en cuenta que el tipo de dato debe ser el mismo que el tipo de dato del String.
//
// .expect() es un método que se usa para manejar errores, si el método .parse() falla,
// se puede usar .expect() para mostrar un mensaje de error.
//
// fn main() {
//     let numero = "42"; // Creamos un String con el valor "42".
//     let numero: i32 = numero.parse().expect("No es un número");    // Convertimos el String en un i32.
//     println!("El número es: {}", numero); // Imprimimos el número.
// }

// 📌   MÉTODO .parse() - Convertir String a tipo numérico - utilizamos Match  
// 
// .parse() es un método que convierte un String en un tipo numérico, pero hay 
// que tener en cuenta que el tipo de dato debe ser el mismo que el tipo de dato del String.
//
// Comprobamos con match si la conversión es correcta. Si es correcta, devolvemos el número. 
//
// fn main() {
//     let numero = "42"; // Creamos un String con el valor "42".
//     let numero: i32 = match numero.parse() {                   // Convertimos el String en un i32.
//         Ok(numero) => numero,                                  // Si la conversión es correcta, devolvemos el número.
//         Err(_) => 0, // Si hay un error, devolvemos 0.         // Si hay un error, devolvemos 0.
//     };
//     println!("El número es: {}", numero); // Imprimimos el número.
// }

// 📌   MÉTODO .parse() - Convertir String a tipo numérico - Utilizamos Result 
//
// CONVERTIR String a tipo numérico con  .parse() y un Result con "std::num::ParseIntError"
//
// fn main() {
//     let numero = "42";                                                 // Creamos un String con el valor "42".
//     let numero: Result<i32, std::num::ParseIntError> = numero.parse(); // Convertimos el String en un i32.
//     match numero {                                                     // Comprobamos si la conversión es correcta.
//         Ok(numero) => println!("El número es: {}", numero), // Si la conversión es correcta, imprimimos el número.
//         Err(_) => println!("No es un número"), // Si hay un error, imprimimos un mensaje de error.
//     }
// } let numero: i32 = numero.parse().expect("No es un número");    // Convertimos el String en un i32.
//     println!("El número es: {}", numero); // Imprimimos el número.
// }

// 📌   MÉTODO .parse() - Convertir String a tipo numérico a un vector - utilizamos Result y ?
// 
// Convertir String a tipo numérico con  .parse() y un Result con "std::num::ParseIntError" 
// Aplicado a un vector de Strings y utilizando "?" para manejar errores.
//
// fn parse_str(input: &str) -> Result<i32, std::num::ParseIntError> { // Función que convierte un String en un i32.
//     let parsed_number = input.parse::<i32>()?;                      // Convertimos el String en un i32.
//     Ok(parsed_number)                                               // Devolvemos el número.
// }
//
// fn main() {
//     let str_vec = vec!["Siete", "8", "9.0", "bien", "6060"];         // Creamos un vector con varios Strings.
//     for item in str_vec {                                            // Recorremos el vector.
//         let parsed = parse_str(item);                                // Convertimos el String en un i32.
//         println!("{:?}", parsed);                                    // Imprimimos el resultado.
//     }
// }
 
// 📌   MÉTODO .chars() - convertir una cadena de caracteres (string) en iterador
// de caracteres individuales
// 
// Al llamar a chars() sobre una cadena, se obtiene un iterador de tipo Chars.
// que permite recorrer una secuencia de valores de forma eficiente.
// Cada elemento del iterador Chars es un char, que representa un carácter Unicode.
//
// Posibilidades: Convertir a Vector, a mayusculas o minuscula, contar, buscar, modificar
// -verificar si es digito, etc.
// 
// // Imprimir cada carácter en linea nueva
// fn main() {
//     let cadena = "Hola, mundo!";
//     for caracter in cadena.chars() {
//         println!("{}", caracter);
//     }
// }
//
// Contar carácteres
// fn main() {
//     let cadena = "Hola, mundo!";
//     let num_caracteres = cadena.chars().count();
//     println!("La cadena tiene {} caracteres.", num_caracteres);
// }
//
// Búscar un carácter
// fn main() {
//     let cadena = "Hola, mundo!";
//     if cadena.chars().any(|c| c == 'a') {
//         println!("La cadena contiene la letra 'a'.");
//     }
// }
//
// Convertir a UN Vector
// fn main() {
//     let cadena = "Hola";
//     let caracteres: Vec<char> = cadena.chars().collect();
//     println!("{:?}", caracteres);
// }
//
// El iterador Chars consume la cadena original, por lo que no puedes modificar 
// -la cadena mientras iteras sobre ella.

// 📌   MÉTODO .rev() - Invertir el oredn de un iterador
//
// El método rev() invierte el orden de un iterador. Se puede usar con cualquier iterador.
// Por ejemplo para invertir el orden de los elementos de un vector, una palabra o una cadena.
//
// ejemplo para un invertir un vector de tipo char
//
// fn main() {
//     let numeros: Vec<char> = vec!['h', 'o', 'l', 'a'];    
//     let numeros_invertidos: Vec<char> = numeros.iter().rev().cloned().collect();
//     println!("{:?}", numeros_invertidos); // Imprime [5, 4, 3, 2, 1]
// }
//
// otro ejemplo para invertir un vector de tipo String
//
// pub fn reverse(input: Vec<String>) -> String {
//     let concatenated: String = input.concat();
//     concatenated.chars().rev().collect()
// }
// fn main() {
//     let mi_vector: Vec<String> = vec!["Hola".to_string(), "Mundo".to_string()];
//     let salida: String = reverse(mi_vector);
//     println!("{}", salida);
// }
//

// Para invertir cadenas: Al convertir una cadena en un iterador de caracteres, puedes invertir 
// -el orden de los caracteres.
// Invertir cualquier secuencia: rev() puede ser aplicado a cualquier tipo de iterador, incluyendo 
// -iteradores creados a partir de conjuntos, mapas, y otros tipos de colecciones.
//
// ejemplo para invertir cadenas
//
// fn main() {
//     let cadena = "Hola, mundo!";
//     let cadena_invertida: String = cadena.chars().rev().collect();
//     println!("{}", cadena_invertida); // Imprime "!odnum ,aloH"
// }
//








// 📌   CICLOS DE VIDA - lifetime de String y &str
// 
// let mi_cadena = "Soy un &str", este tipo de &str dura toda la ejecución del programa ya que es literal y están escritas 
// -directamente en el código fuente y así se traspasa al código binario, el tipo real que tienen estas variables es &'static str.
// Las cadenas de caracteres literales tienen un tiempo de vida llamado 'static, que sirve para expresar 
// que la cadena existe durante toda la ejecución del programa.
// 
// El apostrofo ' indica el tiempo o ciclo de vida (lifetime) de este valor.
// 
//
//
// str prestados: Es la forma habitual del tipo &str. No tiene un tiempo de vida 'static, su tiempo de vida 
// es menor a la duración de todo el programa.
// 
// Por ejemplo, si se crea un String y se obtiene una referencia a ella, Rust la convertirá a &str si se necesita y
// su lifetime será el mismo que el del String.
//
// fn prints_str(my_str: &str) { // Esta función puede recibir &String y &str
// println!("{}", my_str);
// }
//
// fn main() {
//     let my_string = String::from("I am a string");
//     prints_str(&my_string); // Se pasa a prints_str u &String
// }
// En Rust, todas las referencias tienen un tiempo de vida asociado.

// 📌   CICLOS DE VIDA (lifetimes)
//
// Los tiempos de vida (lifetimes) son una característica de Rust que garantiza que las referencias
// no sobrevivan a los valores a los que hacen referencia.
// Los tiempos de vida se definen con el símbolo de apóstrofe ' seguido de un nombre.
// Los tiempos de vida se pueden añadir a las funciones con la sintaxis <'a> después del nombre de la función.
// Los tiempos de vida se pueden añadir a las referencias con la sintaxis &'a después del tipo de dato.
// Los tiempos de vida se pueden añadir a las estructuras con la sintaxis <'a> después del nombre de la estructura.
// Los tiempos de vida se pueden añadir a las implementaciones con la sintaxis <'a> después del nombre de la 
// estructura.
// 
// El tiempo de vida asociado a todos los valores y variables indica "cuánto vive una variable". 
// Solo es necesario pensar en ellos cuando se trabaja con referencias. 
// Esto se debe a que las referencias no pueden vivir más tiempo que el propio objeto al que referencian. 
// Por ejemplo, esta función no compila:
//  
// fn returns_reference() -> &str                     // no compila ⚠️
//     let my_string = String::from("I am a string");
//     &my_string // ⚠️
// }
//
// fn main() {}                                       // no compila ⚠️
// 
// El problema es que my_string solo vive dentro de la propia función, pero la función intenta devolver una
// referencia &my_string y esta no podrá existir cuando se libere my_string al terminar de ejecutarse la función. 
// Por eso el compilador falla.
// Este otro código tampoco funciona:
//
// fn returns_str() -> &str {
//     let my_string = String::from("I am a string"); // no compila ⚠️
//     "I am a str" // ⚠️
// }
//
// fn main() {                                        // no compila ⚠️
//     let my_str = returns_str();
//     println!("{}", my_str);
// }
// Aquí el problema es que "I am a str" es un &str que vive en el stack y se libera al final de la función.
// Por tanto, no se puede devolver una referencia a él.
// El compilador aquí nos indica una ayuda para solucionar el problema: "help: consider giving it a 'static lifetime".
// El mensaje missing lifetieme specifier significa que tenemos que añadir ' con un tiempo de vida.
//
// Con la modificación, lo siguiente funciona:
//
// fn returns_str() -> &'static str {
//     let my_string = String::from("I am a string");
//     "I am a str"
// }
//
// fn main() {
//     let my_str = returns_str();
//     println!("{}", my_str);
// }
// NECESARIO: leer https://www.jmgaguilera.com/rust_facil/37.html

// 📌   NUTABILIDAD INTERIOR
//
// 
// La mutabilidad interior permite modificar el interior de un elemento sin necesidad de que la variable sea mut.
// Esto se consigue con la palabra clave "RefCell" y el método "borrow_mut".
//
// Rust permite hacer esto en algunos casos de forma segura, modificando los valores internos de un struct
// -que es inmutable. 
// Cada uno de los mecanismos que se tienen a disposición, sigue unas reglas que aseguran que la modificación es segura.
// NECESARIO: leer https://www.jmgaguilera.com/rust_facil/38.html
     
// 📌   ATOMIC
//    
// Atomic es una variante de Mutex que permite la modificación de valores de forma atómica.
// Atomic es útil en la programación concurrente, donde varios procesos pueden modificar un valor al mismo tiempo.
// Atomic es más rápido que Mutex, pero solo se puede usar con tipos de datos primitivos.
// NECESARIO: leer https://www.jmgaguilera.com/rust_facil/38.html

// 📌   COW 
//
// Cow es un tipo de dato que permite tener una referencia o una copia de un valor.
// es un tipo de enumerado que puede ser "clone on write" (clonar al escribir) o 
// "borrow on write" (prestar al escribir).
// Permite devolver una referencia o una copia de un valor sin tener que clonarlo.
// NECESARIO: leer https://www.jmgaguilera.com/rust_facil/39.html

// 📌   ALIAS DE TIPOS
//
// Los alias de tipos son una forma de dar un nombre más corto a un tipo de dato.
// ejemplo de un nombre más facil de recordar para un tipo de dato.
//
// type CharacterVec = Vec<char>;
//
// fn main() {}
//
// EL siguiente ejemplo sirve para mostrar un caso con un tipo difícil de entender:
//
// fn returns<'a>(input: &'a Vec<char>) -> std::iter::Take<std::iter::Skip<std::slice::Iter<'a, char>>> {
//     input.iter().skip(4).take(5)
// }
//
// fn main() {}
//
// Con un alias queda mucho más claro:
//
// type SkipFourTakeFive<'a> = std::iter::Take<std::iter::Skip<std::slice::Iter<'a, char>>>;
//
// fn returns<'a>(input: &'a Vec<char>) -> SkipFourTakeFive {
//     input.iter().skip(4).take(5)
// }
//
// fn main() {}

// 📌   ALIAS DE TIPOS
//
// también se puede importar un tipo para hacer las definiones más simples.
// ejemplo:
// use std::iter::{Take, Skip};
// use std::slice::Iter;
//
// fn returns<'a>(input: &'a Vec<char>) -> Take<Skip<Iter<'a, char>>> {
//     input.iter().skip(4).take(5)
// }
//
// fn main() {}
// Así que se puede decidir qué es mejor en cada caso.
// 
// Los alias no crean un tipo nuevo. Solo se trata de un nuevo nombre que representa al tipo ya existente.
// type File = String;
//
// fn main() {
//     let my_file = File::from("I am file contents");
//     let my_string = String::from("I am file contents");
//     println!("{}", my_file == my_string);
// }
//
// Los alias no crean un tipo nuevo. Solo se trata de un nuevo nombre que representa al tipo ya existente. 
// Por ello, si se escribe el siguiente código type File = String;, el compilador solo ve el tipo String
// NECESARIO: leer https://www.jmgaguilera.com/rust_facil/40.html

// 📌   MACRO todo!
//
// La macro todo! es una macro que se usa para marcar tareas pendientes en el código.
// Se puede usar para marcar partes del código que faltan por implementar.
// Se puede usar para marcar partes del código que se deben revisar, que se deben eliminar, refactorizar, 
// mejorar, documentar o testear.
// 
// En ocasiones, se necesita escribir código para ayudar a imaginar el proyecto que se está desarrollando.
// ejemplo:
//
// fn delete_libro(libro: Libro) -> Result<(), String> {
//     todo!()
// }
//
// fn main() {}
// NECEARIO: leer https://www.jmgaguilera.com/rust_facil/41.html

// 📌   RC
//
// En Rust cada valor solo puede ter un dueño. Por eso, el siguiente código no funciona:
//
// fn takes_a_string(input: String) {
//     println!("It is: {}", input)
// }
//
// fn also_takes_a_string(input: String) {
//     println!("It is: {}", input)
// }
//
// fn main() {
//     let user_name = String::from("User MacUserson");
//
//     takes_a_string(user_name);
//     also_takes_a_string(user_name); // ⚠️
// }
//
// Después de que takes_a_string reciba user_name, no se puede volver a usar. 
// En este caso, se podría solventar utilizando user_name.clone(). 
// Pero en ocasiones, un valor forma parte de un struct y puede que no se pueda clonar ese struct.
// Rc sirve para permitir que un valor tenga más de un dueño de forma simultánea.
// Rc anota quienes tienen la propiedad y cuántos. Posteriormente, cuando el número de dueños 
// baja a cero, el valor asociado se liberará.
// NECESARIO: leer https://www.jmgaguilera.com/rust_facil/42.html

// 📌   MULTIPLES HILOS - MULTIPLE THREADS
//
// Para ejecutar diferentes tareas al mismo tiempo, se usan los hilos (threads). 
// Esto significa que el sistema operativo crea este hilo y lo asigna a un núcleo de proceso
// Los ordenadores modernos suelen tener más de un núcleo de proceso por lo que pueden ejecutar más de una cosa a la vez.
// Para trabajar con múltiples hilos en Rust, se puede usar la librería estándar std::thread.
// Se puede crear un nuevo hilo con la función thread::spawn y pasarle un cierre (closure) 
// con el código que se quiere ejecutar.
// Se puede esperar a que un hilo termine con el método join.
// Se puede usar la macro thread::sleep para hacer que un hilo espere un tiempo determinado.
// Se pueden crear hilos con std::thread::spawn al que se le pasa un cierre para indicarle qué 
// tiene que hacer. Los hilos son interesantes porque se ejecutan a la vez.
//
// ejemplo:
// fn main() {
//     std::thread::spawn(|| {
//         println!("I am printing something");
//     });
// }
//
// Si se ejecuta este código, en ocasiones se imprimirá algo y otras veces no. 
// Dependerá también de la velocidad del ordenador en que se ejecute. 
// Esto sucede porque main() se ejecuta en el hilo principal del programa y el cierre en un hilo secundario. 
// Cuando el hilo principal, main(), finaliza, el programa se para.
// NECESARIO: leer https://www.jmgaguilera.com/rust_facil/43.html

// 📌   CLOSURES EN FUNCIONES - Cierres en funciones 
//
// NECESARIO: https://www.jmgaguilera.com/rust_facil/44.html

// 📌   IMPL TRAIT
//
// Es similar a una función genérica, pero en lugar de devolver un tipo genérico,
// devuelve un tipo concreto que implementa un trait.
// Se puede usar impl Trait en lugar de un tipo genérico cuando se conoce el tipo de dato que se va a devolver.
// NECESARIO: leer https://www.jmgaguilera.com/rust_facil/45.html

// 📌   ARC 
// 
// 📌   CANALES
// 
// 📌   MACRO assert_eq!

// 📌   ATRIBUTOS
//
// Ya se ha visto anteriormente código como este #[derive(Debug)]. Este tipo de código es un atributo. 
// Los atributos son pequeñas piezas de código que dan información al compilador. No son fáciles de crear, 
// pero son muy fáciles de usar.
// Un atributo puede comenzar con solo #, lo que significa que solo afecta al código de la siguiente línea. 
// Sin embargo, si comienza con #! afectará a todo lo que esté en su espacio.
// NECESARIO: leer https://www.jmgaguilera.com/rust_facil/49.html

// 📌   BOX
//
// Box es un tipo de dato que permite almacenar valores en el heap.
// Permite almacenar en el heap (el montón) un valor, en lugar de almacenarlo en la pila. 
// Para crear un elemento de este tipo se usa Box::new() con el elemento como parámetro.
// NECESARIO: leer https://www.jmgaguilera.com/rust_facil/50.html
  
// 📌   BOX Y LOS RASGOS
// 
// 📌 Default y el patrón constructor (builder)
// 
// 📌 Deref y DerefMut
// 
// 📌 Crates (cajones) y módulos
//
// 📌 Pruebas (testing) - leer https://www.jmgaguilera.com/rust_facil/55.html
//   
// 📌 Crates externas - leer https://www.jmgaguilera.com/rust_facil/56.html
//
// 📌 NECESARIO: leer https://www.jmgaguilera.com/rust_facil/57.html

// 📌 MACROS - Escritura de macros
//
// Una forma de escribir una macro nueva es usar la macro macro_rules!, dándole un nombre 
// y seguida de un bloque {}. Dentro del bloque, se comporta como una especie de sentencia match
// En una macro los valores de retorno tienen que ser del mismo tipo
// En el caso de una macro, sí se puede "devolver" diferente código, 
// no es algo compilado, es código lo que se devuelve
// NECESARIO: leer https://www.jmgaguilera.com/rust_facil/58.html



// 📌  Aqui generacidad, poner algo

// 📌  Aqui CLOSURES

// 📌  Structs, traits y POO en Rust


      

    




















// 📌 ESTRUCTURAS 
// Las estructuras son un tipo de dato que permite agrupar varios campos con diferentes tipos en un solo objeto.
// Se definen con la palabra clave "struct" seguida del nombre de la estructura y los campos o atributos y sus tipos de datos.
// Se pueden crear instancias de una estructura con la palabra clave "let" seguida del nombre de la estructura y los valores.
// Se pueden acceder a los valores de una estructura con el operador "." seguido del nombre del valor.
// Se pueden modificar los valores de una estructura con la palabra clave "mut".
// Se pueden crear métodos para una estructura con la palabra clave "impl" seguida del nombre de la estructura y los métodos.

// 📌 ESTRUCTURAS e IMPLEMENTACIONES con "impl"
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




    

// ver: hasmap, hashset, List

// 📌   MÉTODO .len() Calcular la longitud en bytes
//
// El método len() en Rust es un método genérico que devuelve la longitud de una colección.
// Se puede usar en vectores, arrays, slices, strings, etc.
//
// fn main() {
//
// let vec: Vec<i32> = vec![1, 2, 3];
// let length = vec.len(); // length == 3
// println!("El tamaño del vector es: {}", length);  
//
// let mut map: HashMap<String, i32> = HashMap::new();
// map.insert("foo", 1);
// map.insert("bar", 2);
// let length = map.len(); // length == 2
// println!("El tamaño del mapa es: {}", length); 
//
// let s: String = "Hello, world!".to_string();
// let length = s.len(); // length == 13
// println!("El tamaño de la cadena de texto es: {}", length);
// }
   
    
// 📌   PRUEBAS arrays, vectores, 
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
// println!("El valor de x es: {}"n de R desestructuración de la tupla en 3 variables, pero solo nos interesa la tercera
// println!("Mi saludo: {}", verdad);
// let cojo_valor = tupla.0; // accedemos al valor de la tupla con el punto y el índice
// println!("Mi valor:// 📌 CARGO {}", cojo_valor);
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

// 📌   CARGO Y BIBLIOTECAS
//
// Las bibliotecas son un conjunto de funciones y estructuras de datos que se pueden usar en un programa.
// Rust tiene un sistema de gestión de paquetes llamado Cargo que permite añadir bibliotecas a un proyecto.
// Cargo se encarga de descargar, compilar e instalar las bibliotecas necesarias para un proyecto.
// Cargo.toml es el archivo de configuración de Cargo y se usa para añadir bibliotecas a un proyecto.
// Cargo.lock es un archivo que se crea para guardar las versiones de las bibliotecas usadas en un proyecto.
// Cargo.toml es un archivo de texto que se usa para configurar un proyecto Rust.
// El archivo Cargo.toml contiene información sobre el proyecto, las dependencias y las versiones de las bibliotecas.
// Cargo.toml se puede editar manualmente o con la herramienta cargo-edit.

// 📌   CARGO - Administrador de paquetes y compilador de Rust
//
// cargo new -> crea un directorio de proyecto
// cargo clean sirve para eliminar las librerías que se hayan descargado por ser necesarias para el proyecto.
// cargo build -> compila el proyecto para que se pueda ejecutar
// cargo run  -> compila si hay cambios en el proyecto y ejecuta el mismo
// cargo check -> testea el proyecto
// cargo build --release -> para compilarlo con optimizaciones (construido para distribuir)
// cargo build --release y cargo run --release hace lo mismo que los anteriores, pero optimizado para distribuir.
//
// Cargo.toml es el archivo de configuración de Cargo y se usa para añadir bibliotecas a un proyecto.
// Cargo.toml es un archivo de texto que se usa para configurar un proyecto Rust.
// Cargo.lock es un archivo que se crea para guardar las versiones de las bibliotecas usadas en un proyecto.
// El archivo Cargo.toml contiene información sobre el proyecto, las dependencias y las versiones de las bibliotecas.
// Cargo.toml se puede editar manualmente o con la herramienta cargo-edit.
// cargo install cargo-edit -> instala el paquete cargo-edit
// cargo check sirve para validar el código. Es como compilar, pero sin que genere el código ejecutable. 
// es una forma de validar si no se quiere ejecutar el código, ya que es más rápida que build o run.
//
// cargo edit -> nos ayuda con las dependencias inserta o importa el nombre de un crate (libreria o módulo)
// -> reescribe el archivo Cargo.toml para adicionar dependencias -> https://github.com/killercup/cargo-edit
// -> viendo la versión que necesitas en crates.io
//
// sobre el compilador: siempre tarda más la primera vez que se ejecuta cargo build o carga run 
// -después de esta primera vez, recordará las librerías usadas y compilará más rápido.
// -eso sí, después de un cargo clean, volverá a tardar mucho en compilar ya que es como si fuese la primera vez que lo hace.

// 📌   CARGO EDIT
// cargo install cargo-edit -> instala el paquete cargo-edit
// cargo add [nombre_crate] -> añade una biblioteca a un proyecto
// cargo rm [nombre_crate] -> elimina una biblioteca de un proyecto
// cargo upgrade -> actualiza todas las bibliotecas de un proyecto
// cargo upgrade [nombre_crate] -> actualiza una biblioteca de un proyecto
// cargo install [nombre_crate] -> instala una biblioteca de forma global
// cargo uninstall [nombre_crate] -> desinstala una biblioteca de forma global

// 📌   CARGO EDIT - Ejemplo
// cargo add rand -> añade la biblioteca rand a un proyecto
// cargo rm rand -> elimina la biblioteca rand de un proyecto
// cargo upgrade -> actualiza todas las bibliotecas de un proyecto
// cargo upgrade rand -> actualiza la biblioteca rand de un proyecto
// cargo install cargo-edit -> instala la herramienta cargo-edit
// cargo uninstall cargo-edit -> desinstala la herramienta cargo-edit
// cargo install rustfmt -> instala la herramienta rustfmt
// cargo uninstall rustfmt -> desinstala la herramienta rustfmt

// 📌   CARGO EDIT - Ejemplo
// cargo add rand -> añade la biblioteca rand a un proyecto
// cargo rm rand -> elimina la biblioteca rand de un proyecto
// cargo upgrade -> actualiza todas las bibliotecas de un proyecto
// cargo upgrade rand -> actualiza la biblioteca rand de un proyecto
// cargo install cargo-edit -> instala la herramienta cargo-edit
// cargo uninstall cargo-edit -> desinstala la herramienta cargo-edit
// cargo install rustfmt -> instala la herramienta rustfmt
// cargo uninstall rustfmt -> desinst// Cargo también se encarga de gestionar las dependencias entre las bibliotecas.

// 📌   APUNTES Y NOTAS VARIAS
//
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
// En Rust hay que favorecer el uso de variables locales, en lugar de globales siempre que sea posible,
// si necesitamos compartir datos entre funciones, se pueden usar argumentos y retornos de funciones
// o estructuras de datos compartidas.
// El entorno de pruebas de Rust, https://play.rust-lang.org/, es una herramienta online para probar código Rust.
//
// Rustfmt formatea el código correctamente.
// Clippy da información extra sobre cómo hacer mejor el código.
// emoticonos utilizados 🚧 y ⚠️
// Comentarios // o /* ... */
//





// 📌   BIBLIOTECA IO - ENTRADA Y SALIDA
// 
// Una biblioteca es un conjunto de funcionalidades reutilizables que se pueden utilizar en otros proyectos.
// Una biblioteca es un tipo particular de crate que se enfoca en proporcionar funcionalidades reutilizables.
// La biblioteca estándar de Rust proporciona una serie de bibliotecas que se pueden utilizar en cualquier proyecto.
// 