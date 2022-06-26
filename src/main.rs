use std::io;

fn main() {
    println!("Adivina un numero");
    println!("Por favor indique un numero que usted crea es el ganador.");
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("No se pudo leer el numero");

    println!{"Adivino, el numero ganador es: {}", guess}

}