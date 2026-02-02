// --------------------- VECTORES ----------------------
/*
 * es una estructura que vive en el stack
 * tiene 3 datos:
 * 1. puntero: direccion donde empieza la lista en el heap
 * 2. capacidad: cuantos elementos caben en el espacio reservado actualmente
 * 3. longitud: cuantos elementos hay ahora
 *
 * cuando se hace un v[2], rust hace:
 * direccion de inicio + ( tamano del tipo * <2> )
 *
 * */
fn vectores() {
    // creacion
    let mut v: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3, 4];

    v2.push(5);

    let uno = &v2[1]; // devuelve un Option<&T>
    let dos = v2.get(2); // devuelve un Option<&T>

    // iteracion
    for i in &mut v2 {
        // son listas de referencias
        *i += 50; // dereferenciamos el puntero para acceder al valor
    }
}

// --------------------- STRINGS ----------------------
/*
 * todo el texto es UTF-8 en rust
 *
 * hay 2 tipos que se deben conocer:
 * 1. String: vive en el heap, puede crecer y yo tengo la propiedad
 * 2. &str: es una referencia a una parte del String
 *
 * */
fn manipulando_strings() {
    let mut s = String::from("Nicolas");
    s.push_str(" felipe"); // esta ananiendo un &str
    s.push('!'); // anade caracter

    // concatenar
    let tic = String::from("tic");
    let tac = String::from("tac");

    let s = format!("{}-{}", tic, tac);
    println!("resultado = {s}");
}

// --------------------- HASHMAPS ----------------------
use std::collections::HashMap;

fn ejemplo_hashmaps() {
    let mut puntajes = HashMap::new();

    puntajes.insert(String::from("Azul"), 10);
    puntajes.insert(String::from("Amarillo"), 50);

    // acceder a un valor
    let nombre_equipo = String::from("Azul");
    let puntaje = puntajes.get(&nombre_equipo); // devuelve Option<i32>

    match puntaje {
        Some(n) => println!("puntaje = {n}"),
        None => println!("no hay puntaje"),
    }
}
