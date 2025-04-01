pub fn sum(left: u8, right: u8) -> u8 {
    left + right
}

pub fn diff(left: i16, right: i16) -> i16 {
    left - right
}

pub fn pro(left: u32, right: u32) -> u32 {
    left * right
}

pub fn quo(left: f32, right: f32) -> f32 {
    left / right
}

pub fn rem(left: f32, right: f32) -> f32 {
    left % right
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
