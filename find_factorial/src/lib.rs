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
