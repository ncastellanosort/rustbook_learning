// --------------------- ENUMS ----------------------
/*
 * permiten decir que un valor es uno de un conjunto de posibles valores
 * */
#[derive(Debug)]
pub enum IpAddr {
    // se denominan variantes
    V4(u8, u8, u8, u8), // guarda 4 numeros
    V6(String),         // guarda un String
}

pub fn ejemplo_enum_datos() {
    let local = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("ipv4 = {:?}", local);
    println!("ipv6 = {:?}", loopback);
}

// --------------------- OPTION<T> ----------------------
// rust no usa null, si una funcion puede no devolver nada, devuelve un Option<T>
/*
 * enum Option<T> {
 *      Some(T),
 *      None
 * }
 * */

pub fn ejemplo_option() {
    let un_numero = Some(5);
    let un_texto: Option<&str> = Some("Hola");
    let numero_ausente: Option<i32> = None; // especie de null

    // para usar los valores, debemos "abrir la caja"
}

// --------------------- MATCH ----------------------
// es un control de flujo que obliga a manejar todos los brazos de un enum

enum Moneda {
    Peso,
    Dolar(f64),
}

fn valor_en_centavos(moneda: Moneda) -> u32 {
    match moneda {
        Moneda::Peso => 100,
        Moneda::Dolar(valor) => {
            // asigna el f64 a una variable temporable llamada "valor" para
            // usarla en este bloque
            println!("valor del dolar es {}", valor);
            (valor * 100.0) as u32
        }
    }
}

// --------------------- IF LET ----------------------
// si solo importa un brazo del enum y quiero ignorar el resto
// IF LET <PATRON> = <VARIABLE>

fn ejemplo_if_let() {
    // sufijos de tipo <numero><tipo> -> 3u8 -> numero 3 de tipo u8
    let algun_valor = Some(3u8);

    if let Some(3) = algun_valor {
        println!("es un tres!");
    }
}

// --------------------- OPTION Y NONE ----------------------
fn duplicar(numero: Option<i32>) -> i32 {
    // depronto recibe un numero
    match numero {
        Some(n) => n * 2, // n es el numero que se desempaca del Option
        None => 0,
    }
}

// EJERCICIO

enum EstadoServidor {
    Online(String),
    Offline,
    Manteminiento(u32),
}

fn procesar_estado_servidor(servidor: EstadoServidor) {
    match servidor {
        EstadoServidor::Online(url) => {
            println!("server online: {}", url);
        }
        EstadoServidor::Manteminiento(horas) => {
            println!("quedan {}h para el mantenimiento", horas);
        }
        _ => println!("opcion desconocida"),
    }
}

fn revisar_estado_servidor(estado: EstadoServidor) {
    if let EstadoServidor::Offline = estado {
        println!("server offline");
    } else {
        println!("server online");
    }
}
