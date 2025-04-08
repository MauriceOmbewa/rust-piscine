pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_string(), // Panics with default message if Err
        Security::Message => server.expect("ERROR: program stops").to_string(), // Panics with custom message
        Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),
        Security::NotFound => match server {
            Ok(value) => value.to_string(),
            Err(err) => panic!("Not found: {}", err),
        },
        Security::UnexpectedUrl => match server {
            Ok(value) => panic!("{}", value),
            Err(err) => err.to_string(),
        },
    }
}
