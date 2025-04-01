pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f-32.0)/(9.0/5.0)
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c+32.0)/(9.0/5.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fahrenheit_to_celsius(32.0);
        assert_eq!(result, 0.0);
    }
}
