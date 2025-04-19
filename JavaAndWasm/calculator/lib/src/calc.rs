#[no_mangle]
pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub fn restar(a: i32, b: i32) -> i32 {
    a - b
}

#[no_mangle]
pub fn multiplicar(a: i32, b: i32) -> i32 {
    a * b
}

pub fn dividir(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sumar_works() {
        let result = sumar(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn restar_works() {
        let result = restar(2, 2);
        assert_eq!(result, 0);
    }

    #[test]
    fn multiplicar_works() {
        let result = multiplicar(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn dividir_none() {
        let result = dividir(2, 0);
        assert_eq!(result, None);
    }

    #[test]
    fn dividir_works() {
        let result = dividir(2, 2);
        assert_eq!(result, Some(1));
    }
}