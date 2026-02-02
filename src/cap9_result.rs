// --------------------- RESULT  ----------------------
/*
 * errores irrecuperables con panic!()
 * errores donde el programa no puede continuar, como un index of out bound
 * */

fn ejemplo_panic() {
    panic!("algo salio muy mal");
}

/*
 * errores recuperables, con Result<T, E>
 * se usa este enum
 * enum Result<T, E> {
 *      Ok(T),
 *      Err(E),
 * }
 * */

use std::fs::File;
use std::io;
use std::io::ErrorKind;

pub fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        // creamos el error
        return Err(String::from("No se puede dividir por cero, genio."));
    }
    Ok(a / b)
}

pub fn abrir_archivo() {
    let f = File::open("hola.txt");

    let archivo = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hola.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problema creando el archivo: {:?}", e),
            },
            otro_error => {
                panic!("Problema abriendo el archivo: {:?}", otro_error);
            }
        },
    };
}

// --------------------- EXPECT Y UNWRAP  ----------------------
/*
 * si estoy seguro que el codigo no va a fallar, o no espero eso
 * */

use std::io::Read;
use std::io::stdin;

fn ejemplo_expect_unwrap() {
    let mut s = String::new();
    let mut s2 = String::new();

    // permite poner un mensaje en el panic
    stdin().read_line(&mut s).expect("Error reading line");

    // manda el panic! y acaba la ejecucion del programa
    stdin().read_line(&mut s2).unwrap();
}

// --------------------- ERRORES CON ?  ----------------------
fn leer_usuario() -> Result<String, io::Error> {
    let mut f = File::open("usuario.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}

use std::io::Write;

// --------------------- FUNCIONES QUE NO DEVUELVEN NADA  ----------------------
// el Result de devuelta es ()
pub fn guardar_log(mensaje: &str) -> Result<(), std::io::Error> {
    let mut archivo = File::create("log.txt")?;

    archivo.write_all(mensaje.as_bytes())?;

    Ok(()) // devolvemos un Ok(()) -> salio bien
}
// --------------------- BOX DYN  ----------------------
/*
 * decirle a rust que no sabemos que tamano tiene eso, que lo guarde en el heap
 * y lo trate como un error
 *
 * se usa Box<T> ya que el compilador necesita saber cuanto ocupa cada variable en el stack
 * ej:
 * 1. io::Error -> tiene un tamano
 * 2. ParseIntError -> tiene un tamano distinto
 *
 * si una funcion devuelve ambos el compilador se vulve loco porque no sabe cuanto espacio reservar
 *
 * la solucion de Box<T> es mandar el contenido al heap y devolver un puntero al stack
 * para asi conocer su tamano (8 bytes)
 *
 * ahora el dyn, es "dynamic", dice que se el metodo que se va a usar para manejar el error
 * se decidira en tiempo de ejecucion y no de compilacion
 *
 * basicamente le dice a rust que no importa la estructura del error si no que implemente el Trait
 * Error
 */
use std::error::Error;
use std::fs;

pub fn operacion_mixta() -> Result<(), Box<dyn Error>> {
    // 1. Esto devuelve un std::io::Error
    let texto = fs::read_to_string("id.txt")?;

    // 2. Esto devuelve un std::num::ParseIntError
    let numero: i32 = texto.trim().parse()?;

    // 3. ¡Incluso podemos lanzar nuestro propio error de String!
    if numero < 0 {
        return Err("¡El ID no puede ser negativo!".into());
    }

    println!("ID procesado: {}", numero);
    Ok(())
}
