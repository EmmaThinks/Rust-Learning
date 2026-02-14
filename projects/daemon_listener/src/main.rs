use std::io;
use std::io::Write;
fn main() {
    loop {
        // variables

        //variable mutable que se adapta al input del usuario
        let mut prompt = String::new();
        //donde vamos a guardar la variable del usuario sin espacios
        let trimmed_prompt: &str;

        // el prompt que el usuario ve, no podemos usar el macro de print! solamente por que no se imprime, a menos que lo forcemos
        print!("OVR > ");
        //forzamos el print! e ignoramos el resultado
        let _ = io::stdout().flush();

        //leemos el input del usuario
        io::stdin()
            //leemos la linea
            .read_line(&mut prompt)
            //si por alguna extraña razon falla el prompt
            .expect("no se pudo leer el prompt");

        //como trim me devuelve &str, guardamos el result en una variable de tipo &str
        trimmed_prompt = prompt.trim();

        //al parecer String y &str no son lo mismo, uno crece, el otro no, por eso usamos &prompt[..] para pasar de String > &str y que puedan hacer match los types
        match trimmed_prompt {
            //comparamos
            "start" => {
                println!("Inicializando sistema...");
                println!("Sistemas en Linea.");
            }
            "status" => {
                println!("CPU: OK");
                println!("RAM: OK");
                println!("DISK: OK");
            }
            "exit" => {
                println!("Nos vemos!");
                break;
            }
            _ => println!("Comando invalido"),
        }
    }
}
