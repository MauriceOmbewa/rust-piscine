pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => match server {
            Ok(value) => value.to_string(),
            Err(_) => panic!(),
        }
        Security::Message => match server {
            Ok(value) => value.to_string(),
            Err(_) => panic!("ERROR: program stops"),
        }
        Security::Warning => match server {
            Ok(value) => value.to_string(),
            Err(_) => panic!("WARNING: check the server"),
        }
        Security::NotFound => match server {
            Ok(value) => value.to_string(),
            Err(_) => panic!("Not found: [MESSAGE]"),
        }
        Security::UnexpectedUrl => match server {
            Ok(err) => panic!("{}", err.to_string()),
            Err(err) => err.to_string(),
        }
        
    }
}