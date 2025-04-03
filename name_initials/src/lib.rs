pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for name in names {
        let mut initials = String::new();
        
        // Split the name by spaces
        for part in name.split_whitespace() {
            if let Some(first_char) = part.chars().next() {
                // Add the first character followed by a full stop
                initials.push(first_char);
                initials.push('.');
            }
        }
        
        result.push(initials);
    }

    result
}