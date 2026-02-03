// --------------------- GENERICS  ----------------------
// definicion de T
fn identidad<T>(valor: T) -> T {
    valor
}

fn genericos() {
    let s = identidad("Hola"); // es un &str
    let s = identidad(2); // es un i32
}

// en las structs (Caja)
struct Caja<T> {
    contenido: T,
}

fn generico_caja() {
    let caja_num = Caja { contenido: 5 };
    let caja_num = Caja {
        contenido: String::from("Texto"),
    };
}

// --------------------- TRAITS  ----------------------
/*
 * la idea es poner limite a la libertad de los <T>
 * es un conjunto de reglas que se deben cumplir a la hora de usar un <T>
 * */

// acepto el T siempre y cuando implemente el Trait Display
fn imprimir_algo<T: std::fmt::Display>(item: T) {
    println!("{}", item);
}

// CREANDO MI PROPIO TRAIT
pub trait Describible {
    fn resumen(&self) -> String;
}

pub struct Coche {
    pub modelo: String,
}
pub struct Avion {
    pub marca: String,
}

// crear el trait
impl Describible for Coche {
    fn resumen(&self) -> String {
        format!("modelo del coche: {}", self.modelo)
    }
}

impl Describible for Avion {
    fn resumen(&self) -> String {
        format!("marca del avion: {}", self.marca)
    }
}

trait Sonido {
    fn hacer_ruido(&self) -> String;
}

struct Perro {}
struct Gato {}

impl Sonido for Perro {
    fn hacer_ruido(&self) -> String {
        format!("guau")
    }
}

// hay otra forma mas sencilla (animal: impl Sonido)
fn notificar<T: Sonido>(animal: T) {
    animal.hacer_ruido();
}

/*
 * con muchos genericos

// Difícil de leer
fn procesar<T: Sonido + Display, U: Clone + Debug>(animal: T, dato: U) { ... }

// Limpio con 'where'
fn procesar<T, U>(animal: T, dato: U)
where
    T: Sonido + Display,
    U: Clone + Debug
{
    // ... cuerpo de la función ...
}
 * */

// devolver "algo que hace ruido", sin decir exactamente que es
fn crear_animal_misterioso() -> impl Sonido {
    Perro {} // el que llama solo sabe que es un Sonido, no que es un perro
}

// --------------------- LIFETIMES  ----------------------
/*
 * la idea de los lifetimes es evitar las "dangling references"
 * significa apuntar a algo que ya fue borrado en memoria
 *
 * se deben usar cuando:
 * 1. funciones que reciben o devuelven referencias
 * 2. hay mas de una referencia de entrada
 * */

fn scope_ejemplo() {
    let r;
    {
        let x = 5;
        r = &x; // r -> x
    } // x sale de scope y r no queda apuntando a nada
    println!("{}", r); // r esta apuntando a algo que ya no existe
}

// seguir explicacion
fn elegir<'a>(x: &'a str, y: &'a str) -> &'a str {
    if true { x } else { y }
}
