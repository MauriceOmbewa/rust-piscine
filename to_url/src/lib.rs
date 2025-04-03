pub fn to_url(s: &str) -> String {
    let mut temp = String::new();
    let letters: Vec<char> = s.chars().collect();
    let mut count = 0;

    loop {
        if count == letters.len(){
            break;
        }
        if letters[count] == ' '{
            temp.push_str("%20")
        } else {
            temp.push(letters[count])
        }
        count += 1;
    }
    temp    
}