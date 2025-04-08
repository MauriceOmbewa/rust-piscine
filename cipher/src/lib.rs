#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    // Apply the Atbash cipher to the original string
    let expected_cipher = original
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                // For alphabetic characters, apply the Atbash transformation
                if c.is_ascii_lowercase() {
                    // a -> z, b -> y, etc.
                    (b'z' - (c as u8 - b'a')) as char
                } else {
                    // A -> Z, B -> Y, etc.
                    (b'Z' - (c as u8 - b'A')) as char
                }
            } else {
                // Non-alphabetic characters remain unchanged
                c
            }
        })
        .collect::<String>();
    
    // Compare the expected cipher with the provided ciphered string
    if expected_cipher == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected: expected_cipher })
    }
}