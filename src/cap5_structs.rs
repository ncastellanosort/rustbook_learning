#[derive(Debug)]
pub struct Usuario {
    pub nombre: String,
    pub activo: bool
}

// para poder imprimirlo bien
// es un atributo
#[derive(Debug)]
pub struct Rectangulo {
    pub ancho: i32,
    pub alto: i32
}

impl Rectangulo {
    // associated function, como una funcion static
    pub fn new(ancho: i32, alto: i32) -> Self {
        Rectangulo {
            ancho,
            alto
        }
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
        activo: true
    };

    u1.activo = false;

    println!("usuario: {:?}", u1);

    let mut r1 = Rectangulo::new(10, 40);
    r1.set_ancho(5);

    println!("area de {:?} es {}", r1, r1.calcular_area());

}
