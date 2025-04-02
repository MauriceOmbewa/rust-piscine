pub fn fibonacci(n: u32) -> u32 {
    let mut numbers: Vec<u32> = Vec::new();
    numbers.push(0);
    numbers.push(1);

    let mut count = 0;

    loop {
        if count == n{
            break
        }
        count += 1;
        numbers.push((numbers[numbers.len()-1]+numbers[numbers.len()-2]).try_into().unwrap());
    }
    numbers[n as usize]
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
