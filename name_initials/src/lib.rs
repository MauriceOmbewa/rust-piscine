pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for name in names {
        let mut initials = String::new();
        let mut count = 1;
        
        // Split the name by spaces
        for part in name.split_whitespace() {
            if let Some(first_char) = part.chars().next() {
                if count == 2 {
                    count = 1;
                    // Add the first character followed by a full stop
                    initials.push(first_char);
                    initials.push('.');
                    continue;
                }
                // Add the first character followed by a full stop
                initials.push(first_char);
                initials.push('.');
                initials.push(' ');
                count += 1;
            }
        }
        
        result.push(initials);
    }

    result
}