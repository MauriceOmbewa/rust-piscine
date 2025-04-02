pub fn factorial(num: u64) -> u64 {
    let mut n = 1;
    let mut result = 1;
    loop{
        if n >= num{
            break;
        }
        n += 1;
        result = result * n;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
