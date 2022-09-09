extern crate inquire;
use inquire::{Text, InquireError, Select};

fn main() {
    let nombre = Text::new("Cual es tu nombre?")
        .with_placeholder("Juan")
        .prompt();
    match nombre {
        Ok(nombre) => println!("Hola {}", nombre),
        Err(_) => println!("Un error ha ocurrido leyendo el prompt"),
    }

    let opciones: Vec<&str> = vec!["Lunes", "Martes", "Miercoles"];
    let resp: Result<&str, InquireError> = Select::new("Elige un dia de la semana", opciones).prompt();

    match resp {
        Ok(opt) => println!("Has escogido el día {}", opt),
        Err(_) => println!("Ha ocurrido un error"), 
    }

}
