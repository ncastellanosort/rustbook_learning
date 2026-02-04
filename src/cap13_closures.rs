/*
 * pasar de un paradigma imperativo a funcional
 *
 * el objetivo es expresar transformaciones de datos sin
 * perder ownership, performance ni seguridad
 * */

// ------------------- CLOSURES --------------------
/*
 * es una funcion anonima
 * captura valores del entorno con tipos INFERIDOS
 * */

fn closures() {
    let x = 5;
    // la closure usa x pero no toma su ownership, es un prestamo inmutable
    let add_x = |y| y + x;
}

// los closure != funciones
// funcion normal:
// no puede capturar entorno y solo usa lo que recibe por parametros
fn add(x: usize, y: usize) -> usize {
    y + x
}

/*
 * closures:

// let add_x = |y| y + x;

* el compilador genera un tipo anonimo por debajo <T>
* el tipo T implementa alguno de estos Traits automaticamente
* Fn, FnMut, FnOnce
*/

// ------------------- 3 TRAITS DE LOS CLOSURES ---------------------
/*
 * trait Fn
 * captura por referencia inmutable
 * se puede llamar multiples veces
 * no modifica ni consume nada
*/

fn fn_trait() {
    let x = 5;
    let c = |y: i32| y + x; // &x
}

/*
 * trait FnMut
 * captura por referencia mutable
 * puede modificar el entorno
 * necesita mut
*/

fn fn_mut_trait() {
    let mut x = 5;
    let mut c = |y: i32| {
        x += y;
        x
    };
}

/*
 * trait FnOnce
 * consume valores capturados
 * solo se puede llamar una vez
*/

fn fn_once_trait() {
    let x = String::from("hola");
    let c = || drop(x);
}

// el compilador decide cual trait implemetara el closure
//
// move -> fuerza a mover x dentro de la closure
// incluso solo si se lee
// comun en threads y async(tokio)
//
// usar move no implica FnOnce automaticamente, depende de lo que se haga dentro

fn move_closure() {
    let x = vec![1, 2, 3];
    let c = move || println!("{:?}", x);
}

// move literalmente mueve las variables del entorno dentro de la closure, no prestadas
// sin move (FnOnce) recibe una referencia (&x)

// ---------------- ITERATORS -----------------
// sirven para separa lo que quiero hacer de como se hace

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// cada llamada a next() devuelve un Some(Item) o None cuando termina

// los iterators son lazy, ejemplo:
fn lazy_map() {
    let v = vec![1, 3, 4, 5];
    v.iter().map(|x| x + 1); // no hace nada, solo describe una transformacion de los datos
}
/*
 * solo se va a ejecutar cuando use un consumer:

 * llame a collect();
 * use for
 * llamo a next()
 * uso sum, count, etc
*/

// TRES TIPOS BASICOS DE ITERACION

/*
 * 1. iter()
 *
 * no consume v, presta los elementos
 *
 * for x in v.iter() {
 *      x: &i32
 * }
 * */

/*
 * 2. iter_mut()
 *
 * no consume v, presta los elementos mutablemente y modifica en sitio
 *
 * for x in v.iter_mut() {
 *      x: &mut i32
 *      *x += 1; // podemos modificar
 *      x
 * }
 * */

/*
 * 2. into_iter()
 *
 * consume el vector, mueve los valores
 *
 * for x in v.into_iter() {
 *      x: i32
 * }
 * */

// el for es azucar sintactico
/*
 * for x in v {
 *      println!("{}", x);
 * }
 *
 * es lo mismo que
 *
 * let iter = v.into_iter();
 *
 * while let Some(x) = iter.next() {
 *      println!("{}", x);
 * }
 *
 * */

// -------------- COLLECT ---------------
// es un consumer que forza la ejecucion del iterator y construye la coleccion
// siempre toca colocar el tipo ya que rust no difiere tipos de estructuras de datos
//
// collect() lo que hace es llamar el .next() hasta que llegue a None

// map y filter
fn map_filter() {
    let v = vec![1, 2, 3, 4];

    let v2: Vec<_> = v.iter().map(|x| x + 1).filter(|x| *x > 2).collect();

    println!("{v2:?}");
}

