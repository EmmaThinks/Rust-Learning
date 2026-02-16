// crates
use std::io;
use std::io::Write;

//primera dependencia externa que uso en rust

use sysinfo::System;

fn main() {
    //variables que no se reinician en el loop
    let mut system_online: bool = false;

    // segun el doc, usamos esta variable con "new_all" para asegurarnos que todas las listas y procesos del procesador esten llenos (?)
    // quizas lo este traduciendo muy directamente, unicamente debe ser para que el programa reconozca todo

    let mut system_gen = System::new_all();

    loop {
        // variables

        //variable mutable que se adapta al input del usuario
        let mut prompt = String::new();

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

        // shadowing al parecer, me deja usar el mismo nombre de la variable que vamos a cambiar como si fuera una nueva
        // no termino de entender muy bien como funciona pero si hace el codigo mas rapido no importa
        let prompt = prompt.trim();

        //al parecer String y &str no son lo mismo, uno crece, el otro no, por eso usamos &prompt[..] para pasar de String > &str y que puedan hacer match los types
        match prompt {
            //comparamos
            "start" => {
                system_online = true;
                println!("Encendiendo Daemon...");
            }
            "stop" => {
                system_online = false;
                println!("Apagando Daemon...");
            }
            "status" => {
                show_status(system_online, &mut system_gen);
            }
            "exit" => {
                println!("Nos vemos!");
                break;
            }
            _ => println!("Comando invalido"),
        }
    }
}

fn show_status(val: bool, sys: &mut System) {
    if val {
        //recargamos el sistema
        sys.refresh_all();

        println!(
            "Sistema:        {}",
            System::host_name().unwrap_or("Desconocido".to_string())
        );
        println!("CPUs:           {}", sys.cpus().len());
        println!(
            "RAM TOTAL:      {} GB",
            sys.total_memory() / 1024 / 1024 / 1024
        );
        println!(
            "RAM USADA:      {} GB",
            sys.used_memory() / 1024 / 1024 / 1024
        );
        println!()
    } else {
        println!("El daemon esta apagado.")
    }
}
