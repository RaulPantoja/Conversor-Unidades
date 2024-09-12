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
}

#[derive(Debug)]
struct Medida {
    valor: f64,
    unidad: Unidad,
}

impl Medida {
    fn convertir_a_metros(&self) -> f64 {
        match self.unidad {
            Unidad::Metro => self.valor,
            Unidad::Kilometro => self.valor * 1000.0,
            Unidad::Centimetro => self.valor / 100.0,
            Unidad::Milimetro => self.valor / 1000.0,
            Unidad::Pulgada => self.valor * 0.0254,
            Unidad::Pie => self.valor * 0.3048,
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

        println!("Ingresa la unidad (metro, kilometro, centimetro, milimetro, pulgada, pie):");
        let mut input_unidad = String::new();
        io::stdin().read_line(&mut input_unidad).expect("Error al leer la entrada");
        let unidad = match Unidad::from_str(&input_unidad) {
            Some(u) => u,
            None => {
                println!("Unidad no válida. Inténtalo de nuevo.");
                continue;
            }
        };

        let medida = Medida { valor, unidad };
        let valor_en_metros = medida.convertir_a_metros();

        println!(
            "El valor {} en {:?} es equivalente a {} metros.",
            medida.valor, medida.unidad, valor_en_metros
        );

        println!("¿Quieres hacer otra conversión? (si/no):");
        let mut continuar = String::new();
        io::stdin().read_line(&mut continuar).expect("Error al leer la entrada");

        if continuar.trim().to_lowercase() != "si" {
            println!("Saliendo del programa.");
            break;
        }
    }
}
