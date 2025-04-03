pub fn arrange_phrase(phrase: &str) -> String {
    // Split the phrase into words and map each to a tuple containing:
    // (the digit as a char, the word slice)
    let mut words: Vec<(char, &str)> = phrase
        .split_whitespace()
        .filter_map(|word| {
            // Find the first digit in the word
            word.chars().find(|c| c.is_ascii_digit()).map(|d| (d, word))
        })
        .collect();
    
    // Sort the words based on the digit.
    words.sort_by_key(|&(d, _)| d);

    // Build the final arranged phrase.
    let mut arranged = String::with_capacity(phrase.len());
    for (i, &(_, word)) in words.iter().enumerate() {
        // Remove all digits from the word by filtering out numeric characters.
        let filtered: String = word.chars().filter(|c| !c.is_ascii_digit()).collect();
        if i > 0 {
            arranged.push(' ');
        }
        arranged.push_str(&filtered);
    }
    arranged
}