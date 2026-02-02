#[derive(Debug)]
pub struct Usuario {
    pub nombre: String,
    pub activo: bool,
}

// --------------------- TUPLE STRUCTS  ----------------------
/*
 * se usan cuando se quiere que la struct tenga un nombre (para que sea un tipo diferente a los
 * otros) pero por el momento los nombres de los campos no son necesarios
*/

pub struct Color(i32, i32, i32);

fn ejemplo_tuple_structs() {
    let negro_rgb = Color(0, 0, 0);
    println!("valor del negro en el verde es: {}", negro_rgb.0);
}

// --------------------- UNIT-LIKE STRUCTS  ----------------------
/*
 * fundamentales para el uso de los traits (CAP. 10)
 * se usan cuando se necesita implementar un comportamiento en algun tipo, pero no se necesita que
 * ese tipo guarde ningun dato
 *
 * "este objeto solo existe para poder llamar a sus funciones"
*/

struct SiempreIgual;

// es un atributo para poder imprimir bien el struct
#[derive(Debug)]
pub struct Rectangulo {
    pub ancho: i32,
    pub alto: i32,
}

impl Rectangulo {
    // associated function, como una funcion static
    pub fn new(ancho: i32, alto: i32) -> Self {
        Rectangulo { ancho, alto }
    }

    fn calcular_area(&self) -> i32 {
        self.ancho * self.alto
    }

    fn set_ancho(&mut self, ancho: i32) {
        self.ancho = ancho;
    }
}

pub fn struct_ejemplo() {
    let mut u1 = Usuario {
        nombre: String::from("Nicolas"),
        activo: true,
    };

    // setear atributos
    u1.activo = false;

    println!("usuario: {:?}", u1);

    // uso de la funcion asociada
    let mut r1 = Rectangulo::new(10, 40);
    r1.set_ancho(5);

    println!("area de {:?} es {}", r1, r1.calcular_area());
}
