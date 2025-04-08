
pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(content: String, user: String) -> Self {
        Self { content, user }
    }

    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.to_lowercase().contains("stupid") {
            None
        } else {
            Some(&self.content)
        }
    }
}

pub fn check_ms(message: &str) -> Result<&str, &str> {
    let msg = Message::new(message.to_string(), "default_user".to_string());
    
    match msg.send_ms() {
        Some(_) => Ok(message),  // Return the original message reference
        None => Err("ERROR: illegal"),
    }
}


// pub struct Message {
//     content: String,
//     user: String,
// }

// impl Message {
//     pub fn new(content: String, user: String) -> Self{
//         Self{
//             content,
//             user,
//         }
//     }

//     pub fn send_ms(&self) -> Option<&str>{
//         match self.content.to_string().as_str() {
//             s if s.contains("stupid") || s.is_empty() => None,
//             _ => Some(&self.content),
//         }
//     }
// }

// pub fn check_ms(message: &Message) -> (bool, &str) {
//     let message_cont = message.send_ms();

//     match message_cont {
//         "None"=> (false, "ERROR: illegal"),
//         _ => (true, message.content),
//     }
//     todo!()
// }