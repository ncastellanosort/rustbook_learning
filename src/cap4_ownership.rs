/*
 * stack vs heap
 *
 * stack -> datos de tamanio fijo, como i32 o bool, rapidismo
 * heap -> datos de tamanio variable o desconocido, como String, mucho mas lento y requiere un
 * puntero
 * */

// --------------------- MOVE ----------------------
pub fn concepto_move() {
    let x = 5;
    let y = x; // se hace una copia porque esta en el stack

    println!("y = {y}, x = {x}"); // funciona

    // s1 es un puntero al String "Hola"
    let s1 = String::from("Hola");
    let s2 = s1; // se hace un move, ya que s1 esta en el heap

    /*
     * rust mueve la propiedad a s2 para evitar que las 2 variables no intenten vaciar la misma
     * memoria al final
     *
     * con x no pasa nada porque son tan simples que rust prefiere copiarlos
     * */
}

// --------------------- BORROWING  ----------------------
// para no regalar la propiedad, usamos las referencias (&T)
pub fn concepto_borrowing() {
    let s1 = String::from("Hola");

    // pasamos la referencia (&s1), no el objeto entero
    let len_s1 = calcular_longitud(&s1);

    println!("s1 = {s1}, len_s1 = {len_s1}"); // sigo siendo duenio de s1
}

fn calcular_longitud(s: &String) -> usize {
    s.len()
} // aca s sale del scope, pero como es una referencia, no borra el String original

// --------------------- MUTABILITY  ----------------------
// 1 referencia mutable (escritura)
// infinitas inmutables (lectura)
pub fn mutabilidad_peligrosa() {
    let mut s = String::from("Hola");

    let r1 = &s; // lectura 1
    let r2 = &s; // lectura 2

    // let r3 = &mut s; -> error, no se puede porque ya hay de lectura

    println!("{} y {}", r1, r2); // aca se limpian ya que pasan al scope del println!

    let r3 = &mut s; // podemos crear ahora si una de escritura
    r3.push_str(", mundo");
    println!("r3 = {}", r3);
}


// --------------------- SLICES  ----------------------
/*
 * imaginemos un String con la frase "Hola mundo" donde solo quiero la primera palabra
 * en otros lenguajes creamos una nueva variable y copiamos los datos
 * en rust, usamos los *slices*
 *
 * un slice es una referencia a una parte contigua de una coleccion
 * no es duenio de los datos, solo los observa
 * */

pub fn prueba_slices() {
    let s = String::from("Hola, mundo");

    let hola = &[..3]; // indice el 0 al 3 para Hola
    let mundo = &[5..]; // mundo

    // SI TENGO UN SLICE (PRESTAMO), NO PUEDO MODIFICAR EL STRING ORIGINAL
    // s.clear(), los slices apuntarian a nada
}
