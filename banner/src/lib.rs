use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Self {
            short_hand: format!("-{}", &name[0..1]),
            long_hand: format!("--{}", name),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert((flag.short_hand, flag.long_hand), func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        // Check if we have at least 2 arguments
        if argv.len() < 2 {
            return Err("Not enough arguments".to_string());
        }
        
        // Find the callback function for the input flag
        for ((short_hand, long_hand), callback) in &self.flags {
            if input == short_hand || input == long_hand {
                // Execute the callback with the first two arguments
                return callback(argv[0], argv[1]).map_err(|e| e.to_string());
            }
        }
        
        Err("Flag not found".to_string())
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_val = a.parse::<f64>()?;
    let b_val = b.parse::<f64>()?;
    Ok((a_val / b_val).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_val = a.parse::<f64>()?;
    let b_val = b.parse::<f64>()?;
    Ok((a_val % b_val).to_string())
}