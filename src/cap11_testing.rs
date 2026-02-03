// -----------------TESTING-----------
// si un test no pasa hay panic!
// !cargo test ejecuta todos los tests en paralelo

#[cfg(test)]
mod tests {
    #[test] // convierte una funcion normal en un test
    fn it_works() {
        assert_eq!(1 + 2, 3) // resultado, mas lo esperado
    }

    #[test]
    fn works_too() {
        assert!(true) // espera un bool
    }

    fn division(a: usize, b: usize) -> Result<usize, String> {
        if b == 0 {
            Err("division by 0".to_string())
        } else {
            Ok(a / b)
        }
    }

    #[test]
    fn division_fails() {
        let err = division(4, 0).unwrap_err(); // devuelve el String error
        assert_eq!(err, "division by 0".to_string());
    }

    #[test]
    fn division_ok() -> Result<(), String> {
        let result = division(10, 2)?;
        assert_eq!(result, 5);
        Ok(())
    }
}
