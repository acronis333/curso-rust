// 📌  lectura por teclado y asignación a  una variable

use std::io;

fn main() {
    println!("Adivina el número!");
    println!("Por favor, introduce tu suposición.");
    let mut corazonada = String::new();
    std::io::stdin().read_line(&mut corazonada)
        .ok()
        .expect("Fallo al leer el mensaje");

    println!("Tu corazonada fue: {}", corazonada);
}