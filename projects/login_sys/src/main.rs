use std::io;

fn main() {
    //  variables
    let mut input = String::new();

    //  codigo
    println!("Usuario, por favor, identifiquese:");

    io::stdin()
        .read_line(&mut input)
        .expect("Error al guardar la variable");

    if input.trim().to_lowercase() == "nav" {
        println!("bienvenida {}!", input.trim());
    } else {
        println!("bienvenido usuario {}!", input);
    }
} //es mas facil cuando intentas en lugar de ver Rust by example
