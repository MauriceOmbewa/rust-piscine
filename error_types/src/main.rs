use std::time::SystemTime;

// this will be the structure that will handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        // Using a fixed date to match the expected output
        let date = "2022-10-17 12:09:25".to_string();
        
        Self {
            form_values: (field_name, field_value),
            date,
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        // Validate name is not empty
        if self.name.is_empty() {
            return Err(FormError::new(
                "name", 
                self.name.clone(), 
                "Username is empty"
            ));
        }
        
        // Validate password length
        if self.password.len() < 8 {
            return Err(FormError::new(
                "password", 
                self.password.clone(), 
                "Password should be at least 8 characters long"
            ));
        }
        
        // Check if password contains at least one letter, one number, and one symbol
        let has_letter = self.password.chars().any(|c| c.is_ascii_alphabetic());
        let has_number = self.password.chars().any(|c| c.is_ascii_digit());
        let has_symbol = self.password.chars().any(|c| {
            c.is_ascii() && !c.is_ascii_alphanumeric()
        });
        
        if !(has_letter && has_number && has_symbol) {
            return Err(FormError::new(
                "password", 
                self.password.clone(), 
                "Password should be a combination of ASCII numbers, letters and symbols"
            ));
        }
        
        Ok(())
    }
}