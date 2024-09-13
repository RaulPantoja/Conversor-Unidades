use std::io::{self, Write};

#[derive(Debug)]
enum Unidad {
    Metro,
    Kilometro,
    Centimetro,
    Milimetro,
    Pulgada,
    Pie,
}

impl Unidad {
    fn from_str(input: &str) -> Option<Unidad> {
        match input.trim().to_lowercase().as_str() {
            "metro" => Some(Unidad::Metro),
            "kilometro" | "km" => Some(Unidad::Kilometro),
            "centimetro" | "cm" => Some(Unidad::Centimetro),
            "milimetro" | "mm" => Some(Unidad::Milimetro),
            "pulgada" | "in" => Some(Unidad::Pulgada),
            "pie" | "ft" => Some(Unidad::Pie),
            _ => None,
        }
    }

    fn a_metros(&self, valor: f64) -> f64 {
        match self {
            Unidad::Metro => valor,
            Unidad::Kilometro => valor * 1000.0,
            Unidad::Centimetro => valor / 100.0,
            Unidad::Milimetro => valor / 1000.0,
            Unidad::Pulgada => valor * 0.0254,
            Unidad::Pie => valor * 0.3048,
        }
    }

    fn desde_metros(&self, metros: f64) -> f64 {
        match self {
            Unidad::Metro => metros,
            Unidad::Kilometro => metros / 1000.0,
            Unidad::Centimetro => metros * 100.0,
            Unidad::Milimetro => metros * 1000.0,
            Unidad::Pulgada => metros / 0.0254,
            Unidad::Pie => metros / 0.3048,
        }
    }
}

fn main() {
    loop {
        println!("Ingresa el valor numérico de la medida:");
        let mut input_valor = String::new();
        io::stdin().read_line(&mut input_valor).expect("Error al leer la entrada");
        let valor: f64 = match input_valor.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, ingresa un número válido.");
                continue;
            }
        };

        println!("Ingresa la unidad de origen (metro, kilometro, centimetro, milimetro, pulgada, pie):");
        let mut input_unidad_origen = String::new();
        io::stdin().read_line(&mut input_unidad_origen).expect("Error al leer la entrada");
        let unidad_origen = match Unidad::from_str(&input_unidad_origen) {
            Some(u) => u,
            None => {
                println!("Unidad de origen no válida. Inténtalo de nuevo.");
                continue;
            }
        };

        loop {
            println!("Ingresa la unidad de destino (metro, kilometro, centimetro, milimetro, pulgada, pie) o 'salir' para terminar:");
            let mut input_unidad_destino = String::new();
            io::stdin().read_line(&mut input_unidad_destino).expect("Error al leer la entrada");
            let input_unidad_destino = input_unidad_destino.trim().to_lowercase();

            if input_unidad_destino == "salir" {
                println!("Saliendo del programa.");
                return;
            }

            let unidad_destino = match Unidad::from_str(&input_unidad_destino) {
                Some(u) => u,
                None => {
                    println!("Unidad de destino no válida. Inténtalo de nuevo.");
                    continue;
                }
            };

            let valor_en_metros = unidad_origen.a_metros(valor);

            let valor_convertido = unidad_destino.desde_metros(valor_en_metros);

            println!(
                "El valor {} en {:?} es equivalente a {} en {:?}.",
                valor, unidad_origen, valor_convertido, unidad_destino
            );

            println!("¿Deseas convertir a otra unidad? (si/no):");
            let mut continuar = String::new();
            io::stdin().read_line(&mut continuar).expect("Error al leer la entrada");

            if continuar.trim().to_lowercase() != "si" {
                break;
            }
        }
    }
}
